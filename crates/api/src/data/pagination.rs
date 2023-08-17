use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::{ApiError, ApiErrorKind};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub count: u32,
    pub links: PaginationLinks,
    pub page: u32,
    pub page_count: u32,
    pub page_size: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginationLinks {
    pub first: Option<Url>,
    #[serde(rename = "prev")]
    pub previous: Option<Url>,

    pub next: Option<Url>,
    pub last: Option<Url>,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub enum PageSize {
    #[serde(rename = "max")]
    Max,

    #[serde(untagged, serialize_with = "serialize_custom_page_size")]
    Custom(u32),
}

impl FromStr for PageSize {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "max" {
            Ok(PageSize::Max)
        } else {
            s.parse::<u32>().map(PageSize::Custom).map_err(|_| {
                ApiError::new(
                    ApiErrorKind::InvalidPageSize,
                    format!("{} is not a valid page size", s),
                    None,
                )
            })
        }
    }
}

impl Display for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageSize::Max => f.write_str("max"),
            PageSize::Custom(size) => write!(f, "{}", size),
        }
    }
}

fn serialize_custom_page_size<S>(size: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u32(*size)
}
