use serde::Deserialize;

use crate::error::{ApiError, ApiErrorKind};

#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    error: String,
    message: String,
}

/// Convenience function to convert [`reqwest::Response`] to a nice
/// and properly set up [`ApiError`].
pub(crate) async fn from_response(response: reqwest::Response) -> ApiError {
    let source = match response.error_for_status_ref() {
        Ok(_) => None,
        Err(e) => Some(e),
    };

    if let Ok(error_response) = response.json::<ApiErrorResponse>().await {
        ApiError::new(
            ApiErrorKind::parse(error_response.error),
            error_response.message,
            source,
        )
    } else {
        ApiError::new(
            ApiErrorKind::Unknown,
            "Failed to parse error response",
            source,
        )
    }
}
