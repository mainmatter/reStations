use crate::db::error::DbError;
use crate::db::finders::*;
use crate::db::station_record::StationRecord;
use crate::state::SharedAppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OsdmGeoPosition {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Serialize)]
pub struct OsdmLink {
    rel: String,
    href: String,
    _type: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OsdmPlace {
    pub id: String,
    pub object_type: String,
    pub name: String,
    pub alternative_ids: Vec<String>,
    pub geo_position: Option<OsdmGeoPosition>,
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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OsdmPlaceRequest {
    pub place_input: Option<OsdmInitialPlaceInput>,
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
    let places = find_all_stations(&app_state.pool)
        .await
        .expect("Unexpected error at places::list");

    PlacesResponse::Ok(places.into())
}

pub async fn search(
    State(app_state): State<SharedAppState>,
    Json(place_req): Json<OsdmPlaceRequest>,
) -> PlacesResponse {
    let maybe_place_input = place_req.place_input;

    let query = match maybe_place_input {
        Some(input) => {
            match (input.name, input.geo_position) {
                // Search by name and position
                (Some(name), Some(position)) => {
                    search_stations_by_name_and_position(
                        &app_state.pool,
                        &name,
                        position.latitude,
                        position.longitude,
                    )
                    .await
                }
                // Search by name only
                (Some(name), None) => search_all_stations(&app_state.pool, &name).await,
                // Search by position only
                (None, Some(position)) => {
                    search_stations_by_position(
                        &app_state.pool,
                        position.latitude,
                        position.longitude,
                    )
                    .await
                }
                // No search criteria, return all
                (None, None) => find_all_stations(&app_state.pool).await,
            }
        }
        None => find_all_stations(&app_state.pool).await,
    };

    let places = query.expect("Unexpected error at places::search");

    PlacesResponse::Ok(places.into())
}

#[axum::debug_handler]
pub async fn show(
    State(app_state): State<SharedAppState>,
    Path(place_id): Path<String>,
) -> PlacesResponse {
    match find_station(&app_state.pool, &place_id).await {
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
