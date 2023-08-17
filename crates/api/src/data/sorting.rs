use std::str::FromStr;

use serde::Serialize;
use strum::Display;

use crate::error::{ApiError, ApiErrorKind};

#[derive(Copy, Clone, Display, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortMode {
    Name,
    CreatedAt,
    UpdatedAt,
}

impl FromStr for SortMode {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("name") => Ok(SortMode::Name),
            s if s.eq_ignore_ascii_case("created_at")
                || s.eq_ignore_ascii_case("createdat")
                || s.eq_ignore_ascii_case("created") =>
            {
                Ok(SortMode::CreatedAt)
            }
            s if s.eq_ignore_ascii_case("updated_at")
                || s.eq_ignore_ascii_case("updatedat")
                || s.eq_ignore_ascii_case("updated") =>
            {
                Ok(SortMode::UpdatedAt)
            }
            _ => Err(ApiError::new(
                ApiErrorKind::InvalidSortMode,
                format!("{} is not a valid sort mode", s),
                None,
            )),
        }
    }
}

#[derive(Default, Copy, Clone, Display, Debug, Serialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,

    #[serde(rename = "desc")]
    #[default]
    Descending,
}

impl FromStr for SortOrder {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("asc") || s.eq_ignore_ascii_case("ascending") => {
                Ok(SortOrder::Ascending)
            }
            s if s.eq_ignore_ascii_case("desc") || s.eq_ignore_ascii_case("descending") => {
                Ok(SortOrder::Descending)
            }
            _ => Err(ApiError::new(
                ApiErrorKind::InvalidSortOrder,
                format!("{} is not a valid sort order", s),
                None,
            )),
        }
    }
}
