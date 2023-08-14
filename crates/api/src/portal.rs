use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::Display;
use url::Url;

use facti_lib::{dependency::Dependency, version::Version, FactorioVersion};

use crate::error::{ApiError, ApiErrorKind};

use super::detail::{self, Category, Tag};

#[derive(Debug, Serialize)]
pub struct SearchQuery {
    pub hide_deprecated: bool,
    pub page: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<PageSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_namelist"
    )]
    pub namelist: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<FactorioVersion>,
}

fn serialize_namelist<S>(namelist: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(namelist) = namelist {
        serializer.serialize_str(namelist.join(",").as_str())
    } else {
        serializer.serialize_none()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub pagination: Pagination,
    pub results: Vec<SearchResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "downloads_count")]
    pub download_count: u32,

    pub latest_release: Option<Release>,
    pub name: String,
    pub owner: String,
    pub releases: Option<Vec<Release>>,
    pub summary: Option<String>,
    pub title: Option<String>,
    pub category: Option<Category>,

    #[serde(rename = "thumbnail")]
    pub thumbnail_path: Option<String>,

    pub changelog: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub source_url: Option<Url>,

    #[deprecated(note = "Use `source_url` instead")]
    pub github_path: Option<String>,

    pub homepage: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub license: Option<License>,
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;

        let version: Option<Version> = match &self.latest_release {
            Some(release) => Some(release.version),
            None => match &self.releases {
                Some(releases) if !releases.is_empty() => {
                    let mut clone = releases.to_vec();
                    clone.sort_by(|a, b| b.version.cmp(&a.version));
                    Some(clone[0].version)
                }
                _ => None,
            },
        };

        if let Some(version) = version {
            write!(f, " v{}", version)?;
        }

        write!(f, " by {}", self.owner)?;

        Ok(())
    }
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Release {
    #[serde(rename = "download_url")]
    pub download_path: String,

    #[serde(rename = "file_name")]
    pub filename: String,

    #[serde(rename = "info_json")]
    pub info: ReleaseInfo,

    pub released_at: DateTime<Utc>,
    pub version: Version,
    pub sha1: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseInfo {
    pub factorio_version: FactorioVersion,
    pub dependencies: Option<Vec<Dependency>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct License {
    pub id: detail::License,
    pub name: String,
    pub title: String,
    pub description: String,
    pub url: String,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            hide_deprecated: true,
            page: 1,
            page_size: None,
            sort: None,
            sort_order: None,
            namelist: None,
            version: None,
        }
    }
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
