//! Error types for the `circle-compliance` crate.

/// Errors that can occur when calling the Circle Compliance Engine API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP transport error from the underlying HTTP client.
    #[error("HTTP transport error: {0}")]
    Http(String),

    /// The Circle API returned a non-2xx response with an error payload.
    #[error("Circle API error {code}: {message}")]
    Api {
        /// Numeric error code from the Circle API response body.
        code: i32,
        /// Human-readable error message from the Circle API response body.
        message: String,
    },

    /// Failed to deserialize the API response JSON into the expected type.
    #[error("Failed to deserialize response: {0}")]
    Deserialize(#[from] serde_json::Error),

    /// A caller-supplied parameter was invalid before the request was sent.
    #[error("Invalid parameter: {0}")]
    InvalidParam(String),
}
