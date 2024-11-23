use clap::Args;
use reqwest;
use tokio::io::AsyncReadExt;
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;
use stations_core::data::StationRecord;

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
        let mut reader = StreamReader::new(stream);

        // For now we just read the entire body in a string and print it
        // let mut body = String::new();
        // reader.read_to_string(&mut body).await?;

        // 2. pipe the data into https://github.com/gwierzchowski/csv-async, and deserialize to [`stations_core::data::StationRecord`]

        // let mut deserializer = AsyncReaderBuilder::new()
        //     .delimiter(b';')
        //     .create_deserializer(reader);

        let mut deserializer = csv_async::AsyncDeserializer::from_reader(reader);

        let mut records = deserializer.deserialize::<StationRecord>();

        while let Some(record) = records.next().await {
            println!("{:?}", record?);
        }



        // 3. pipe deserialized data into database

        // println!("body = {body:?}");

        // fetch (or stream) csv from https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv
        // load csv into StationRecord, either directly or via GeoJSON for which there's a tool in the repo

        Ok(())
    }
}
