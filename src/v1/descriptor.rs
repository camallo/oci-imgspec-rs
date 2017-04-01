use std::collections;

/// OCI Content Descriptor
#[derive(Debug,Default,Deserialize,Serialize)]
pub struct Descriptor {
    #[serde(rename = "mediaType")]
    media_type: String,
    digest: String,
    data: Option<String>,
    size: u64,
    urls: Option<Vec<String>>,
    annotations: Option<collections::HashMap<String, String>>,
}
