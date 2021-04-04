//! # Message Broker Data Types
//!
//! This module contains data types for the `wascc:messaging` capability provider

use crate::Sample;

/// The operation to publish a message from an actor
pub const OP_PUBLISH_MESSAGE: &str = "Publish";
/// The operation to deliver a message to an actor
pub const OP_DELIVER_MESSAGE: &str = "DeliverMessage";
/// The operation for an actor to perform a request-reply operation
pub const OP_PERFORM_REQUEST: &str = "Request";

/// A representation of a broker message
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrokerMessage {
    /// The message subject or topic
    pub subject: String,
    /// The reply-to field of the subject. This will be empty if there is no reply subject
    pub reply_to: String,
    /// The raw bytes of the message. Encoding/contents is determined by applications out of band
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
}

/// A request for the broker to make a request-and-reply publication. Inbox management
/// is handled by the provider implementation, not by the actor
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMessage {
    /// Subject on which to publish the request
    pub subject: String,
    /// Raw body of the request message
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub body: Vec<u8>,
    /// The timeout (milliseconds) to await a reply before giving up
    #[serde(rename = "timeout")]
    pub timeout_ms: i64,
}

impl Sample for RequestMessage {
    fn sample() -> Self {
        RequestMessage {
            subject: "user.profile.175".to_string(),
            body: b"raw query bytes".to_vec(),
            timeout_ms: 100,
        }
    }
}
