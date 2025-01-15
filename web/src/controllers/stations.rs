use axum::response::{IntoResponse, Response};
use axum::http::{StatusCode, header};
use axum_streams::StreamBodyAs;
// TODO figure out why only using types:: doesn't work here
use crate::state::SharedAppState;
use crate::types::station_record::StationRecord;
use axum::extract::State;
use tokio::sync::mpsc;

use super::super::db;

// TODO what's the right notation here for a collection of station records?
/// Responds with a [`[StationRecord]`], encoded as JSON.
#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> impl IntoResponse {
    let conn = app_state.conn.clone();
    let locked_conn = conn.lock().unwrap();
    let (tx, rx) = mpsc::unbounded_channel::<Result<StationRecord, db::DbError>>();

    db::find_all_stations(&locked_conn, tx);
    let stations_stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(StreamBodyAs::json_array(stations_stream))
        .unwrap()
}
