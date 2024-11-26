use std::path::PathBuf;

use clap::Args;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest;
use rusqlite::{Connection, Result};
use stations_core::data::StationRecord;
use tokio::sync::mpsc;
use tokio_util::{io::StreamReader, sync::PollSender};

use crate::{error::Error, CommonArgs};

#[derive(Debug, Clone, Args)]
pub struct SyncAction {
    #[arg(short, long = "db", default_value = "./stations.db")]
    db_path: PathBuf,
}

impl SyncAction {
    pub async fn exec(self, _common: CommonArgs) -> Result<(), Error> {
        // A channel for sending the records to the database worker thread
        let (tx, mut rx) = mpsc::channel::<StationRecord>(32);

        // Spawn worker thread for the blocking database operations
        let db_task = tokio::task::spawn_blocking(move || {
            let conn = Connection::open(self.db_path)?;

            // Refresh the table
            conn.execute_batch(include_str!("../../reset_db.sql"))?;

            // 4. Insert records in database
            while let Some(record) = rx.blocking_recv() {
                // TODO insert other fields
                conn.execute(
                    "INSERT into stations (name) VALUES (?1)",
                    [record.name.as_str()],
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
}
