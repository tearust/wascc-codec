//! # Event Streaming
//!
//! This module contains data types and operation constants for the `wascc:eventstreams` capability.
//! For more information on append-only event streams, event sourcing, and how they apply
//! to waSCC actor development, check the documentation on [waSCC.dev](https://wascc.dev)

use crate::Sample;
use std::collections::HashMap;

/// Capability provider uses this operation to deliver an event to an actor
pub const OP_DELIVER_EVENT: &str = "DeliverEvent";
/// Actor invokes this operation on provider to write an event to a given event stream
pub const OP_WRITE_EVENT: &str = "WriteEvent";
/// Actor invokes this operation to execute a query against an event stream
pub const OP_QUERY_STREAM: &str = "QueryStream";

/// Represents an immutable event within a stream
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// The unique ID of the event
    pub event_id: String,
    /// The stream in which the event occurs
    pub stream: String,
    #[serde(default)]
    pub values: HashMap<String, String>,
}

/// The response from the provider after writing an event to a stream
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteResponse {
    /// Unique ID of the event written
    pub event_id: String,
}

/// A query against a given stream
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamQuery {
    /// ID of the stream to query
    pub stream_id: String,
    /// An optional time range limiter on the results of the query
    #[serde(default)]
    pub range: Option<TimeRange>,
    /// A maximum count to return from the query. 0 will return the maximum available
    /// (which may not include all events--consult the individual provider documentation to verify this behavior)
    pub count: u64,
}

impl Sample for StreamQuery {
    fn sample() -> Self {
        StreamQuery {
            stream_id: "stream1".to_string(),
            range: Some(TimeRange {
                min_time: 0,
                max_time: 1000,
            }),
            count: 42,
        }
    }
}

/// Results of a stream query
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamResults {
    /// The list of events returned by the query
    #[serde(default)]
    pub events: Vec<Event>,
}

/// Represents a timeslice range for a stream
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeRange {
    /// Minimum time after which events must have occurred to be in the results (seconds since the epoch)
    pub min_time: u64,
    /// Maximum time before which events must have occurred to be in the results (seconds since the epoch)
    pub max_time: u64,
}
