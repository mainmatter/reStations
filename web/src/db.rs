use super::types::station_record::StationRecord;
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

impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value.to_string())
    }
}

type Sender = mpsc::Sender<Result<StationRecord, DbError>>;

pub use sqlx::sqlite::SqlitePool as DbPool;

pub async fn create_pool(db_file: &str) -> DbPool {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(false);
    sqlx::sqlite::SqlitePool::connect_with(options)
        .await
        .expect("Failed to create pool")
}

pub async fn find_station(db: &DbPool, place_id: &String) -> Result<StationRecord, DbError> {
    sqlx::query_as!(
        StationRecord,
        "SELECT * FROM stations WHERE uic = $1",
        place_id
    )
    .fetch_optional(db)
    .await?
    .ok_or(DbError::RecordNotFound(format!(
        "Could not find station with uic #{}",
        &place_id
    )))
}

pub async fn find_all_stations(db: &DbPool) -> Result<Vec<StationRecord>, DbError> {
    let stations = sqlx::query_as!(
        StationRecord,
        "SELECT * FROM stations WHERE uic IS NOT NULL"
    )
    .fetch_all(db)
    .await?;
    Ok(stations)
}

pub async fn search_all_stations(db: &DbPool, name: &str) -> Result<Vec<StationRecord>, DbError> {
    let pattern = format!("%{}%", name);
    let stations = sqlx::query_as!(StationRecord, "SELECT * from stations  WHERE uic IS NOT NULL AND (name like $1 OR info_de like $1 OR info_en like $1 OR info_es like $1 OR info_fr like $1 OR info_it like $1 OR info_nb like $1 OR info_nl like $1 OR info_cs like $1 OR info_da like $1 OR info_hu like $1 OR info_ja like $1 OR info_ko like $1 OR info_pl like $1 OR info_pt like $1 OR info_ru like $1 OR info_sv like $1 OR info_tr like $1 OR info_zh like $1)", pattern)
            .fetch_all(db)
            .await?;
    Ok(stations)
}

pub async fn stream_all_stations(db: &DbPool, sender: Sender) {
    let stations = sqlx::query_as!(
        StationRecord,
        "SELECT * FROM stations WHERE uic IS NOT NULL"
    )
    .fetch_all(db)
    .await
    .unwrap();

    for station in stations {
        sender.blocking_send(Ok(station)).ok();
    }
}
