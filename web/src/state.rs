use super::db;
use restations_config::Config;
use std::sync::Arc;

/// The application's state that is available in [`crate::controllers`] and [`crate::middlewares`].
pub struct AppState {
    pub pool: db::DbPool,
}

/// The application's state as it is shared across the application, e.g. in controllers and middlewares.
///
/// This is the [`AppState`] struct wrappend in an [`std::sync::Arc`].
pub type SharedAppState = Arc<AppState>;

/// Initializes the application state.
///
/// This function creates an [`AppState`] based on the current [`restations_config::Config`].
pub async fn init_app_state(_config: Config) -> AppState {
    let pool = db::create_pool("stations.sqlite.db").await;

    AppState { pool }
}
