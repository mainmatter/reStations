use crate::state::SharedAppState;
use crate::types::station_record::StationRecord;
use axum::extract::State;
use axum::body::Body;
use axum::response::Json;

use serde_rusqlite::from_row;

use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use tokio_util::sync::PollSender;

use futures_util::{SinkExt, StreamExt, TryStreamExt};


// Questions:

// Do we want to stream the response, or just the records out of the database?

// Should we switch to sqlx in order to stream the records?

#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> () {
    let (sender, mut receiver) = mpsc::channel::<StationRecord>(32);

    let db_task = tokio::task::spawn_blocking(move || {
        let conn = app_state.conn.clone();
        let locked_conn = conn.lock().unwrap();
        
        let mut stmt = locked_conn.prepare("SELECT * from stations").unwrap();

        let station_records = stmt.query_and_then([], from_row::<StationRecord>).unwrap();

        let sender =
            PollSender::new(sender).sink_map_err(|_| panic!("Error sending StationRecord to channel."));

        // How to stream this without iterating through the collection that's already
        // loaded in memory? rusqlite is synchronous and doesn't have .next()
        station_records.forward(sender).await.unwrap();
    });

    db_task.await.unwrap();

    let stream = ReceiverStream::new(receiver);

    // How to turn this into json?
    Body::from_stream(stream);
}

// Do we want to handle db and other errors?
    // match spawn_blocking(|| read_all_records_in_chunks()).await {
    //     Ok(Ok(records)) => Json(records),
    //     Ok(Err(e)) => {
    //         eprintln!("Error reading records: {:?}", e);
    //         (
    //             axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    //             format!("Error reading records: {:?}", e),
    //         )
    //             .into_response()
    //     }
    //     Err(join_err) => {
    //         eprintln!("Thread join error: {:?}", join_err);
    //         (
    //             axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    //             format!("Thread join error: {:?}", join_err),
    //         )
    //             .into_response()
    //     }
    // }

// /// Create a `Stream` of `Bytes` that reads a SQLite DB in chunks on a separate thread.
// fn stream_records_db() -> impl Stream<Item = Result<Bytes, io::Error>> + Send + 'static {
//     // An unbounded channel for sending Bytes from the blocking thread to the async context.
//     let (tx, rx) = mpsc::unbounded_channel::<Result<Bytes, io::Error>>();

//     // Spawn a blocking task for all DB operations (to avoid blocking the async runtime).
//     spawn_blocking(move || {
//         // Wrap any error in an io::Error for simplicity
//         if let Err(err) = read_db_in_chunks(tx) {
//             eprintln!("DB error: {:?}", err);
//         }
//     });

//     // Convert the receiver into a `Stream` of `Result<Bytes, io::Error>`.
//     tokio_stream::wrappers::UnboundedReceiverStream::new(rx)
// }

// /// Read records in chunks from the database and send JSON pieces over `tx`.
// fn read_db_in_chunks(
//     tx: mpsc::UnboundedSender<Result<Bytes, io::Error>>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // Open or create the DB
//     let conn = Connection::open("example.db")?;

//     // For demo purposes, create and populate the table if it doesn't exist
//     setup_database(&conn)?;

//     // Start streaming JSON array
//     tx.send(Ok(Bytes::from("[")))?;

//     let mut offset = 0;
//     let chunk_size = 100;
//     let mut first_record = true;

//     loop {
//         // Query a chunk of records
//         let records = query_records_chunk(&conn, chunk_size, offset)?;

//         if records.is_empty() {
//             break;
//         }

//         for record in records {
//             let json = serde_json::to_string(&record)?;
//             if first_record {
//                 // First record => just write the object
//                 first_record = false;
//                 tx.send(Ok(Bytes::from(json)))?;
//             } else {
//                 // Subsequent record => prepend comma
//                 tx.send(Ok(Bytes::from(format!(",{}", json))))?;
//             }
//         }

//         offset += chunk_size;
//     }

//     // Close JSON array
//     tx.send(Ok(Bytes::from("]")))?;

//     Ok(())
// }

// /// Queries a chunk (LIMIT/OFFSET) of records from the database.
// fn query_records_chunk(
//     conn: &Connection,
//     chunk_size: i64,
//     offset: i64,
// ) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
//     let mut stmt = conn.prepare(
//         "SELECT id, name FROM records ORDER BY id LIMIT ? OFFSET ?",
//     )?;
//     let rows = stmt.query_map(params![chunk_size, offset], |row| {
//         Ok(Record {
//             id: row.get(0)?,
//             name: row.get(1)?,
//         })
//     })?;

//     let mut records = Vec::new();
//     for row_result in rows {
//         records.push(row_result?);
//     }
//     Ok(records)
// }

    // let json_stream = stream::iter(records.into_iter().enumerate().map(|(i, record)| {
    //     let json = serde_json::to_string(&record).unwrap();
    //     if i == 0 {
    //         // first record => start array
    //         Bytes::from(format!("[{}", json))
    //     } else {
    //         // subsequent records => prepend comma
    //         Bytes::from(format!(",{}", json))
    //     }
    // }))
    // // After we yield all records, close the array
    // .chain(stream::once(async {
    //     Bytes::from("]")
    // }));

    // HttpResponse::Ok()
    //     .content_type("application/json")
    //     .streaming(json_stream)

