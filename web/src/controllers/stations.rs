use axum::response::Json;
use serde::{Deserialize, Serialize};
use rusqlite::{Connection};
// TODO figure out why only using types:: doesn't work here
use crate::types::station_record::StationRecord;
use crate::state::SharedAppState;
use std::sync::Arc;
use axum::extract::State;

// TODO what's the right notation here for a collection of station records?
/// Responds with a [`[StationRecord]`], encoded as JSON.
#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> Json<Vec<StationRecord>> {
    let conn = app_state.conn.clone();
    let locked_conn = conn.lock().unwrap();

    let mut stmt = locked_conn.prepare("SELECT * from stations").unwrap();
    let rows = stmt.query([]).unwrap();

    let mut stations = Vec::new();
    for row in rows {
        // TODO map row to StationRecord
        // from_row() doesn't exist yet
        // let station_result = StationRecord::from_row(row?);
        let station_result = StationRecord {
            id: "1".to_string(),
            ..Default::default()
        };
        stations.push(station_result);
    }

    Json(stations)
}
