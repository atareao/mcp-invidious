use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("missing required environment variable: {0}")]
    MissingEnvVar(&'static str),

    #[error("invalid environment variable {0}: {1}")]
    InvalidEnvVar(&'static str, String),

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Invidious API error: {0}")]
    InvidiousApi(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
