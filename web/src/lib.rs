//! The restations_web crate contains the application's web interface which mainly are controllers implementing HTTP endpoints. It also includes the application tests that are black-box tests, interfacing with the application like any other HTTP client.
use anyhow::Context;
use axum::serve;
use restations_config::{get_env, load_config, Config};
use tokio::net::TcpListener;
use tracing::info;
use tracing_panic::panic_hook;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// The application's controllers that implement request handlers.
pub mod controllers;
/// DB access code e.g. for loading records
pub mod db;
/// Contains the application's error type and related conversion implementation.
pub mod error;
/// Middlewares that incoming requests are passed through before being passed to [`controllers`].
pub mod middlewares;
/// Contains OSDM-specific datatypes for responses and requests.
pub mod osdm;
/// Contains the application's route definitions.
pub mod routes;
/// Contains the application state definition and functionality to initialize it.
pub mod state;

/// Runs the application.
///
/// This function does all the work to initiatilize and run the application:
///
/// 1. Determine the environment the application is running in (see [`restations_config::get_env`])
/// 2. Load the configuration (see [`restations_config::load_config`])
/// 3. Initialize the application state (see [`state::init_app_state`])
/// 4. Initialize the application's router (see [`routes::init_routes`])
/// 5. Boot the application and start listening for requests on the configured interface and port
pub async fn run() -> anyhow::Result<()> {
    let env = get_env().context("Cannot get environment!")?;
    let config: Config = load_config(&env).context("Cannot load config!")?;

    let app_state = state::init_app_state(config.clone()).await;

    let app = routes::init_routes(app_state);

    let addr = config.server.addr();
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on {}", &addr);
    serve(listener, app.into_make_service()).await?;

    Ok(())
}

/// Initializes tracing.
///
/// This function
///
/// * registers a [`tracing_subscriber::fmt::Subscriber`]
/// * registers a [`tracing_panic::panic_hook`]
///
/// The function respects the `RUST_LOG` if set or defaults to filtering spans and events with level [`tracing_subscriber::filter::LevelFilter::INFO`] and higher.
pub fn init_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::fmt::time::UtcTime;

    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_timer(UtcTime::rfc_3339());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter)
        .init();

    std::panic::set_hook(Box::new(panic_hook));
}

/// Helpers that simplify writing application tests.
#[cfg(feature = "test-helpers")]
pub mod test_helpers;
