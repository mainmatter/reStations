use axum::response::Json;
// TODO figure out why only using types:: doesn't work here
use crate::state::SharedAppState;
use crate::types::station_record::StationRecord;
use axum::extract::State;

use super::super::db;

// TODO what's the right notation here for a collection of station records?
/// Responds with a [`[StationRecord]`], encoded as JSON.
#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> Json<Vec<StationRecord>> {
    let conn = app_state.conn.clone();
    let locked_conn = conn.lock().unwrap();

    let stations = db::find_all_stations(&locked_conn);

    Json(stations)
}
