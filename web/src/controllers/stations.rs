use axum::response::Json;
// TODO figure out why only using types:: doesn't work here
use crate::types::station_record::StationRecord;
use crate::state::SharedAppState;
use axum::extract::State;
use serde_rusqlite::from_rows;

// TODO what's the right notation here for a collection of station records?
/// Responds with a [`[StationRecord]`], encoded as JSON.
#[axum::debug_handler]
pub async fn list(State(app_state): State<SharedAppState>) -> Json<Vec<StationRecord>> {
    let conn = app_state.conn.clone();
    let locked_conn = conn.lock().unwrap();

    let mut stmt = locked_conn.prepare("SELECT * from stations").unwrap();

    // let mut rows = stmt.query([]).unwrap();

    // while let Ok(row) = rows.next() {
    //     println!("{:?}", row.unwrap());
    // }

    let res = from_rows::<StationRecord>(stmt.query([]).unwrap());
    let mut stations:Vec<StationRecord> = Vec::new();

    for station in res {
        stations.push(station.unwrap());
        // println!("{:?}", station.unwrap());
    }


    Json(stations)
}
