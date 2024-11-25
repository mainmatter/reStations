#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    Io(#[from] tokio::io::Error),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] csv_async::Error),
}
