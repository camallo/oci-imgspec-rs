//! OCI image-spec v1.

use std::collections;

mod descriptor;
pub use self::descriptor::Descriptor;

mod manifest;
pub use self::manifest::Manifest;

pub mod manifest_list;

/// OCI image-spec revision implemented by this library.
pub static OCI_REVISION: &'static str = "1.0.0-rc4";

/// OCI Manifest List
#[derive(Debug,Default,Deserialize,Serialize)]
pub struct ManifestList {
    #[serde(rename = "schemaVersion")]
    schema_version: u8,
    #[serde(rename = "mediaType")]
    media_type: Option<String>,
    pub manifests: Vec<manifest_list::ManifestObj>,
    pub annotations: collections::HashMap<String, String>,
}

