use axum::body::Bytes;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use hyper::body::Frame;
use serde::Serialize;
use serde_json::to_string;
// TODO figure out why only using types:: doesn't work here
use crate::state::SharedAppState;
use crate::types::station_record::StationRecord;
use axum::extract::State;
use futures_util::{TryStream, TryStreamExt};
use std::task::Poll;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

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
    has_sent_first: bool,
}

enum JsonStreamBodyState {
    Initial,
    Stations,
    Problems,
    End,
}

impl<S> JsonStreamBody<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            problems: vec![],
            has_sent_first: false,
            current_state: JsonStreamBodyState::Initial,
        }
    }
}

impl<I, S, E> axum::body::HttpBody for JsonStreamBody<S>
where
    I: Serialize,
    E: std::error::Error,
    S: TryStream<Ok = I, Error = E> + std::marker::Unpin,
{
    type Data = axum::body::Bytes;
    type Error = std::convert::Infallible;

    fn poll_frame(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>> {
        let mut this = self.as_mut();
        match this.current_state {
            // initial state: send `{ "places": [`
            JsonStreamBodyState::Initial => {
                this.current_state = JsonStreamBodyState::Stations;
                Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(
                    br#"{ "places": ["#,
                )))))
            }
            // streaming: send results and commas
            JsonStreamBodyState::Stations => {
                let stream_pinned = std::pin::Pin::new(&mut this.stream);
                // ) -> Poll<Option<Result<Self::Ok, Self::Error>>>;
                match stream_pinned.try_poll_next(cx) {
                    Poll::Ready(None) => {
                        // streaming_done: send `], "problems": [`
                        this.current_state = JsonStreamBodyState::Problems;
                        Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(br#"]"#)))))
                    }
                    Poll::Ready(Some(Ok(station))) => match to_string(&station) {
                        Ok(station_json) => {
                            let delimiter = if self.has_sent_first { ", " } else { "" };
                            self.has_sent_first = true;
                            let json = Bytes::from(format!(r#"{delimiter}{station_json}"#));
                            Poll::Ready(Some(Ok(Frame::data(json))))
                        }
                        Err(e) => {
                            let err = e.to_string();
                            this.problems.push(Problem {
                                code: "station_error".to_string(),
                                title: err,
                            });
                            Poll::Pending
                        }
                    },
                    Poll::Ready(Some(Err(station_err))) => {
                        let err = station_err.to_string();
                        this.problems.push(Problem {
                            code: "station_error".to_string(),
                            title: err,
                        });
                        Poll::Pending
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            // sending_problems: send problems and commas
            JsonStreamBodyState::Problems => {
                this.current_state = JsonStreamBodyState::End;
                let json = Bytes::from(format!(
                    r#", "problems": {} }}"#,
                    to_string(&self.problems).unwrap()
                ));
                Poll::Ready(Some(Ok(Frame::data(json))))
            }
            // end: return None
            JsonStreamBodyState::End => Poll::Ready(None),
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
