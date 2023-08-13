use reqwest::{blocking::RequestBuilder, header};
use serde::de::DeserializeOwned;
use url::Url;

use crate::factorio::api::error::ApiErrorKind;

use self::{
    detail::{ModDetailsRequest, ModDetailsResponse},
    error::ApiError,
    image::{
        ImageAddRequest, ImageAddResponse, ImageEditRequest, ImageEditResponse, ImageUploadRequest,
        ImageUploadResponse,
    },
    portal::{SearchQuery, SearchResponse, SearchResult},
    publish::{InitPublishRequest, InitPublishResponse, PublishRequest, PublishResponse},
    upload::{InitUploadRequest, InitUploadResponse, UploadRequest, UploadResponse},
};

pub mod detail;
pub mod error;
pub mod image;
pub mod portal;
pub mod publish;
pub mod upload;

const DEFAULT_BASE_URL: &str = "https://mods.factorio.com/api/";

pub struct ApiClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    api_token: Option<String>,
}

type Result<T> = core::result::Result<T, ApiError>;

impl ApiClient {
    pub fn new<T: Into<String>>(api_token: T) -> Self {
        Self::builder().api_token(api_token).build()
    }

    pub fn builder() -> ApiClientBuilder {
        ApiClientBuilder::new()
    }

    pub fn search(&self, query: SearchQuery) -> Result<SearchResponse> {
        self.get("mods", false, |r| r.query(&query))
    }

    pub fn info_short(&self, name: &str) -> Result<SearchResult> {
        self.get(&format!("mods/{}", name), false, |r| r)
    }

    pub fn info_full(&self, name: &str) -> Result<SearchResult> {
        self.get(&format!("mods/{}/full", name), false, |r| r)
    }

    pub fn init_upload(&self, data: InitUploadRequest) -> Result<InitUploadResponse> {
        self.post("v2/mods/upload", true, |r| r.multipart(data.into()))
    }

    pub fn upload(&self, url: Url, data: UploadRequest) -> Result<UploadResponse> {
        let form_data = data.try_into().map_err(|e| {
            ApiError::new(
                ApiErrorKind::ImageIo,
                format!("Could not read mod file {:?}", e),
                None,
            )
        })?;

        self.send(self.client.post(url).multipart(form_data), false)
    }

    pub fn edit_details(&self, data: ModDetailsRequest) -> Result<ModDetailsResponse> {
        self.post("v2/mods/edit_details", true, |r| r.multipart(data.into()))
    }

    pub fn add_image(&self, data: ImageAddRequest) -> Result<ImageAddResponse> {
        self.post("v2/mods/images/add", true, |r| r.multipart(data.into()))
    }

    pub fn upload_image(&self, url: Url, data: ImageUploadRequest) -> Result<ImageUploadResponse> {
        let form_data = data.try_into().map_err(|e| {
            ApiError::new(
                ApiErrorKind::ImageIo,
                format!("Could not read image file: {:?}", e),
                None,
            )
        })?;

        self.send(self.client.post(url).multipart(form_data), false)
    }

    pub fn edit_images(&self, data: ImageEditRequest) -> Result<ImageEditResponse> {
        self.post("v2/mods/images/edit", true, |r| r.multipart(data.into()))
    }

    pub fn init_publish(&self, data: InitPublishRequest) -> Result<InitPublishResponse> {
        self.post("v2/mods/init_publish", true, |r| r.multipart(data.into()))
    }

    pub fn publish(&self, url: Url, data: PublishRequest) -> Result<PublishResponse> {
        let form_data = data.try_into().map_err(|e| {
            ApiError::new(
                ApiErrorKind::ImageIo,
                format!("Could not read mod file: {:?}", e),
                None,
            )
        })?;

        self.send(self.client.post(url).multipart(form_data), false)
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
            if let Some(api_token) = &self.api_token {
                request = request.bearer_auth(api_token)
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
        Self::new("")
    }
}

#[derive(Default)]
pub struct ApiClientBuilder {
    client: Option<reqwest::blocking::Client>,
    base_url: Option<Url>,
    api_token: Option<String>,
}

impl ApiClientBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn client(mut self, client: reqwest::blocking::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn base_url(mut self, base_url: Url) -> Self {
        self.base_url = Some(base_url);
        self
    }

    pub fn api_token<T: Into<String>>(mut self, api_token: T) -> Self {
        self.api_token = Some(api_token.into());
        self
    }

    pub fn build(self) -> ApiClient {
        let client = self.client.unwrap_or(reqwest::blocking::Client::new());
        let base_url = self
            .base_url
            .unwrap_or(Url::parse(DEFAULT_BASE_URL).unwrap());

        ApiClient {
            client,
            base_url,
            api_token: self.api_token,
        }
    }
}
