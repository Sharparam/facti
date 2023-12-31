use std::str::FromStr;

use strum::Display;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{kind}")]
pub struct ApiError {
    kind: ApiErrorKind,
    message: String,

    #[source]
    source: Option<reqwest::Error>,
}

#[derive(Display, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ApiErrorKind {
    #[strum(to_string = "Missing or invalid API key for the current endpoint")]
    InvalidApiKey,

    #[strum(to_string = "Invalid request")]
    InvalidRequest,

    #[strum(to_string = "Internal error, please try again later")]
    InternalError,

    #[strum(to_string = "Insufficient permission for current endpoint")]
    Forbidden,

    #[strum(to_string = "Invalid release data in info.json")]
    InvalidModRelease,

    #[strum(to_string = "Invalid mod data in zip file")]
    InvalidModUpload,

    #[strum(to_string = "Invalid image file uploaded")]
    InvalidImageUpload,

    #[strum(to_string = "Mod does not exist in mod portal")]
    UnknownMod,

    #[strum(to_string = "API key has not been set")]
    MissingApiKey,

    #[strum(to_string = "Failed to parse URL")]
    UrlParseFailed,

    #[strum(to_string = "Failed to deserialize response")]
    DeserializationFailed,

    #[strum(to_string = "Failed to read the image file")]
    ImageIo,

    #[strum(to_string = "Invalid page size, must be a number or 'max'")]
    InvalidPageSize,

    #[strum(to_string = "Invalid sort mode, must be name, created_at, or updated_at")]
    InvalidSortMode,

    #[strum(to_string = "Invalid sort order, must be 'asc(ending)' or 'desc(ending)'")]
    InvalidSortOrder,

    #[strum(to_string = "Unknown error, please try again later")]
    Unknown,
}

impl ApiError {
    pub fn new<T: Into<String>>(
        kind: ApiErrorKind,
        message: T,
        source: Option<reqwest::Error>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            source,
        }
    }

    pub fn kind(&self) -> ApiErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl ApiErrorKind {
    pub fn parse<T: Into<String>>(s: T) -> ApiErrorKind {
        // `unwrap` is safe, the default case of our `from_str` is to return
        // `ApiErrorKind::Unknown`
        s.into().parse().unwrap()
    }
}

// async fn from_response(response: reqwest::Response) -> ApiError {
//     #[derive(Debug, Deserialize)]
//     struct ApiErrorResponse {
//         error: String,
//         message: String,
//     }

//     let source = match response.error_for_status_ref() {
//         Ok(_) => None,
//         Err(e) => Some(e),
//     };

//     if let Ok(error_response) = response.json::<ApiErrorResponse>().await {
//         ApiError::new(
//             ApiErrorKind::parse(error_response.error),
//             error_response.message,
//             source,
//         )
//     } else {
//         ApiError::new(
//             ApiErrorKind::Unknown,
//             "Failed to parse error response",
//             source,
//         )
//     }
// }

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self::new(ApiErrorKind::Unknown, "Unknown error", Some(error))
    }
}

impl FromStr for ApiErrorKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ApiErrorKind::*;

        Ok(match s {
            "InvalidApiKey" => InvalidApiKey,
            "InvalidRequest" => InvalidRequest,
            "InternalError" => InternalError,
            "Forbidden" => Forbidden,
            "InvalidModRelease" => InvalidModRelease,
            "InvalidModUpload" => InvalidModUpload,
            "InvalidImageUpload" => InvalidImageUpload,
            "UnknownMod" => UnknownMod,
            "Unknown" => Unknown,
            _ => Unknown,
        })
    }
}
