use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum_streams::StreamBodyAs;
// TODO figure out why only using types:: doesn't work here
use crate::state::SharedAppState;
use crate::types::station_record::StationRecord;
use futures_util::TryStreamExt;
use axum::extract::State;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use super::super::db;

type StationsStream = UnboundedReceiverStream<Result<StationRecord, db::DbError>>;

// TODO what's the right notation here for a collection of station records?
/// Responds with a [`[StationRecord]`], encoded as JSON.
#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> impl IntoResponse {
    let (tx, rx) = mpsc::unbounded_channel::<Result<StationRecord, db::DbError>>();

    tokio::task::spawn_blocking(move || {
        let conn = app_state.conn.clone();
        let locked_conn = conn.lock().unwrap();
        db::find_all_stations(&locked_conn, tx);
    });
    let stations_stream = UnboundedReceiverStream::new(rx).map_err(crate::error::Error::from);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(StreamBodyAs::json_array_with_errors(stations_stream))
        .unwrap()
}
