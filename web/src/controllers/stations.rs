use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum_streams::StreamBodyAs;
// TODO figure out why only using types:: doesn't work here
use crate::state::SharedAppState;
use crate::types::station_record::StationRecord;
use axum::extract::State;
use futures_util::TryStreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use super::super::db;

pub type StationsListResponse = Vec<StationRecord>;

// TODO what's the right notation here for a collection of station records?
/// Responds with a [`[StationRecord]`], encoded as JSON.
#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> impl IntoResponse {
    let (sender, receiver) = mpsc::channel::<Result<StationRecord, db::DbError>>(100);

    tokio::task::spawn_blocking(move || {
        let conn = app_state.pool.get().unwrap();
        db::find_all_stations(&conn, sender);
    });

    let stations_stream = ReceiverStream::new(receiver).map_err(crate::error::Error::from);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(StreamBodyAs::json_array_with_errors(stations_stream))
        .unwrap()
}
