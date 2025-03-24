use crate::osdm::{
    OsdmGeoPosition, OsdmPlace, OsdmPlaceRequest, OsdmPlaceResponse, PlacesResponse,
};
use crate::{error::Error, state::SharedAppState};
use axum::extract::{Path, State};
use axum::response::Json;
use restations_db::entities::stations;
use std::convert::From;

impl From<stations::Station> for OsdmPlace {
    fn from(station: stations::Station) -> Self {
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

impl From<Vec<stations::Station>> for OsdmPlaceResponse {
    fn from(stations: Vec<stations::Station>) -> Self {
        OsdmPlaceResponse {
            places: stations.into_iter().map(|station| station.into()).collect(),
        }
    }
}

// Endpoint handlers
//
#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> Result<PlacesResponse, Error> {
    let places = stations::load_all(&app_state.db_pool).await?;

    Ok(PlacesResponse::Ok(places.into()))
}

pub async fn search(
    State(app_state): State<SharedAppState>,
    Json(place_req): Json<OsdmPlaceRequest>,
) -> Result<PlacesResponse, Error> {
    let maybe_place_input = place_req.place_input;

    // TODO improve input handling
    let stations = match maybe_place_input {
        Some(input) => {
            match (input.name, input.geo_position) {
                // Search by name and position
                (Some(name), Some(position)) => {
                    stations::search_by_name_and_position(
                        &name,
                        position.latitude,
                        position.longitude,
                        &app_state.db_pool,
                    )
                    .await?
                }
                // Search by name only
                (Some(name), None) => stations::search_by_name(&name, &app_state.db_pool).await?,
                // Search by position only
                (None, Some(position)) => {
                    // TODO handle missing coordinates
                    stations::search_by_position(
                        position.latitude,
                        position.longitude,
                        &app_state.db_pool,
                    )
                    .await?
                }
                // No search criteria, return all
                (None, None) => stations::load_all(&app_state.db_pool).await?,
            }
        }
        None => stations::load_all(&app_state.db_pool).await?,
    };

    Ok(PlacesResponse::Ok(stations.into()))
}

#[axum::debug_handler]
pub async fn show(
    State(app_state): State<SharedAppState>,
    Path(place_id): Path<String>,
) -> Result<PlacesResponse, Error> {
    let station = stations::load(place_id.parse().unwrap(), &app_state.db_pool).await?;

    Ok(PlacesResponse::Ok(vec![station].into()))
}
