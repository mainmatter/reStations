use restations_config::Config;

use std::sync::{Arc, Mutex};

use super::db;

/// The application's state that is available in [`crate::controllers`] and [`crate::middlewares`].
pub struct AppState {
    pub conn: Arc<Mutex<db::Connection>>,
}

/// The application's state as it is shared across the application, e.g. in controllers and middlewares.
///
/// This is the [`AppState`] struct wrappend in an [`std::sync::Arc`].
pub type SharedAppState = Arc<AppState>;

/// Initializes the application state.
///
/// This function creates an [`AppState`] based on the current [`restations_config::Config`].

pub async fn init_app_state(_config: Config) -> Result<AppState, crate::error::Error> {
    let conn = db::create_connection()?;

    // TODO
    // how to use SharedAppState instead?
    Ok(AppState {
        conn: Arc::new(Mutex::new(conn)),
    })
}
