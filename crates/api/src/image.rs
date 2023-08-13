use std::io;

use serde::Deserialize;
use url::Url;

#[derive(Debug)]
pub struct ImageAddRequest {
    pub name: String,
}

impl ImageAddRequest {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self { name: name.into() }
    }
}

impl From<ImageAddRequest> for reqwest::blocking::multipart::Form {
    fn from(request: ImageAddRequest) -> Self {
        reqwest::blocking::multipart::Form::new().text("mod", request.name)
    }
}

#[derive(Debug, Deserialize)]
pub struct ImageAddResponse {
    pub upload_url: Url,
}

impl ImageAddResponse {
    pub fn upload_url(&self) -> &Url {
        &self.upload_url
    }
}

#[derive(Debug)]
pub struct ImageUploadRequest {
    pub path: String,
}

impl TryFrom<ImageUploadRequest> for reqwest::blocking::multipart::Form {
    type Error = io::Error;

    fn try_from(request: ImageUploadRequest) -> Result<Self, Self::Error> {
        reqwest::blocking::multipart::Form::new().file("image", request.path)
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

impl From<ImageEditRequest> for reqwest::blocking::multipart::Form {
    fn from(request: ImageEditRequest) -> Self {
        let images = request.images.join(",");
        reqwest::blocking::multipart::Form::new()
            .text("mod", request.name)
            .text("images", images)
    }
}

#[derive(Debug, Deserialize)]
pub struct ImageEditResponse {
    pub success: bool,
    pub images: Vec<ImageUploadResponse>,
}
