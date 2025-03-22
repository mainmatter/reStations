use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde::{Deserialize, Serialize};

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
