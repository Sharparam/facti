use serde::Deserialize;

use crate::error::{ApiError, ApiErrorKind};

/// Convenience impl to convert [`reqwest::blocking::Response`] to a nice
/// and properly set up [`ApiError`].
#[doc(hidden)]
impl From<reqwest::blocking::Response> for ApiError {
    fn from(response: reqwest::blocking::Response) -> Self {
        #[derive(Debug, Deserialize)]
        struct ApiErrorResponse {
            error: String,
            message: String,
        }

        let source = response.error_for_status_ref().err();

        if let Ok(error_response) = response.json::<ApiErrorResponse>() {
            Self::new(
                ApiErrorKind::parse(error_response.error),
                error_response.message,
                source,
            )
        } else {
            Self::new(
                ApiErrorKind::Unknown,
                "Failed to parse error response",
                source,
            )
        }
    }
}
