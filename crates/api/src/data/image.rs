use serde::{Deserialize, Serialize};
use tracing::debug;
use url::Url;

use crate::reqwest::{FormContainer, FormLike};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub url: Url,
    pub thumbnail_url: Url,
}

impl Image {
    pub fn new<T: Into<String>>(id: T) -> Self {
        let id = id.into();
        Self {
            id: id.clone(),
            url: Url::parse(&format!(
                "https://assets-mod.factorio.com/assets/{}.png",
                id
            ))
            .unwrap(),
            thumbnail_url: Url::parse(&format!(
                "https://assets-mod.factorio.com/assets/{}.thumb.png",
                id
            ))
            .unwrap(),
        }
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

pub(crate) fn parse_html_images(html: &str) -> Vec<Image> {
    debug!("Extracting images from HTML");
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse(".mod-page-info .gallery img").unwrap();

    let mut images = Vec::<Image>::new();

    for element in document.select(&selector) {
        let elem = element.value();
        if let Some(id) = elem.attr("data-filename") {
            debug!("Found image with ID {}", id);
            images.push(Image::new(id));
        }
    }

    debug!("Found {} images", images.len());

    images
}
