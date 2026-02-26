//! Output formatting helpers for the Circle CLI.

use std::io::Write;

use serde::Serialize;

/// Output format for CLI results.
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum OutputFormat {
    /// JSON output (default).
    #[default]
    Json,
    /// Human-readable text output.
    Text,
}

/// Print a serializable result to stdout in the requested format.
///
/// Both `Json` and `Text` formats produce pretty-printed JSON; `Text` is
/// provided as a convenience alias for human-readable output.
pub(crate) fn print_result<T: Serialize>(val: &T, format: OutputFormat) {
    let _ = format; // both variants use the same pretty-JSON output
    match serde_json::to_string_pretty(val) {
        Ok(s) => {
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            let _ = writeln!(handle, "{s}");
        }
        Err(e) => {
            tracing::error!("Serialization error: {e}");
        }
    }
}
