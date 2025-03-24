use crate::{connect_pool, DbPool};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use restations_config::DatabaseConfig;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions};
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;

pub mod stations;

/// Sets up a dedicated database to be used in a test case.
///
/// This sets up a dedicated database as a fork of the main test database as configured in `.env.test`. The database can be used in a test case to ensure the test case is isolated from other test cases. The function returns a connection pool connected to the created database.
/// This function is automatically called by the [`restations-macros::db_test`] macro. The return connection pool is passed to the test case via the [`restations-macros::DbTestContext`].
#[allow(unused)]
pub async fn setup_db(config: &DatabaseConfig) -> DbPool {
    let test_db_config = prepare_db(config).await;
    connect_pool(test_db_config.clone())
        .await
        .expect("Could not connect to database!")
}

/// Drops a dedicated database for a test case.
///
/// This function is automatically called by the [`restations-macros::db_test`] macro. It ensures test-specific database are cleaned up after each test run so we don't end up with large numbers of unused databases.
pub async fn teardown_db(db_pool: DbPool) {
    let options = db_pool.connect_options();
    let db_file = options.get_filename();

    let message = format!("Failed to delete database {:?}!", db_file);
    fs::remove_file(db_file).await.expect(&message);
}

async fn prepare_db(config: &DatabaseConfig) -> DatabaseConfig {
    let db_config = parse_db_config(&config.url);
    let db_file = db_config.get_filename();
    let db_file_name = db_file
        .file_name()
        .expect("Failed to get file name of main test database!")
        .to_str()
        .expect("Failed to get file name of main test database!");

    let test_db_file_name = build_test_db_file_name(db_file_name);
    fs::copy(db_file, test_db_file_name.clone())
        .await
        .expect("Failed to copy test database from main test database!");

    let db_config = db_config.filename(test_db_file_name);

    DatabaseConfig {
        url: db_config.to_url_lossy().to_string(),
    }
}

fn build_test_db_file_name(base_name: &str) -> PathBuf {
    let temp_dir = env::temp_dir();
    let rand_string: String = rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    temp_dir.join(format!("{}-{}", rand_string, base_name))
}

fn parse_db_config(url: &str) -> SqliteConnectOptions {
    SqliteConnectOptions::from_str(url).expect("Invalid DATABASE_URL!")
}
