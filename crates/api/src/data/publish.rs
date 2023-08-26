use serde::Deserialize;
use url::Url;

use crate::reqwest::{FormContainer, FormLike};

use super::detail::{Category, License};

#[derive(Debug)]
pub struct PublishRequest {
    pub path: String,
    pub description: Option<String>,
    pub category: Option<Category>,
    pub license: Option<License>,
    pub source_url: Option<Url>,
}

impl PublishRequest {
    pub fn new<T: Into<String>>(path: T) -> Self {
        Self {
            path: path.into(),
            description: None,
            category: None,
            license: None,
            source_url: None,
        }
    }

    pub fn description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    pub fn license(mut self, license: License) -> Self {
        self.license = Some(license);
        self
    }

    pub fn source_url<T: Into<Url>>(mut self, source_url: T) -> Self {
        self.source_url = Some(source_url.into());
        self
    }
}

impl<T: FormLike> From<PublishRequest> for FormContainer<T> {
    fn from(req: PublishRequest) -> Self {
        let mut form = T::new();

        if let Some(description) = req.description {
            form = form.text("description", description);
        }

        if let Some(category) = req.category {
            form = form.text("category", category.to_string());
        }

        if let Some(license) = req.license {
            form = form.text("license", license.to_string());
        }

        if let Some(source_url) = req.source_url {
            form = form.text("source_url", source_url.to_string());
        }

        FormContainer(form)
    }
}

#[derive(Debug, Deserialize)]
pub struct PublishResponse {
    pub success: bool,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub(crate) struct InitPublishResponse {
    pub upload_url: Url,
}
