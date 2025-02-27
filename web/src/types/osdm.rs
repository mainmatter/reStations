use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OsdmGeoPosition {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Deserialize, Serialize)]
pub struct OsdmLink {
    rel: String,
    href: String,
    _type: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
pub struct OsdmPlace {
    pub id: i64,
    pub object_type: String,
    pub alternative_ids: Vec<String>,
    pub geo_position: OsdmGeoPosition,
    pub _links: Vec<OsdmLink>,
}

#[derive(Deserialize, Serialize)]
pub struct OsdmPlaceResponse {
    pub places: Vec<OsdmPlace>,
}

#[derive(Deserialize, Serialize)]
pub struct OsdmProblem {
    pub code: String,
    pub title: String,
}
