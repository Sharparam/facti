use std::io;

use serde::Deserialize;
use url::Url;

#[derive(Debug)]
pub struct InitUploadRequest {
    name: String,
}

impl InitUploadRequest {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self { name: name.into() }
    }
}

impl From<InitUploadRequest> for reqwest::blocking::multipart::Form {
    fn from(req: InitUploadRequest) -> Self {
        reqwest::blocking::multipart::Form::new().text("mod", req.name)
    }
}

#[derive(Debug, Deserialize)]
pub struct InitUploadResponse {
    pub upload_url: Url,
}

#[derive(Debug)]
pub struct UploadRequest {
    pub path: String,
}

impl UploadRequest {
    pub fn new<T: Into<String>>(path: T) -> Self {
        Self { path: path.into() }
    }
}

impl TryFrom<UploadRequest> for reqwest::blocking::multipart::Form {
    type Error = io::Error;

    fn try_from(req: UploadRequest) -> Result<Self, Self::Error> {
        reqwest::blocking::multipart::Form::new().file("file", req.path)
    }
}

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub success: bool,
}
