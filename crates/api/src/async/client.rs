use std::path::Path;

use reqwest::{header, multipart::Form, RequestBuilder};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    data::{
        detail::{ModDetailsRequest, ModDetailsResponse},
        game::LatestReleases,
        image::{
            self, Image, ImageAddResponse, ImageEditRequest, ImageEditResponse, ImageUploadResponse,
        },
        portal::{SearchQuery, SearchResponse, SearchResult},
        publish::{InitPublishResponse, PublishRequest, PublishResponse},
        upload::{InitUploadResponse, UploadResponse},
    },
    error::{ApiError, ApiErrorKind},
    reqwest::FormContainer,
    FactorioUrls,
};

use super::{error, reqwest::AsyncFormFile};

pub struct ApiClient {
    client: reqwest::Client,
    urls: FactorioUrls,
    api_key: Option<String>,
}

type Result<T> = core::result::Result<T, ApiError>;

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Default::default(),
            urls: Default::default(),
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
        self.get(self.portal_api_url("mods")?, false, |r| r.query(query))
            .await
    }

    pub async fn info_short(&self, name: &str) -> Result<SearchResult> {
        self.get(self.portal_api_url(format!("mods/{}", name))?, false, |r| r)
            .await
    }

    /// Get detailed information about a mod by its internal name.
    pub async fn info_full(&self, name: &str) -> Result<SearchResult> {
        self.get(
            self.portal_api_url(format!("mods/{}/full", name))?,
            false,
            |r| r,
        )
        .await
    }

    pub async fn init_upload<T: Into<String>>(&self, name: T) -> Result<InitUploadResponse> {
        let form = Form::new().text("mod", name.into());
        self.post(self.portal_api_url("v2/mods/upload")?, true, |r| {
            r.multipart(form)
        })
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
        self.post(self.portal_api_url("v2/mods/edit_details")?, true, |r| {
            r.multipart(form)
        })
        .await
    }

    pub async fn get_images(&self, name: &str) -> Result<Vec<Image>> {
        let url = self.portal_url(format!("mod/{}", name))?;
        let page = self.client.get(url.to_owned()).send().await?;
        let html = page.text().await?;
        let images = image::parse_html_images(&html);

        Ok(images)
    }

    pub async fn add_image<T: Into<String>>(&self, name: T) -> Result<ImageAddResponse> {
        self.post(self.portal_api_url("v2/mods/images/add")?, true, |r| {
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
        self.post(self.portal_api_url("v2/mods/images/edit")?, true, |r| {
            r.multipart(form)
        })
        .await
    }

    pub async fn init_publish<T: Into<String>>(&self, name: T) -> Result<InitPublishResponse> {
        let form = Form::new().text("mod", name.into());
        self.post(self.portal_api_url("v2/mods/init_publish")?, true, |r| {
            r.multipart(form)
        })
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

    /// Get information about the latest available releases of the game.
    pub async fn latest_releases(&self) -> Result<LatestReleases> {
        self.get(self.game_url("latest-releases")?, false, |r| r)
            .await
    }

    fn portal_api_url<T: AsRef<str>>(&self, path: T) -> Result<Url> {
        self.urls.portal_api(path.as_ref()).map_err(|_| {
            ApiError::new(
                ApiErrorKind::UrlParseFailed,
                format!("Failed to join base URL with path {}", path.as_ref()),
                None,
            )
        })
    }

    fn portal_url<T: AsRef<str>>(&self, path: T) -> Result<Url> {
        self.urls.portal(path.as_ref()).map_err(|_| {
            ApiError::new(
                ApiErrorKind::UrlParseFailed,
                format!("Failed to join portal base URL with path {}", path.as_ref()),
                None,
            )
        })
    }

    fn game_url(&self, path: &str) -> Result<Url> {
        self.urls.game(path).map_err(|_| {
            ApiError::new(
                ApiErrorKind::UrlParseFailed,
                format!("Failed to join game base URL with path {}", path),
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

    async fn get<T, U, F>(&self, url: U, auth: bool, f: F) -> Result<T>
    where
        T: DeserializeOwned,
        U: Into<Url>,
        F: FnOnce(RequestBuilder) -> RequestBuilder,
    {
        let request = f(self.client.get(url.into()));

        self.send::<T>(request, auth).await
    }

    async fn post<T, U, F>(&self, url: U, auth: bool, f: F) -> Result<T>
    where
        T: DeserializeOwned,
        U: Into<Url>,
        F: FnOnce(RequestBuilder) -> RequestBuilder,
    {
        let request = f(self.client.post(url.into()));

        self.send::<T>(request, auth).await
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

api_client_builder!(reqwest::Client, ApiClient);
