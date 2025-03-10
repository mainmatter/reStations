use serde::{Deserialize, Serialize};

//
// Data
//

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
pub struct OsdmInitialPlaceInput {
    pub name: Option<String>,
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
