use std::path::Path;

use reqwest::{blocking::RequestBuilder, header};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    detail::{ModDetailsRequest, ModDetailsResponse},
    error::{ApiError, ApiErrorKind},
    image::{ImageAddResponse, ImageEditRequest, ImageEditResponse, ImageUploadResponse},
    portal::{SearchQuery, SearchResponse, SearchResult},
    publish::{InitPublishResponse, PublishRequest, PublishResponse},
    reqwest::FormContainer,
    upload::{InitUploadResponse, UploadResponse},
    DEFAULT_BASE_URL,
};

pub struct ApiClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    api_key: Option<String>,
}

type Result<T> = core::result::Result<T, ApiError>;

impl ApiClient {
    pub fn new<T: Into<String>>(api_key: Option<T>) -> Self {
        Self::builder().api_key(api_key).build()
    }

    pub fn builder() -> ApiClientBuilder {
        ApiClientBuilder::new()
    }

    pub fn search(&self, query: &SearchQuery) -> Result<SearchResponse> {
        self.get("mods", false, |r| r.query(query))
    }

    pub fn info_short(&self, name: &str) -> Result<SearchResult> {
        self.get(&format!("mods/{}", name), false, |r| r)
    }

    pub fn info_full(&self, name: &str) -> Result<SearchResult> {
        self.get(&format!("mods/{}/full", name), false, |r| r)
    }

    pub fn init_upload<T: Into<String>>(&self, name: T) -> Result<InitUploadResponse> {
        let form = reqwest::blocking::multipart::Form::new().text("mod", name.into());
        self.post("v2/mods/upload", true, |r| r.multipart(form))
    }

    pub fn upload(&self, url: Url, path: &Path) -> Result<UploadResponse> {
        let form = reqwest::blocking::multipart::Form::new()
            .file("file", path)
            .map_err(|e| {
                ApiError::new(
                    ApiErrorKind::ImageIo,
                    format!("Could not read mod file {:?}", e),
                    None,
                )
            })?;

        self.send(self.client.post(url).multipart(form), false)
    }

    pub fn edit_details(&self, data: ModDetailsRequest) -> Result<ModDetailsResponse> {
        let container: FormContainer<reqwest::blocking::multipart::Form> = data.into();
        let form = container.into_inner();
        self.post("v2/mods/edit_details", true, |r| r.multipart(form))
    }

    pub fn add_image<T: Into<String>>(&self, name: T) -> Result<ImageAddResponse> {
        self.post("v2/mods/images/add", true, |r| {
            r.multipart(reqwest::blocking::multipart::Form::new().text("mod", name.into()))
        })
    }

    pub fn upload_image(&self, url: Url, path: &Path) -> Result<ImageUploadResponse> {
        let form = reqwest::blocking::multipart::Form::new()
            .file("image", path)
            .map_err(|e| {
                ApiError::new(
                    ApiErrorKind::ImageIo,
                    format!("Could not read image file: {:?}", e),
                    None,
                )
            })?;

        self.send(self.client.post(url).multipart(form), false)
    }

    pub fn edit_images(&self, data: ImageEditRequest) -> Result<ImageEditResponse> {
        let container: FormContainer<reqwest::blocking::multipart::Form> = data.into();
        let form = container.into_inner();
        self.post("v2/mods/images/edit", true, |r| r.multipart(form))
    }

    pub fn init_publish<T: Into<String>>(&self, name: T) -> Result<InitPublishResponse> {
        let form = reqwest::blocking::multipart::Form::new().text("mod", name.into());
        self.post("v2/mods/init_publish", true, |r| r.multipart(form))
    }

    pub fn publish(&self, url: Url, data: PublishRequest, path: &Path) -> Result<PublishResponse> {
        let container: FormContainer<reqwest::blocking::multipart::Form> = data.into();
        let mut form = container.into_inner();
        form = form.file("file", path).map_err(|e| {
            ApiError::new(
                ApiErrorKind::ImageIo,
                format!("Could not read mod file {:?}", e),
                None,
            )
        })?;

        self.send(self.client.post(url).multipart(form), false)
    }

    fn url(&self, path: &str) -> Result<Url> {
        self.base_url.join(path).map_err(|_| {
            ApiError::new(
                ApiErrorKind::UrlParseFailed,
                format!("Failed to join base URL with path {}", path),
                None,
            )
        })
    }

    fn send<T>(&self, request: reqwest::blocking::RequestBuilder, auth: bool) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut request = request.header(header::USER_AGENT, "facti");
        if auth {
            if let Some(api_key) = &self.api_key {
                request = request.bearer_auth(api_key)
            } else {
                return Err(ApiError::new(
                    ApiErrorKind::MissingApiKey,
                    "Missing API key",
                    None,
                ));
            }
        }

        let response = request.send()?;

        if response.status().is_success() {
            Ok(response.json::<T>()?)
        } else {
            Err(response.into())
        }
    }

    fn get<T, F>(&self, path: &str, auth: bool, f: F) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        F: FnOnce(RequestBuilder) -> RequestBuilder,
    {
        let url = self.url(path)?;
        let request = f(self.client.get(url));

        self.send::<T>(request, auth)
    }

    fn post<T, F>(&self, path: &str, auth: bool, f: F) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        F: FnOnce(RequestBuilder) -> RequestBuilder,
    {
        let url = self.url(path)?;
        let request = f(self.client.post(url));

        self.send::<T>(request, auth)
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new::<String>(None)
    }
}

#[derive(Default)]
pub struct ApiClientBuilder {
    client: Option<reqwest::blocking::Client>,
    base_url: Option<Url>,
    api_key: Option<String>,
}

impl ApiClientBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn client(mut self, client: Option<reqwest::blocking::Client>) -> Self {
        self.client = client;
        self
    }

    pub fn base_url(mut self, base_url: Option<Url>) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn api_key<T: Into<String>>(mut self, api_key: Option<T>) -> Self {
        self.api_key = api_key.map(Into::into);
        self
    }

    pub fn build(self) -> ApiClient {
        let client = self.client.unwrap_or_default();
        let base_url = self
            .base_url
            .unwrap_or(Url::parse(DEFAULT_BASE_URL).unwrap());

        ApiClient {
            client,
            base_url,
            api_key: self.api_key,
        }
    }
}
