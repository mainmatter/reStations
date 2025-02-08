//! The restations_web crate contains the application's web interface which mainly are controllers implementing HTTP endpoints. It also includes the application tests that are black-box tests, interfacing with the application like any other HTTP client.

use std::sync::Arc;

use anyhow::Context;
use axum::serve;
use error::Error;
use restations_config::{get_env, load_config, Config};

pub mod db;

use tokio::{net::TcpListener, sync::mpsc};
use tracing::{info, instrument};
use tracing_panic::panic_hook;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use types::station_record::StationRecord;

use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest;

use tokio_util::{io::StreamReader, sync::PollSender};

/// The application's controllers that implement request handlers.
pub mod controllers;
/// Contains the application's error type and related conversion implementation.
pub mod error;
/// Middlewares that incoming requests are passed through before being passed to [`controllers`].
pub mod middlewares;
/// Contains the application's route definitions.
pub mod routes;
/// Contains the application state definition and functionality to initialize it.
pub mod state;
/// Bunch of types we need
pub mod types;

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

    let app_state = if let Ok(state) = state::init_app_state(config.clone()).await {
        state
    } else {
        return Err(anyhow::anyhow!("Failed to initialize app state"));
    };

    sync(app_state.pool.clone()).await?;

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

/// TODO move this function somewhere else
/// TODO don't take ownership
#[instrument(skip_all)]
async fn sync(pool: Arc<db::Pool>) -> Result<(), Error> {
    // A channel for sending the records to the database worker thread
    let (tx, mut rx) = mpsc::channel::<StationRecord>(32);

    // Spawn worker thread for the blocking database operations
    let db_task = tokio::task::spawn_blocking(move || {
        let conn = pool.get().unwrap();
        // Refresh the table
        db::create_tables(&conn)?;

        // 4. Insert records in database
        while let Some(record) = rx.blocking_recv() {
            db::insert_station(&conn, &record)?;
        }
        Ok(())
    });

    // 1. streamingly fetch csv from https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv
    // Get the response bytes as stream (https://docs.rs/futures/latest/futures/prelude/trait.Stream.html)
    let stream = reqwest::get(
        "https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv",
    )
    .await?
    .bytes_stream()
    // map items from a Result<Bytes, reqwest::Error> to Result<Bytes, tokio::io::Error>,
    // in order for the stream to be wrapped in a StreamReader
    .map(|r| r.map_err(tokio::io::Error::other));
    // Wrap the stream in a StreamReader, which implements AsyncRead, the trait csv-async is built around
    let reader = StreamReader::new(stream);

    // 2. Pipe the data into https://github.com/gwierzchowski/csv-async, and deserialize to [`stations_core::data::StationRecord`]
    let mut deserializer = csv_async::AsyncReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .create_deserializer(reader);

    let records = deserializer
        .deserialize::<StationRecord>()
        .map_err(Error::from);

    // 3. Send the deserialized data to the database task
    // Wrap the Sender in a PollSender, which implements Sink, allowing us to
    // forward the records to it.
    let tx =
        PollSender::new(tx).sink_map_err(|_| panic!("Error sending StationRecord to channel."));
    records.forward(tx).await?;

    // Wait for the database task to finish
    db_task.await.unwrap()
}
