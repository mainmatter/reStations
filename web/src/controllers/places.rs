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
    let conn = app_state.pool.get().unwrap();

    let places = db::find_all_stations(&conn)
        .expect("Unexpected error at places::list")
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
    let conn = app_state.pool.get().unwrap();

    match db::find_station(&conn, &place_id) {
        Ok(station) => render_show_found_station(station),
        Err(db::DbError::RecordNotFound(_msg)) => render_show_not_found(place_id),
        _ => todo!("Unexpected error at places::show"),
    }
}

fn render_show_found_station(station: StationRecord) -> PlacesResponse {
    let response = OsdmPlaceResponse {
        places: vec![station_to_osdm_place(station)],
    };

    PlacesResponse::Ok(response)
}

fn render_show_not_found(place_id: String) -> PlacesResponse {
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
        id: station.uic,
        object_type: "StopPlace".into(),
        alternative_ids: vec![],
        geo_position,
        _links: vec![],
    }
}
