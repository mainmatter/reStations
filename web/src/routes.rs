use crate::controllers::places;
use crate::state::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

/// Initializes the application's routes.
///
/// This function maps paths (e.g. "/stations") and HTTP methods (e.g. "GET") to functions in [`crate::controllers`] as well as includes middlewares defined in [`crate::middlewares`] into the routing layer (see [`axum::Router`]).
pub fn init_routes(app_state: AppState) -> Router {
    let shared_app_state = Arc::new(app_state);

    Router::new()
        .route("/places", get(places::list))
        .route("/places/{id}", get(places::show))
        .with_state(shared_app_state)
}
