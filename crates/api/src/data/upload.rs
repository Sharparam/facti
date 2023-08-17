use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct InitUploadResponse {
    pub upload_url: Url,
}

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub success: bool,
}
