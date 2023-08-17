use std::path::Path;

use reqwest::{header, multipart::Form, RequestBuilder};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    data::{
        detail::{ModDetailsRequest, ModDetailsResponse},
        image::{ImageAddResponse, ImageEditRequest, ImageEditResponse, ImageUploadResponse},
        portal::{SearchQuery, SearchResponse, SearchResult},
        publish::{InitPublishResponse, PublishRequest, PublishResponse},
        upload::{InitUploadResponse, UploadResponse},
    },
    error::{ApiError, ApiErrorKind},
    reqwest::FormContainer,
    DEFAULT_BASE_URL,
};

use super::{error, reqwest::AsyncFormFile};

pub struct ApiClient {
    client: reqwest::Client,
    base_url: Url,
    api_key: Option<String>,
}

type Result<T> = core::result::Result<T, ApiError>;

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Default::default(),
            base_url: Url::parse(DEFAULT_BASE_URL).unwrap(),
            api_key: None,
        }
    }

    pub fn with_api_key<T: Into<String>>(api_key: T) -> Self {
        Self {
            api_key: Some(api_key.into()),
            ..Default::default()
        }
    }

    pub fn builder() -> ApiClientBuilder {
        ApiClientBuilder::new()
    }

    pub async fn search(&self, query: &SearchQuery) -> Result<SearchResponse> {
        self.get("mods", false, |r| r.query(query)).await
    }

    pub async fn info_short(&self, name: &str) -> Result<SearchResult> {
        self.get(&format!("mods/{}", name), false, |r| r).await
    }

    /// Get detailed information about a mod by its internal name.
    pub async fn info_full(&self, name: &str) -> Result<SearchResult> {
        self.get(&format!("mods/{}/full", name), false, |r| r).await
    }

    pub async fn init_upload<T: Into<String>>(&self, name: T) -> Result<InitUploadResponse> {
        let form = Form::new().text("mod", name.into());
        self.post("v2/mods/upload", true, |r| r.multipart(form))
            .await
    }

    pub async fn upload(&self, url: Url, path: &Path) -> Result<UploadResponse> {
        let form = Form::new().file("file", path).map_err(|e| {
            ApiError::new(
                ApiErrorKind::ImageIo,
                format!("Could not read mod file {:?}", e),
                None,
            )
        })?;

        self.send(self.client.post(url).multipart(form), false)
            .await
    }

    pub async fn edit_details(&self, data: ModDetailsRequest) -> Result<ModDetailsResponse> {
        let container: FormContainer<Form> = data.into();
        let form = container.into_inner();
        self.post("v2/mods/edit_details", true, |r| r.multipart(form))
            .await
    }

    pub async fn add_image<T: Into<String>>(&self, name: T) -> Result<ImageAddResponse> {
        self.post("v2/mods/images/add", true, |r| {
            r.multipart(Form::new().text("mod", name.into()))
        })
        .await
    }

    pub async fn upload_image(&self, url: Url, path: &Path) -> Result<ImageUploadResponse> {
        let form = Form::new().file("image", path).map_err(|e| {
            ApiError::new(
                ApiErrorKind::ImageIo,
                format!("Could not read image file: {:?}", e),
                None,
            )
        })?;

        self.send(self.client.post(url).multipart(form), false)
            .await
    }

    pub async fn edit_images(&self, data: ImageEditRequest) -> Result<ImageEditResponse> {
        let container: FormContainer<Form> = data.into();
        let form = container.into_inner();
        self.post("v2/mods/images/edit", true, |r| r.multipart(form))
            .await
    }

    pub async fn init_publish<T: Into<String>>(&self, name: T) -> Result<InitPublishResponse> {
        let form = Form::new().text("mod", name.into());
        self.post("v2/mods/init_publish", true, |r| r.multipart(form))
            .await
    }

    pub async fn publish(
        &self,
        url: Url,
        data: PublishRequest,
        path: &Path,
    ) -> Result<PublishResponse> {
        let container: FormContainer<Form> = data.into();
        let mut form = container.into_inner();
        form = form.file("file", path).map_err(|e| {
            ApiError::new(
                ApiErrorKind::ImageIo,
                format!("Could not read mod file {:?}", e),
                None,
            )
        })?;

        self.send(self.client.post(url).multipart(form), false)
            .await
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

    async fn send<T>(&self, request: RequestBuilder, auth: bool) -> Result<T>
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

        let response = request.send().await?;

        if response.status().is_success() {
            Ok(response.json::<T>().await?)
        } else {
            Err(error::from_response(response).await)
        }
    }

    async fn get<T, F>(&self, path: &str, auth: bool, f: F) -> Result<T>
    where
        T: DeserializeOwned,
        F: FnOnce(RequestBuilder) -> RequestBuilder,
    {
        let url = self.url(path)?;
        let request = f(self.client.get(url));

        self.send::<T>(request, auth).await
    }

    async fn post<T, F>(&self, path: &str, auth: bool, f: F) -> Result<T>
    where
        T: DeserializeOwned,
        F: FnOnce(RequestBuilder) -> RequestBuilder,
    {
        let url = self.url(path)?;
        let request = f(self.client.post(url));

        self.send::<T>(request, auth).await
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct ApiClientBuilder {
    client: Option<reqwest::Client>,
    base_url: Option<Url>,
    api_key: Option<String>,
}

impl ApiClientBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn client(&mut self, client: reqwest::Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    pub fn base_url<T: Into<Url>>(&mut self, base_url: T) -> &mut Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn api_key<T: Into<String>>(&mut self, api_key: T) -> &mut Self {
        self.api_key = Some(api_key.into());
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
