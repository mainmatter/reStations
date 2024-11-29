use axum::response::Json;
use serde::{Deserialize, Serialize};
use rusqlite::{Connection};
// TODO figure out why only using types:: doesn't work here
use crate::types::station_record::StationRecord;

// TODO what's the right notation here for a collection of station records?
/// Responds with a [`[StationRecord]`], encoded as JSON.
#[axum::debug_handler]
pub async fn list() -> Json<Vec<StationRecord>> {
    // TODO inject connection in through app state
    let conn = Connection::open_in_memory()?;
    let stmt = conn.prepare("SELECT * from stations")?;
    let rows = stmt.query([])?;

    let mut stations = Vec::new();
    for row in rows {
        // TODO map row to StationRecord
        // from_row() doesn't exist yet
        let station_result = StationRecord::from_row(row?);
        stations.push(station_result?);
    }

    Json(stations)
}
