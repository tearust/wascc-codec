//! # Logging Data Types
//!
//! This module contains data types for the `wascc:logging` capability provider

use crate::Sample;

/// An operation to request a log write
pub const OP_LOG: &str = "WriteLog";

/// Represents a request to write a log entry. Use this type of log entry if you are
/// pulling or aggregating logs on a per-actor basis from the host. If you just need
/// to dump debug information to the log, use the built-in simple `println` or `consoleLog`
/// function from the actor API
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteLogRequest {
    /// Corresponds to the log level
    ///
    /// "OFF"=0 , "ERROR"=1, "WARN"=2, "INFO"=3, "DEBUG"=4, "TRACE"=5
    pub level: u32,
    /// A string that represents the body of the log message
    pub body: String,
}

impl Sample for WriteLogRequest {
    fn sample() -> Self {
        WriteLogRequest {
            level: 4,
            body: "This is a debug message".to_string(),
        }
    }
}
