#[derive(Debug,Default,Deserialize,Serialize)]
pub struct ManifestObj {
    #[serde(rename = "mediaType")]
    media_type: String,
    pub platform: PlatformObj,
}

#[derive(Debug,Default,Deserialize,Serialize)]
pub struct PlatformObj {
    pub architecture: String,
    pub os: String,
    #[serde(rename = "os.version")]
    pub os_version: Option<String>,
    #[serde(rename = "os.features")]
    pub os_features: Option<Vec<String>>,
    pub variant: Option<String>,
    pub features: Option<Vec<String>>,
}
