use serde::Deserialize;

/// Error response returned by the JetEmail API.
#[derive(Clone, Debug, Deserialize)]
pub struct ErrorResponse {
    /// Error message from the API.
    #[serde(alias = "error")]
    pub message: String,

    /// HTTP status code.
    #[serde(default)]
    pub status_code: u16,
}

/// Top-level error type for the JetEmail SDK.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP or network error from reqwest.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// API error returned by JetEmail.
    #[error("JetEmail API error ({status_code}): {message}")]
    JetEmail {
        message: String,
        status_code: u16,
        response: Option<serde_json::Value>,
    },

    /// Failed to parse the API response.
    #[error("Failed to parse response: {0}")]
    Parse(String),
}

/// Convenience result type for the JetEmail SDK.
pub type Result<T> = std::result::Result<T, Error>;
