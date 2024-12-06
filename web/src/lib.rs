//! The restations_web crate contains the application's web interface which mainly are controllers implementing HTTP endpoints. It also includes the application tests that are black-box tests, interfacing with the application like any other HTTP client.

use std::sync::{Arc, Mutex};

use anyhow::Context;
use axum::serve;
use error::Error;
use restations_config::{get_env, load_config, Config};

use tokio::{net::TcpListener, sync::mpsc};
use tracing::info;
use tracing_panic::panic_hook;
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use types::station_record::StationRecord;

use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest;
use rusqlite::{Connection, Result};


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

    println!("Running sync");
    sync(app_state.conn.clone()).await?;

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
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    std::panic::set_hook(Box::new(panic_hook));
}

/// Helpers that simplify writing application tests.
#[cfg(feature = "test-helpers")]
pub mod test_helpers;

/// TODO move this function somewhere else
/// TODO don't take ownership
async fn sync(conn: Arc<Mutex<Connection>>) -> Result<(), Error> { // -> rusqlite_connection
    // A channel for sending the records to the database worker thread
        let (tx, mut rx) = mpsc::channel::<StationRecord>(32);

        // Spawn worker thread for the blocking database operations
        let db_task = tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            // Refresh the table
            conn.execute_batch(include_str!("../../db.sql"))?;

            // 4. Insert records in database
            while let Some(record) = rx.blocking_recv() {
                conn.execute(
                    "INSERT into stations (name,slug,uic,uic8_sncf,latitude,longitude,parent_station_id,country,time_zone,is_city,is_main_station,is_airport,is_suggestable,country_hint,main_station_hint,sncf_id,sncf_tvs_id,sncf_is_enabled,entur_id,entur_is_enabled,db_id,db_is_enabled,busbud_id,busbud_is_enabled,distribusion_id,distribusion_is_enabled,flixbus_id,flixbus_is_enabled,cff_id,cff_is_enabled,leoexpress_id,leoexpress_is_enabled,obb_id,obb_is_enabled,ouigo_id,ouigo_is_enabled,trenitalia_id,trenitalia_is_enabled,trenitalia_rtvt_id,trenord_id,ntv_rtiv_id,ntv_id,ntv_is_enabled,hkx_id,hkx_is_enabled,renfe_id,renfe_is_enabled,atoc_id,atoc_is_enabled,benerail_id,benerail_is_enabled,westbahn_id,westbahn_is_enabled,sncf_self_service_machine,same_as,info_de,info_en,info_es,info_fr,info_it,info_nb,info_nl,info_cs,info_da,info_hu,info_ja,info_ko,info_pl,info_pt,info_ru,info_sv,info_tr,info_zh,normalised_code,iata_airport_code) \
                    VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25,?26,?27,?28,?29,?30,?31,?32,?33,?34,?35,?36,?37,?38,?39,?40,?41,?42,?43,?44,?45,?46,?47,?48,?49,?50,?51,?52,?53,?54,?55,?56,?57,?58,?59,?60,?61,?62,?63,?64,?65,?66,?67,?68,?69,?70,?71,?72,?73,?74,?75)",
                    rusqlite::params![record.name.as_str(),record.slug.as_str(),record.uic.as_str(),record.uic8_sncf.as_str(),record.latitude.as_str(),record.longitude.as_str(),record.parent_station_id.as_str(),record.country.as_str(),record.time_zone.as_str(),record.is_city.as_str(),record.is_main_station.as_str(),record.is_airport.as_str(),record.is_suggestable.as_str(),record.country_hint.as_str(),record.main_station_hint.as_str(),record.sncf_id.as_str(),record.sncf_tvs_id.as_str(),record.sncf_is_enabled.as_str(),record.entur_id.as_str(),record.entur_is_enabled.as_str(),record.db_id.as_str(),record.db_is_enabled.as_str(),record.busbud_id.as_str(),record.busbud_is_enabled.as_str(),record.distribusion_id.as_str(),record.distribusion_is_enabled.as_str(),record.flixbus_id.as_str(),record.flixbus_is_enabled.as_str(),record.cff_id.as_str(),record.cff_is_enabled.as_str(),record.leoexpress_id.as_str(),record.leoexpress_is_enabled.as_str(),record.obb_id.as_str(),record.obb_is_enabled.as_str(),record.ouigo_id.as_str(),record.ouigo_is_enabled.as_str(),record.trenitalia_id.as_str(),record.trenitalia_is_enabled.as_str(),record.trenitalia_rtvt_id.as_str(),record.trenord_id.as_str(),record.ntv_rtiv_id.as_str(),record.ntv_id.as_str(),record.ntv_is_enabled.as_str(),record.hkx_id.as_str(),record.hkx_is_enabled.as_str(),record.renfe_id.as_str(),record.renfe_is_enabled.as_str(),record.atoc_id.as_str(),record.atoc_is_enabled.as_str(),record.benerail_id.as_str(),record.benerail_is_enabled.as_str(),record.westbahn_id.as_str(),record.westbahn_is_enabled.as_str(),record.sncf_self_service_machine.as_str(),record.same_as.as_str(),record.info_de.as_str(),record.info_en.as_str(),record.info_es.as_str(),record.info_fr.as_str(),record.info_it.as_str(),record.info_nb.as_str(),record.info_nl.as_str(),record.info_cs.as_str(),record.info_da.as_str(),record.info_hu.as_str(),record.info_ja.as_str(),record.info_ko.as_str(),record.info_pl.as_str(),record.info_pt.as_str(),record.info_ru.as_str(),record.info_sv.as_str(),record.info_tr.as_str(),record.info_zh.as_str(),record.normalised_code.as_str(),record.iata_airport_code.as_str()],
                )?;
            }
            Ok(())
        });

        // 1. streamingly fetch csv from https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv
        // Get the response bytes as stream (https://docs.rs/futures/latest/futures/prelude/trait.Stream.html)
        let stream = reqwest::get("https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv")
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
