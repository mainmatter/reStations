use super::super::db;
use crate::state::SharedAppState;
use axum::extract::{Path, State};
use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Place {
    id: i64,
    object_type: String,
    alternative_ids: Vec<String>,
    geo_position: GeoPosition,
    _links: Vec<ApiLink>,
}
#[derive(Serialize)]
pub struct GeoPosition {
    latitude: f64,
    longitude: f64,
}
#[derive(Serialize)]
pub struct ApiLink {
    rel: String,
    href: String,
    _type: String,
    value: String,
}

#[axum::debug_handler]
pub async fn show(State(app_state): State<SharedAppState>, Path(id): Path<u64>) -> Json<Place> {
    let conn = app_state.pool.get().unwrap();

    let station = db::find_station(&conn, id).unwrap();
    let latitude = station
        .latitude
        .parse::<f64>()
        .expect("Failed to parse latitude");
    let longitude = station
        .longitude
        .parse::<f64>()
        .expect("Failed to parse longitude");

    let geo_position = GeoPosition {
        latitude: latitude,
        longitude: longitude,
    };

    let place = Place {
        id: station.id,
        object_type: "StopPlace".into(),
        alternative_ids: vec![],
        geo_position,
        _links: vec![],
    };

    Json(place)
}
