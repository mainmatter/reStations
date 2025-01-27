use std::task::Poll;

use axum::body::Bytes;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum_streams::StreamBodyAs;
use hyper::body::Frame;
// TODO figure out why only using types:: doesn't work here
use crate::state::SharedAppState;
use crate::types::station_record::StationRecord;
use axum::extract::State;
use futures_util::TryStreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use futures_util::TryStream;

use super::super::db;

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
        .body(JsonStreamBody::new(stations_stream))
        // .body(StreamBodyAs::json_array_with_errors(stations_stream))
        // .body(JsonStreamBody {})
        .unwrap()
}
struct JsonStreamBody<S> {
    stream: S,
    problems: Vec<Problem>,
    current_state: JsonStreamBodyState,
}

enum JsonStreamBodyState {
    Initial,
    Stations,
    Problems,
    ProblemsDone,
    End,
}

impl<S> JsonStreamBody<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            problems: vec![],
            current_state: JsonStreamBodyState::Initial,
        }
    }
}

impl<I: serde::Serialize, S: futures_util::TryStream<Item = I> + std::marker::Unpin> axum::body::HttpBody for JsonStreamBody<S> {
    type Data = axum::body::Bytes;
    type Error = std::convert::Infallible;
    
    fn poll_frame(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>> {
        let this = self.get_mut();
        match this.current_state {
            // initial state: send `{ "places": [`
            JsonStreamBodyState::Initial => {
                this.current_state = JsonStreamBodyState::Stations;
                Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(br#"{ "places": ["#)))))
            },
            // streaming: send results and commas
            JsonStreamBodyState::Stations => {
                let stream_pinned = std::pin::Pin::new(&mut this.stream);
            // ) -> Poll<Option<Result<Self::Ok, Self::Error>>>;
                match stream_pinned.try_poll_next(cx) {
                    Poll::Ready(None) => {
                        // streaming_done: send `], "problems": [`
                        this.current_state = JsonStreamBodyState::Problems;
                        Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(br#"], "problems": ["#)))))
                    },
                    Poll::Ready(msg) => {
                        todo!()
                    },
                    Poll::Pending => Poll::Pending,
                }
            }
            // sending_problems: send problems and commas
            JsonStreamBodyState::Problems => todo!(),
            // problems_done: send `] }`
            JsonStreamBodyState::ProblemsDone => todo!(),
            // end: return None
            JsonStreamBodyState::End => todo!(),
        }
    }
}
/*
    { "places": [112312, 123123 ,123123 ,123 , ❌, 123123 312,31231, 420,123 , ❌, 12313,], "problems": [{idx: 0, e: "wrong stuff"}] };

*/

#[derive(serde::Serialize, serde::Deserialize)]
struct Problem {
    code: String,
    title: String,
}