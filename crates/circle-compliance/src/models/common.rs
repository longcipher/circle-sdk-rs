//! Common types shared across the Compliance Engine API.

/// Error response body returned by the Circle API on non-2xx status codes.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ApiErrorBody {
    /// Numeric error code.
    pub code: i32,
    /// Human-readable error message.
    pub message: String,
}
