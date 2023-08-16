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

/// A blocking [`ApiClient`] to make requests to the Factorio APIs with.
///
/// The minimal client you can construct is one that does not use an API key:
///
/// ```
/// # use facti_api::blocking::ApiClient;
/// let client = ApiClient::new();
/// ```
///
/// This will let you use any APIs that do not require authentication:
///
/// - [`search`][ApiClient::search]
/// - [`info_short`][ApiClient::info_short]
/// - [`info_full`][ApiClient::info_full]
///
/// Technically, you can also use [`upload`][ApiClient::upload],
/// [`upload_image`][ApiClient::upload_image], and [`publish`][ApiClient::publish]
/// without an API key, as they instead rely on unique URLs,
/// but the only way to obtain these URLs is by calling [`init_upload`][ApiClient::init_upload],
/// [`add_image`][ApiClient::add_image], or [`init_publish`][ApiClient::init_publish], respectively,
/// which *do* require an API key.
///
/// Conversely, the methods that require an API key to use are:
///
/// - [`init_upload`][ApiClient::init_upload]
/// - [`edit_details`][ApiClient::edit_details]
/// - [`add_image`][ApiClient::add_image]
/// - [`edit_images`][ApiClient::edit_images]
/// - [`init_publish`][ApiClient::init_publish]
///
/// To construct a minimal client with an API key, simply pass it as a string
/// to the constructor:
///
/// ```
/// # use facti_api::blocking::ApiClient;
/// let client = ApiClient::with_api_key("<YOUR_API_KEY>");
/// ```
///
/// You can obtain API keys by visiting [your Factorio profile][factorio-profile].
///
/// [factorio-profile]: https://factorio.com/profile
///
/// Note that for full functionality of the API client, you will need to select
/// all permissions for your API key. If you know you will only use a limited
/// subset of the methods, you can of course limit the scope of your key.
///
/// If you want to customize the base URL or the underlying [`reqwest::blocking::Client`]
/// used, please construct an [`ApiClientBuilder`] by calling [`ApiClient::builder`],
/// where you can set (or not set) any of the client properties. Those that are
/// not set will get their default values.
///
/// The default base URL for the Factorio API can be obtained from the constant
/// [`DEFAULT_BASE_URL`].
pub struct ApiClient {
    client: reqwest::blocking::Client,
    base_url: Url,
    api_key: Option<String>,
}

type Result<T> = core::result::Result<T, ApiError>;

impl ApiClient {
    /// Constructs a new [`ApiClient`] with no API key configured.
    ///
    /// A non-mut client constructed in this manner will only be able to use
    /// APIs that do not require authentication.
    pub fn new() -> Self {
        Self {
            client: Default::default(),
            base_url: Url::parse(DEFAULT_BASE_URL).unwrap(),
            api_key: None,
        }
    }

    /// Constructs a new [`ApiClient`] with the given API key.
    pub fn with_api_key<T: Into<String>>(api_key: T) -> Self {
        Self {
            api_key: Some(api_key.into()),
            ..Default::default()
        }
    }

    /// Constructs a new [`ApiClientBuilder`], letting you customize details
    /// of all the client properties.
    pub fn builder() -> ApiClientBuilder {
        ApiClientBuilder::new()
    }

    /// Search for mods on the Factorio mod portal.
    ///
    /// The following fields in [`SearchResult`] may be set in each result
    /// in the returned [`SearchResponse::results`]:
    ///
    /// - [`download_count`][SearchResult::download_count]
    /// - [`latest_release`][SearchResult::latest_release]
    /// - [`name`][SearchResult::name]
    /// - [`owner`][SearchResult::owner]
    /// - [`releases`][SearchResult::releases]
    /// - [`summary`][SearchResult::summary]
    /// - [`title`][SearchResult::title]
    /// - [`category`][SearchResult::category]
    ///
    /// Any other fields on [`SearchResult`] will *never* have a value set when
    /// constructed as a result of calling this method.
    pub fn search(&self, query: &SearchQuery) -> Result<SearchResponse> {
        self.get("mods", false, |r| r.query(query))
    }

    /// Get brief information about a mod by its internal name.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use facti_api::blocking::ApiClient;
    /// #
    /// let client = ApiClient::new();
    /// let result = client.info_short("cybersyn-combinator")?;
    /// dbg!(result);
    /// # Ok::<(), Box<dyn Error>>(())
    /// ```
    pub fn info_short(&self, name: &str) -> Result<SearchResult> {
        self.get(&format!("mods/{}", name), false, |r| r)
    }

    /// Get detailed information about a mod by its internal name.
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
        Self::new()
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

    pub fn client(&mut self, client: reqwest::blocking::Client) -> &mut Self {
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
