use std::fmt::Display;

use serde::{Deserialize, Serialize, Serializer};
use strum::Display;
use url::Url;

#[derive(Clone, Debug, Serialize)]
pub struct ModDetailsRequest {
    #[serde(rename = "mod")]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub faq: Option<String>,
}

pub struct ModDetailsRequestBuilder {
    request: ModDetailsRequest,
}

impl ModDetailsRequest {
    pub fn builder<T: Into<String>>(name: T) -> ModDetailsRequestBuilder {
        ModDetailsRequestBuilder::new(name)
    }
}

impl From<ModDetailsRequest> for reqwest::blocking::multipart::Form {
    fn from(data: ModDetailsRequest) -> Self {
        let mut form = Self::new().text("mod", data.name);

        if let Some(title) = data.title {
            form = form.text("title", title);
        }

        if let Some(summary) = data.summary {
            form = form.text("summary", summary);
        }

        if let Some(description) = data.description {
            form = form.text("description", description);
        }

        if let Some(category) = data.category {
            form = form.text("category", category.to_string());
        }

        if let Some(tags) = data.tags {
            for tag in tags {
                form = form.text("tags", tag.to_string());
            }
        }

        if let Some(license) = data.license {
            form = form.text("license", license.to_string());
        }

        if let Some(homepage) = data.homepage {
            form = form.text("homepage", homepage.to_string());
        }

        if let Some(deprecated) = data.deprecated {
            form = form.text("deprecated", deprecated.to_string());
        }

        if let Some(source_url) = data.source_url {
            form = form.text("source_url", source_url.to_string());
        }

        if let Some(faq) = data.faq {
            form = form.text("faq", faq);
        }

        form
    }
}

impl ModDetailsRequestBuilder {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            request: ModDetailsRequest {
                name: name.into(),
                title: None,
                summary: None,
                description: None,
                category: None,
                tags: None,
                license: None,
                homepage: None,
                deprecated: None,
                source_url: None,
                faq: None,
            },
        }
    }

    pub fn build(&mut self) -> ModDetailsRequest {
        self.request.clone()
    }

    pub fn title<T: Into<String>>(&mut self, title: T) -> &mut Self {
        self.request.title = Some(title.into());
        self
    }

    pub fn summary<T: Into<String>>(&mut self, summary: T) -> &mut Self {
        self.request.summary = Some(summary.into());
        self
    }

    pub fn description<T: Into<String>>(&mut self, description: T) -> &mut Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn category(&mut self, category: Category) -> &mut Self {
        self.request.category = Some(category);
        self
    }

    pub fn tag(&mut self, tag: Tag) -> &mut Self {
        self.request
            .tags
            .get_or_insert_with(Default::default)
            .push(tag);
        self
    }

    pub fn license(&mut self, license: License) -> &mut Self {
        self.request.license = Some(license);
        self
    }

    pub fn homepage(&mut self, homepage: Url) -> &mut Self {
        self.request.homepage = Some(homepage);
        self
    }

    pub fn deprecated(&mut self, deprecated: bool) -> &mut Self {
        self.request.deprecated = Some(deprecated);
        self
    }

    pub fn source_url(&mut self, source_url: Url) -> &mut Self {
        self.request.source_url = Some(source_url);
        self
    }

    pub fn faq<T: Into<String>>(&mut self, faq: T) -> &mut Self {
        self.request.faq = Some(faq.into());
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct ModDetailsResponse {
    pub success: bool,

    #[serde(rename = "url")]
    pub path: String,
}

#[derive(Copy, Clone, Display, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Category {
    #[serde(rename = "no-category")]
    #[strum(serialize = "no-category")]
    None,
    Content,
    Overhaul,
    Tweaks,
    Utilities,
    Scenarios,
    ModPacks,
    Localizations,
    Internal,
}

#[derive(Copy, Clone, Display, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Tag {
    Transportation,
    Logistics,
    Trains,
    Combat,
    Armor,
    Enemies,
    Environment,
    Mining,
    Fluids,
    LogisticNetwork,
    CircuitNetwork,
    Manufacturing,
    Power,
    Storage,
    Blueprints,
    Cheats,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum License {
    #[serde(rename = "default_mit")]
    MIT,

    #[serde(rename = "default_gnugplv3")]
    GPLv3,

    #[serde(rename = "default_gnulgplv3")]
    LGPLv3,

    #[serde(rename = "default_mozilla2")]
    MPL2,

    #[serde(rename = "default_apache2")]
    Apache2,

    #[serde(rename = "default_unlicense")]
    Unlicense,
    #[serde(
        untagged,
        serialize_with = "custom_license_serialize",
        deserialize_with = "custom_license_deserialize"
    )]
    Custom(String),
}

impl License {
    pub fn custom<T: Into<String>>(id: T) -> Self {
        License::Custom(id.into())
    }
}

fn custom_license_serialize<S: Serializer>(id: &String, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(format!("custom_{}", id).as_str())
}

fn custom_license_deserialize<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    let s = String::deserialize(deserializer)?;

    if let Some(custom) = s.strip_prefix("custom_") {
        Ok(custom.to_string())
    } else {
        Err(serde::de::Error::custom(format!(
            "invalid custom license: {}",
            s
        )))
    }
}

impl Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            License::MIT => write!(f, "default_mit"),
            License::GPLv3 => write!(f, "default_gnugplv3"),
            License::LGPLv3 => write!(f, "default_gnulgplv3"),
            License::MPL2 => write!(f, "default_mozilla2"),
            License::Apache2 => write!(f, "default_apache2"),
            License::Unlicense => write!(f, "default_unlicense"),
            License::Custom(id) => write!(f, "custom_{}", id),
        }
    }
}
