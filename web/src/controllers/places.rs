use crate::db;
use crate::state::SharedAppState;
use crate::types::osdm::*;
use crate::types::station_record::StationRecord;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};

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

#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> PlacesResponse {
    let places = db::find_all_stations(&app_state.pool)
        .await
        .expect("Unexpected error at places::list")
        .into_iter()
        .map(station_to_osdm_place)
        .collect();

    PlacesResponse::Ok(OsdmPlaceResponse { places })
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
        Some(name) => db::search_all_stations(&app_state.pool, &name).await,
        None => db::find_all_stations(&app_state.pool).await,
    };

    let places = query
        .expect("Unexpected error at places::search")
        .into_iter()
        .map(station_to_osdm_place)
        .collect();

    PlacesResponse::Ok(OsdmPlaceResponse { places })
}

#[axum::debug_handler]
pub async fn show(
    State(app_state): State<SharedAppState>,
    Path(place_id): Path<String>,
) -> PlacesResponse {
    match db::find_station(&app_state.pool, &place_id).await {
        Ok(station) => render_place_response(station),
        Err(db::DbError::RecordNotFound(_msg)) => render_not_found(place_id),
        _ => todo!("Unexpected error at places::show"),
    }
}

fn render_place_response(station: StationRecord) -> PlacesResponse {
    let response = OsdmPlaceResponse {
        places: vec![station_to_osdm_place(station)],
    };

    PlacesResponse::Ok(response)
}

fn render_not_found(place_id: String) -> PlacesResponse {
    let api_problem = OsdmProblem {
        code: String::from("not-found"),
        title: format!("Could not find place with id #{}", place_id),
    };
    PlacesResponse::NotFound(api_problem)
}

fn station_to_osdm_place(station: StationRecord) -> OsdmPlace {
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
