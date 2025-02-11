use super::super::db;
use crate::state::SharedAppState;
use axum::extract::{Path, State};
use axum::response::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApiPlaceResponse {
    pub places: Vec<ApiPlace>,
}
#[derive(Deserialize, Serialize)]
pub struct ApiPlace {
    pub id: i64,
    pub object_type: String,
    alternative_ids: Vec<String>,
    pub geo_position: ApiGeoPosition,
    _links: Vec<ApiLink>,
}
#[derive(Deserialize, Serialize)]
pub struct ApiGeoPosition {
    pub latitude: f64,
    pub longitude: f64,
}
#[derive(Deserialize, Serialize)]
pub struct ApiLink {
    rel: String,
    href: String,
    _type: String,
    value: String,
}

#[axum::debug_handler]
pub async fn show(State(app_state): State<SharedAppState>, Path(id): Path<u64>) -> Json<ApiPlaceResponse> {
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

    let geo_position = ApiGeoPosition {
        latitude: latitude,
        longitude: longitude,
    };

    let place = ApiPlace {
        id: station.id,
        object_type: "StopPlace".into(),
        alternative_ids: vec![],
        geo_position,
        _links: vec![],
    };
    let response = ApiPlaceResponse {
        places: vec![place],
    };

    Json(response)
}
