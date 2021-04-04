//! # Extras
//!
//! Miscellaneous operations that might be common across many different types of actors that
//! shouldn't require a full capability provider plugin, like random numbers, sequence
//! numbers, etc.

use crate::Sample;

/// The operation to request the generation of a GUID
pub const OP_REQUEST_GUID: &str = "RequestGuid";
/// The operation to request a new sequence number
pub const OP_REQUEST_SEQUENCE: &str = "RequestSequence";
/// The operation to request a random number with an optional range
pub const OP_REQUEST_RANDOM: &str = "RequestRandom";

/// The results of a generation request. The struct has been flattened rather than
/// using an enum variant in order to make serialization compatibility easier
/// with other parsers that might not handle enums in a predictable way.
#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorResult {
    /// The requested GUID, if it exists
    pub guid: Option<String>,
    /// The requested sequence number (0 if not requested)
    pub sequence_number: u64,
    /// The requested random number (0 if not requested)
    pub random_number: u32,
}

impl Sample for GeneratorResult {
    fn sample() -> Self {
        GeneratorResult {
            guid: Some("insert_generated_guid_here".to_string()),
            sequence_number: 0,
            random_number: 0,
        }
    }
}

/// A request for the generation of numbers that standalone actors cannot
/// normally produce because they require random numbers or cryptography
/// libraries. This struct has been flattened and avoids enum variants
/// to maintain a high level of msgpack serialization compatibility with
/// other languages and parsers.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorRequest {
    /// Indicates a request for a GUID
    pub guid: bool,
    /// Indicates a request for a sequence number
    pub sequence: bool,
    /// Indicates a request for a random number
    pub random: bool,
    /// Minimum value for a random number request
    pub min: u32,
    /// Maximum value for a random number request
    pub max: u32,
}
