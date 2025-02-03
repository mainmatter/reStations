use super::super::db;
use crate::state::SharedAppState;
use crate::types::station_record::StationRecord;
use axum::extract::{Path, State};
use axum::response::Json;

#[axum::debug_handler]
pub async fn show(
    State(app_state): State<SharedAppState>,
    Path(id): Path<u64>,
) -> Json<StationRecord> {
    let conn = app_state.pool.get().unwrap();

    let station = db::find_station(&conn, id).unwrap();

    Json(station)
}
