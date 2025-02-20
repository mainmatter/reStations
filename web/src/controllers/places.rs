use crate::db;
use crate::state::SharedAppState;
use crate::types::osdm::*;
use crate::types::station_record::StationRecord;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};

pub enum PlacesShowResponse {
    Ok(OsdmPlaceResponse),
    NotFound(OsdmProblem),
}

impl IntoResponse for PlacesShowResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Ok(body) => (StatusCode::OK, Json(body)).into_response(),
            Self::NotFound(body) => (StatusCode::NOT_FOUND, Json(body)).into_response(),
        }
    }
}

#[axum::debug_handler]
pub async fn show(
    State(app_state): State<SharedAppState>,
    Path(place_id): Path<String>, // TODO: fix uic type at sync stage, like latitude and longitude
) -> PlacesShowResponse {
    let conn = app_state.pool.get().unwrap();

    match db::find_station(&conn, &place_id) {
        Ok(station) => show_found_station(station),
        Err(db::DbError::RecordNotFound(_msg)) => show_not_found(place_id),
        _ => todo!("Unexpected error at places::show"),
    }
}

fn show_found_station(station: StationRecord) -> PlacesShowResponse {
    let latitude = station
        .latitude
        .parse::<f64>()
        .expect("Failed to parse latitude");
    let longitude = station
        .longitude
        .parse::<f64>()
        .expect("Failed to parse longitude");

    let geo_position = OsdmGeoPosition {
        latitude,
        longitude,
    };

    let place = OsdmPlace {
        // TODO: fix uic type at sync stage, like latitude and longitude
        id: station.uic.parse::<i64>().expect("Failed to parse uic"),
        object_type: "StopPlace".into(),
        alternative_ids: vec![],
        geo_position,
        _links: vec![],
    };
    let response = OsdmPlaceResponse {
        places: vec![place],
    };

    PlacesShowResponse::Ok(response)
}

fn show_not_found(place_id: String) -> PlacesShowResponse {
    let api_problem = OsdmProblem {
        code: String::from("not-found"),
        title: format!("Could not find place with id #{}", place_id),
    };
    PlacesShowResponse::NotFound(api_problem)
}
