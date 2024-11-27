use clap::Args;

use crate::{error::Error, CommonArgs};

#[derive(Debug, Clone, Args)]
pub struct SyncAction {
    // todo fields
}

impl SyncAction {
    pub async fn exec(self, _common: CommonArgs) -> Result<(), Error> {
        // 1. streamingly fetch csv from https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv,
        // 2. pipe the data into https://github.com/gwierzchowski/csv-async, and deserialize to [`stations_core::data::StationRecord`]
        // 3. pipe deserialized data into database
        
        
        // fetch (or stream) csv from https://raw.githubusercontent.com/trainline-eu/stations/refs/heads/master/stations.csv
        // load csv into StationRecord, either directly or via GeoJSON for which there's a tool in the repo

        todo!("this")
    }
}
