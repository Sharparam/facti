use serde::Deserialize;
use url::Url;

use crate::reqwest::{FormContainer, FormLike};

#[derive(Debug, Deserialize)]
pub struct ImageAddResponse {
    pub upload_url: Url,
}

impl ImageAddResponse {
    pub fn upload_url(&self) -> &Url {
        &self.upload_url
    }
}

#[derive(Debug, Deserialize)]
pub struct ImageUploadResponse {
    pub id: String,
    pub url: Url,

    #[serde(rename = "thumbnail")]
    pub thumbnail_url: Url,
}

#[derive(Debug)]
pub struct ImageEditRequest {
    pub name: String,
    pub images: Vec<String>,
}

impl<T: FormLike> From<ImageEditRequest> for FormContainer<T> {
    fn from(request: ImageEditRequest) -> Self {
        let images = request.images.join(",");
        FormContainer(T::new().text("mod", request.name).text("images", images))
    }
}

#[derive(Debug, Deserialize)]
pub struct ImageEditResponse {
    pub success: bool,
    pub images: Vec<ImageUploadResponse>,
}
