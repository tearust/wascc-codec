//! # Binary object storage and streaming
//!
//! This module contains data types for the `wascc:blobstore` capability provider. For more information on
//! how the blob store capability works within the constraints of a WebAssembly host runtime, check out
//! the documentation on [waSCC.dev](https://wascc.dev)

use crate::Sample;

/// Guest sends a Container to the capability provider, receives a Container back
pub const OP_CREATE_CONTAINER: &str = "CreateContainer";
/// Guest sends a Container to the capability provider, lack of error indicates success
pub const OP_REMOVE_CONTAINER: &str = "RemoveContainer";
/// Guest sends a Blob to the capability provider, lack of error indicates success
pub const OP_REMOVE_OBJECT: &str = "RemoveObject";
/// Guest sends a Container to the capability provider, receives a BlobList back
pub const OP_LIST_OBJECTS: &str = "ListObjects";
/// Guest sends a FileChunk to capability provider for storing as part of a Blob, lack of error indicates success
pub const OP_UPLOAD_CHUNK: &str = "UploadChunk";
/// Guest sends a StreamRequest to the capability provider, immediate termination w/success. Guest will then
/// start receiving OP_RECEIVE_CHUNK operations from the provider as chunks are streamed to the guest
pub const OP_START_DOWNLOAD: &str = "StartDownload";
/// Guest sends a metadata-carrying FileChunk to initiate an upload, lack of error is success
pub const OP_START_UPLOAD: &str = "StartUpload";
/// Guest will receive a FileChunk for each piece of a file requested to download
pub const OP_RECEIVE_CHUNK: &str = "ReceiveChunk";
/// Query information on a single blob. Guest sends an incomplete blob struct and gets a complete one in return
pub const OP_GET_OBJECT_INFO: &str = "GetObjectInfo";

/// Represents a single chunk of a segmented file stream
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChunk {
    /// A sequence number that can be used for retry and ordering logic
    pub sequence_no: u64,
    /// The container in which this file exists
    pub container: String,
    /// The unique ID of the blob
    pub id: String,
    /// Total number of bytes in the entire blob
    pub total_bytes: u64,
    /// The number of bytes within any given chunk. Note that the last chunk in a file stream may be less than `chunk_size`
    pub chunk_size: u64,
    /// The raw bytes contained in this chunk
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub chunk_bytes: Vec<u8>,
}

impl Sample for FileChunk {
    fn sample() -> Self {
        FileChunk {
            sequence_no: 5,
            container: "container".to_string(),
            id: "blob".to_string(),
            total_bytes: 53400,
            chunk_size: 1024,
            chunk_bytes: vec![1, 2, 3, 4, 5],
        }
    }
}

/// Represents a container within a blob store
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub id: String,
}

/// Used to hold a list of containers
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerList {
    #[serde(default)]
    pub containers: Vec<Container>,
}

impl Sample for ContainerList {
    fn sample() -> Self {
        ContainerList {
            containers: vec![Container {
                id: "container".to_string(),
            }],
        }
    }
}

/// Metadata about a blob, not the raw bytes
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    /// Unique ID of the blob
    pub id: String,
    /// Container in which the blob resides
    pub container: String,
    /// Total number of bytes of the blob (file size)
    pub byte_size: u64,
}

/// A wrapper for a list of blobs
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobList {
    #[serde(default)]
    pub blobs: Vec<Blob>,
}

/// A request to begin downloading a stream for a blob
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamRequest {
    /// The unique ID of the requested blob
    pub id: String,
    /// The container of the requested blob
    pub container: String,
    /// The preferred size of chunks to be delivered. Consumers must not assume this is the size of the chunks they will get
    pub chunk_size: u64,
}

/// Metadata about an in-progress file transfer
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    /// Unique ID of the blob
    pub blob_id: String,
    /// ID of the container
    pub container: String,
    /// Size of chunks being transferred
    pub chunk_size: u64,
    /// Total number of bytes being transferred
    pub total_size: u64,
    /// Total number of chunks being transferred
    pub total_chunks: u64,
}
