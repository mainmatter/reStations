use axum::response::Json;
use crate::state::SharedAppState;
use crate::types::station_record::StationRecord;
use axum::extract::State;
use serde_rusqlite::from_row;
use tokio::sync::mpsc;

#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> Json<Vec<StationRecord>> {
    let (sender, mut receiver) = mpsc::channel::<StationRecord>(32);

    let db_task = tokio::task::spawn_blocking(move || {
        let conn = app_state.conn.clone();
        let locked_conn = conn.lock().unwrap();

        let mut stmt = locked_conn.prepare("SELECT * from stations").unwrap();
        let station_results = stmt.query_and_then([], from_row::<StationRecord>).unwrap();

        for station in station_results {
            let station = station.unwrap();
            sender.blocking_send(station).unwrap();
        }
    });

    db_task.await.unwrap();

    let mut stations: Vec<StationRecord> = Vec::new();

    while let Some(station) = receiver.recv().await {
        stations.push(station);
    }

    Json(stations)
}
