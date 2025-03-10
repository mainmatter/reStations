use crate::controllers::osdm::*;
use crate::db::error::DbError;
use crate::db::finders::*;
use crate::db::station_record::StationRecord;
use crate::state::SharedAppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use std::convert::From;

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
    let maybe_place_name = match place_req.place_input {
        Some(value) => value.name,
        None => None,
    };
    let query = match maybe_place_name {
        Some(name) => search_all_stations(&app_state.pool, &name).await,
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
