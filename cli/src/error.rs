#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpClient(#[from] reqwest::Error),
}
