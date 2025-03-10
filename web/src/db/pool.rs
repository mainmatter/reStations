pub use sqlx::sqlite::SqlitePool as DbPool;

pub async fn create_pool(db_file: &str) -> DbPool {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(false);
    sqlx::sqlite::SqlitePool::connect_with(options)
        .await
        .expect("Failed to create pool")
}
