use clap::Args;
use reqwest;
use tokio::io::AsyncReadExt;
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;
use stations_core::data::StationRecord;
use rusqlite::{params, Connection, Result};

use crate::{error::Error, CommonArgs};

#[derive(Debug, Clone, Args)]
pub struct SyncAction {
    // todo fields
}

impl SyncAction {
    pub async fn exec(self, _common: CommonArgs) -> Result<(), Error> {
        // 1. streamingly fetch csv from https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv,

        // Get the response bytes as stream (https://docs.rs/futures/latest/futures/prelude/trait.Stream.html)
        let stream = reqwest::get("https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv")
            .await?
            .bytes_stream()
            // map items from a Result<Bytes, reqwest::Error> to Result<Bytes, tokio::io::Error>,
            // in order for the stream to be wrapped in a StreamReader
            .map(|r| r.map_err(tokio::io::Error::other));
        // Wrap the stream in a StreamReader, which implements AsyncRead, the trait csv-async is built around
        let reader = StreamReader::new(stream);

        // 2. pipe the data into https://github.com/gwierzchowski/csv-async, and deserialize to [`stations_core::data::StationRecord`]

        let mut deserializer = csv_async::AsyncReaderBuilder::new()
            .delimiter(b';')
            .create_deserializer(reader);

        let mut records = deserializer.deserialize::<StationRecord>();

        // 3. pipe deserialized data into database

        // println!("body = {body:?}");

        // fetch (or stream) csv from https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv
        // load csv into StationRecord, either directly or via GeoJSON for which there's a tool in the repo

        let conn = Connection::open_in_memory()?;

        conn.execute(
            "CREATE TABLE stations (
                id   TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                slug TEXT,
                uic TEXT,
                uic8_sncf TEXT,
                latitude TEXT,
                longitude TEXT,
                parent_station_id TEXT,
                country TEXT,
                time_zone TEXT,
                is_city TEXT,
                is_main_station TEXT,
                is_airport TEXT,
                is_suggestable TEXT,
                country_hint TEXT,
                main_station_hint TEXT,
                sncf_id TEXT,
                sncf_tvs_id TEXT,
                sncf_is_enabled TEXT,
                entur_id TEXT,
                entur_is_enabled TEXT,
                db_id TEXT,
                db_is_enabled TEXT,
                busbud_id TEXT,
                busbud_is_enabled TEXT,
                distribusion_id TEXT,
                distribusion_is_enabled TEXT,
                flixbus_id TEXT,
                flixbus_is_enabled TEXT,
                cff_id TEXT,
                cff_is_enabled TEXT,
                leoexpress_id TEXT,
                leoexpress_is_enabled TEXT,
                obb_id TEXT,
                obb_is_enabled TEXT,
                ouigo_id TEXT,
                ouigo_is_enabled TEXT,
                trenitalia_id TEXT,
                trenitalia_is_enabled TEXT,
                trenitalia_rtvt_id TEXT,
                trenord_id TEXT,
                ntv_rtiv_id TEXT,
                ntv_id TEXT,
                ntv_is_enabled TEXT,
                hkx_id TEXT,
                hkx_is_enabled TEXT,
                renfe_id TEXT,
                renfe_is_enabled TEXT,
                atoc_id TEXT,
                atoc_is_enabled TEXT,
                benerail_id TEXT,
                benerail_is_enabled TEXT,
                westbahn_id TEXT,
                westbahn_is_enabled TEXT,
                sncf_self_service_machine TEXT,
                same_as TEXT,
                info_de TEXT,
                info_en TEXT,
                info_es TEXT,
                info_fr TEXT,
                info_it TEXT,
                info_nb TEXT,
                info_nl TEXT,
                info_cs TEXT,
                info_da TEXT,
                info_hu TEXT,
                info_ja TEXT,
                info_ko TEXT,
                info_pl TEXT,
                info_pt TEXT,
                info_ru TEXT,
                info_sv TEXT,
                info_tr TEXT,
                info_zh TEXT,
                normalised_code TEXT,
                iata_airport_code TEXT
            )",
            ()
        )?;

        while let Some(record) = records.next().await {
            let name = &record.unwrap().name;
            conn.execute("
            INSERT into stations (name) VALUES (?1)
            ",
            params![name]
            ,
            )?;
        
        }

        let mut statement = conn.prepare("SELECT id, name FROM stations")?;

        let mut rows = statement.query([])?;

        while let Some(row) = rows.next()? {
            println!("Row {:?}", row);
        }

        Ok(())
    }
}
