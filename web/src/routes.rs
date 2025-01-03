use crate::controllers::greeting;
use crate::controllers::stations;
use crate::state::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

/// Initializes the application's routes.
///
/// This function maps paths (e.g. "/greet") and HTTP methods (e.g. "GET") to functions in [`crate::controllers`] as well as includes middlewares defined in [`crate::middlewares`] into the routing layer (see [`axum::Router`]).
pub fn init_routes(app_state: AppState) -> Router {
    let shared_app_state = Arc::new(app_state);

    Router::new()
        .route("/greet", get(greeting::hello))
        .route("/stations", get(stations::list))
        .with_state(shared_app_state)
}
