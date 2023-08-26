use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct InitUploadResponse {
    pub upload_url: Url,
}
