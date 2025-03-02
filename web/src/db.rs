use super::types::station_record::StationRecord;
use r2d2_sqlite::SqliteConnectionManager;
use serde_rusqlite::{columns_from_statement, from_row_with_columns};
use tokio::sync::mpsc;

#[derive(serde::Serialize, Debug, thiserror::Error)]
pub enum DbError {
    #[error("Unknown error")]
    UnknownError,

    #[error("Database error: {0}")]
    Database(String),

    #[error("RecordNotFound: {0}")]
    RecordNotFound(String),
}

impl From<rusqlite::Error> for DbError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Database(value.to_string())
    }
}

type Sender = mpsc::Sender<Result<StationRecord, DbError>>;

pub type Connection = rusqlite::Connection;
pub type Pool = r2d2::Pool<SqliteConnectionManager>;

pub fn create_pool(db_file: &str) -> Pool {
    let manager = SqliteConnectionManager::file(db_file);
    Pool::new(manager).expect("Failed to create pool")
}

pub fn find_station(db: &Connection, place_id: &String) -> Result<StationRecord, DbError> {
    // OSDM place id maps to station's uic
    let mut stmt = db.prepare("SELECT * from stations where uic=?")?;

    let columns = columns_from_statement(&stmt);
    let result = stmt.query_row([place_id], |row| {
        Ok(from_row_with_columns::<StationRecord>(row, &columns).unwrap())
    });

    match result {
        Ok(result) => Ok(result),
        Err(rusqlite::Error::QueryReturnedNoRows) => Err(DbError::RecordNotFound(format!(
            "Could not find station with uic #{}",
            &place_id
        ))),
        _ => todo!("Unexpected error at db::find_station"),
    }
}

pub fn find_all_stations(db: &Connection) -> Result<Vec<StationRecord>, DbError> {
    let mut stmt = db
        .prepare("SELECT * from stations WHERE uic IS NOT NULL")
        .unwrap();

    let columns = columns_from_statement(&stmt);
    let rows = stmt
        .query_map([], |row| {
            Ok(from_row_with_columns::<StationRecord>(row, &columns).unwrap())
        })
        .unwrap();

    let mut result: Vec<StationRecord> = Vec::new();
    for row in rows {
        result.push(row.unwrap());
    }
    Ok(result)
}

pub fn search_all_stations(db: &Connection, name: &str) -> Result<Vec<StationRecord>, DbError> {
    let pattern = format!("%{}%", name);
    let mut stmt = db.prepare("SELECT * from stations  WHERE uic IS NOT NULL AND uic != '' AND (name like ?1 OR info_de like ?1 OR info_en like ?1 OR info_es like ?1) OR info_fr like ?1 OR info_it like ?1 OR info_nb like ?1 OR info_nl like ?1 OR info_cs like ?1 OR info_da like ?1 OR info_hu like ?1 OR info_ja like ?1 OR info_ko like ?1 OR info_pl like ?1 OR info_pt like ?1 OR info_ru like ?1 OR info_sv like ?1 OR info_tr like ?1 OR info_zh like ?1")?;

    let columns = columns_from_statement(&stmt);
    let rows = stmt
        .query_map([pattern], |row| {
            Ok(from_row_with_columns::<StationRecord>(row, &columns).unwrap())
        })
        .unwrap();

    let mut result: Vec<StationRecord> = Vec::new();
    for row in rows {
        result.push(row.unwrap());
    }
    Ok(result)
}

pub fn stream_all_stations(db: &Connection, sender: Sender) {
    let mut stmt = db
        .prepare("SELECT * from stations WHERE uic IS NOT NULL")
        .unwrap();

    let columns = columns_from_statement(&stmt);
    let stations = stmt
        .query_map([], |row| {
            Ok(from_row_with_columns::<StationRecord>(row, &columns).unwrap())
        })
        .unwrap();

    for station in stations {
        sender.blocking_send(Ok(station.unwrap())).ok();
    }
}
