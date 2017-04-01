use std::collections;
use v1::*;

/// OCI Manifest
#[derive(Debug,Default,Deserialize,Serialize)]
pub struct Manifest {
    #[serde(rename = "schemaVersion")]
    schema_version: u8,
    #[serde(rename = "mediaType")]
    media_type: Option<String>,
    pub config: Descriptor,
    pub layers: Vec<Descriptor>,
    pub annotations: Option<collections::HashMap<String, String>>,
}
