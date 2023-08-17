use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use facti_lib::{dependency::Dependency, version::Version, FactorioVersion};

use super::{
    detail::{self, Category, Tag},
    pagination::{PageSize, Pagination},
    sorting::{SortMode, SortOrder},
};

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

    #[deprecated(note = "Use [`source_url`] instead")]
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
