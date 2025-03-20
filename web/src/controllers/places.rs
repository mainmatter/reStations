use crate::db::error::DbError;
use crate::db::finders::Search;
use crate::db::station_record::StationRecord;
use crate::state::SharedAppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde::{Deserialize, Serialize};
use std::convert::From;

// TODO perhaps make this configurable through an environment variable?
pub const DEFAULT_NUMBER_OF_RESULTS: i32 = 20;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OsdmGeoPosition {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct OsdmLink {
    rel: String,
    href: String,
    _type: String,
    value: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OsdmPlace {
    pub id: String,
    pub object_type: String,
    pub name: String,
    pub alternative_ids: Vec<String>,
    pub geo_position: Option<OsdmGeoPosition>,
    pub country_code: Option<String>,
    pub _links: Vec<OsdmLink>,
}

//
// Requests
//

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OsdmInitialPlaceInput {
    pub name: Option<String>,
    pub geo_position: Option<OsdmGeoPosition>,
}

// Define the restrictions structure according to the spec
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OsdmPlaceRestrictions {
    pub number_of_results: Option<i32>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OsdmPlaceRequest {
    pub place_input: Option<OsdmInitialPlaceInput>,
    pub restrictions: Option<OsdmPlaceRestrictions>,
}

//
// Responses
//

#[derive(Deserialize, Serialize)]
pub struct OsdmPlaceResponse {
    pub places: Vec<OsdmPlace>,
}

#[derive(Deserialize, Serialize)]
pub struct OsdmProblem {
    pub code: String,
    pub title: String,
}

pub enum PlacesResponse {
    Ok(OsdmPlaceResponse),
    NotFound(OsdmProblem),
}

impl IntoResponse for PlacesResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Ok(body) => (StatusCode::OK, Json(body)).into_response(),
            Self::NotFound(body) => (StatusCode::NOT_FOUND, Json(body)).into_response(),
        }
    }
}

impl From<StationRecord> for OsdmPlace {
    fn from(station: StationRecord) -> Self {
        let geo_position = match (station.latitude, station.longitude) {
            (Some(latitude), Some(longitude)) => Some(OsdmGeoPosition {
                latitude,
                longitude,
            }),
            _ => None,
        };

        OsdmPlace {
            id: format!("urn:uic:stn:{}", station.uic),
            object_type: "StopPlace".into(),
            name: station.name,
            alternative_ids: vec![],
            geo_position,
            country_code: station.country,
            _links: vec![],
        }
    }
}

impl From<Vec<StationRecord>> for OsdmPlaceResponse {
    fn from(stations: Vec<StationRecord>) -> Self {
        OsdmPlaceResponse {
            places: stations.into_iter().map(|station| station.into()).collect(),
        }
    }
}

// Endpoint handlers
//
#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> PlacesResponse {
    // This is temporary. The changes to the db crate on #77 will replace this
    // with an entity search that will fetch all records
    // Search::all is also used as fallback in search(), when no input is provided
    let limit = 100000;
    let places = Search::all(&app_state.pool, limit)
        .await
        .expect("Unexpected error at places::list");

    PlacesResponse::Ok(places.into())
}

pub async fn search(
    State(app_state): State<SharedAppState>,
    Json(place_req): Json<OsdmPlaceRequest>,
) -> PlacesResponse {
    let maybe_place_input = place_req.place_input;
    let maybe_restrictions = place_req.restrictions;

    let limit = maybe_restrictions
        .and_then(|restrictions| restrictions.number_of_results)
        .unwrap_or(DEFAULT_NUMBER_OF_RESULTS);

    // TODO improve input handling
    let query = match maybe_place_input {
        Some(input) => {
            match (input.name, input.geo_position) {
                // Search by name and position
                (Some(name), Some(position)) => {
                    Search::by_name_and_position(
                        &app_state.pool,
                        &name,
                        position.latitude,
                        position.longitude,
                        limit,
                    )
                    .await
                }
                // Search by name only
                (Some(name), None) => Search::by_name(&app_state.pool, &name, limit).await,
                // Search by position only
                (None, Some(position)) => {
                    // TODO handle missing coordinates
                    Search::by_position(
                        &app_state.pool,
                        position.latitude,
                        position.longitude,
                        limit,
                    )
                    .await
                }
                // No search criteria, return all
                (None, None) => Search::all(&app_state.pool, limit).await,
            }
        }
        None => Search::all(&app_state.pool, limit).await,
    };

    let places = query.expect("Unexpected error at places::search");

    PlacesResponse::Ok(places.into())
}

#[axum::debug_handler]
pub async fn show(
    State(app_state): State<SharedAppState>,
    Path(place_id): Path<String>,
) -> PlacesResponse {
    match Search::by_place_id(&app_state.pool, &place_id).await {
        Ok(station) => render_place_response(station),
        Err(DbError::RecordNotFound(_msg)) => render_not_found(place_id),
        _ => todo!("Unexpected error at places::show"),
    }
}

// Response rendering helpers
//
fn render_place_response(station: StationRecord) -> PlacesResponse {
    PlacesResponse::Ok(vec![station].into())
}

fn render_not_found(place_id: String) -> PlacesResponse {
    let api_problem = OsdmProblem {
        code: String::from("not-found"),
        title: format!("Could not find place with id #{}", place_id),
    };
    PlacesResponse::NotFound(api_problem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_record_to_osdm_place() {
        let record = StationRecord {
            id: 8260,
            uic: String::from("7051430"),
            name: String::from("London Charing Cross"),
            latitude: Some(51.508362),
            longitude: Some(-0.123835),
            country: Some(String::from("GB")),
            ..StationRecord::default()
        };

        let place = OsdmPlace::from(record);

        assert_eq!(
            place,
            OsdmPlace {
                id: String::from("urn:uic:stn:7051430"),
                object_type: String::from("StopPlace"),
                name: String::from("London Charing Cross"),
                geo_position: Some(OsdmGeoPosition {
                    latitude: 51.508362,
                    longitude: -0.123835,
                }),
                country_code: Some(String::from("GB")),
                alternative_ids: vec![],
                _links: Vec::<OsdmLink>::new(),
            }
        );
    }
}
