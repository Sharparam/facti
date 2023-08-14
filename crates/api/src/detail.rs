use std::fmt::Display;

use serde::{Deserialize, Serialize, Serializer};
use strum::Display;
use url::Url;

/// Describes a request to modify details for a mod.
///
/// The `name` field is required to identify the mod to change,
/// all other fields can be set to modify that particular property of the mod.
///
/// [You can find more information about the Mod details structure on the Factorio wiki.](https://wiki.factorio.com/Mod_details_API)
#[derive(Clone, Debug, Serialize)]
pub struct ModDetailsRequest {
    /// Internal name of the mod whose details are to be changed.
    #[serde(rename = "mod")]
    pub name: String,

    /// Display name of the mod.
    ///
    /// Min length 1, max length 250.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Short description of the mod.
    ///
    /// Max length 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Long description of the mod in markdown format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Mod category.
    ///
    /// [You can find more information about mod categories on the Factorio wiki.](https://wiki.factorio.com/Mod_details_API#Category)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,

    /// Mod tags.
    ///
    /// [You can find more information about mod tags on the Factorio wiki.](https://wiki.factorio.com/Mod_details_API#Tags)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    /// Mod license.
    ///
    /// [You can find more information about mod licenses on the Factorio wiki.](https://wiki.factorio.com/Mod_details_API#License)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    /// The mod's homepage.
    ///
    /// Must use the `http` or `https` scheme, max length 256.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Url>,

    /// Whether the mod is deprecated.
    ///
    /// Deprecated mods will not show up in public listings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// URL of the mod's source code repository.
    ///
    /// Must use the `http` or `https` scheme, max length 256.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Url>,

    /// FAQ for the mod in markdown format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faq: Option<String>,
}

/// Builder for [`ModDetailsRequest`].
pub struct ModDetailsRequestBuilder {
    /// The request being built.
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

    /// Builds a finished [`ModDetailsRequest`] from the properties set
    /// on this builder.
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

/// The response returned by the Factorio API on successful modification of
/// mod details.
#[derive(Debug, Deserialize)]
pub struct ModDetailsResponse {
    /// Always has the value `true`.
    ///
    /// Factorio's API includes this property for unknown reasons.
    /// It is only present on successful API calls, and as such will always
    /// have the value `true`.
    pub success: bool,

    /// Relative path to use for obtaining information about the updated mod.
    ///
    /// This is not very useful when using this crate, as you can simply
    /// use the API client to get the updated mod details instead.
    ///
    /// Use [`ApiClient::info_full`][crate::ApiClient::info_full] to get the updated mod details.
    #[serde(rename = "url")]
    pub path: String,
}

/// Categories a mod can belong to.
///
/// A mod can only have a single category assigned.
#[derive(Default, Copy, Clone, Display, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Category {
    #[default]
    #[serde(rename = "no-category", alias = "")]
    #[strum(serialize = "no-category")]
    None,

    /// Mods introducing new content into the game.
    Content,

    /// Large total conversion mods.
    Overhaul,

    /// Small changes concerning balance, gameplay, or graphics.
    Tweaks,

    /// Providing the player with new tools or adjusting the game interface,
    /// without fundamentally changing gameplay.
    Utilities,

    /// Scenarios, maps, and puzzles.
    Scenarios,

    /// Collections of mods with tweaks to make them work together.
    ModPacks,

    /// Translations for other mods.
    Localizations,

    /// Lua libraries for use by other mods and submods that are parts of a
    /// larger mod.
    Internal,
}

/// Tags a mod can have.
///
/// Mods can have several tags assigned.
#[derive(Copy, Clone, Display, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Tag {
    /// Transportation of the player, be it vehicles or teleporters.
    Transportation,

    /// Augmented or new ways of transporting materials - belts, inserters, pipes!
    Logistics,

    /// Trains are great, but what if they could do even more?
    Trains,

    /// New ways to deal with enemies, be it attack or defense.
    Combat,

    /// Armors or armor equipment.
    Armor,

    /// Changes to enemies or entirely new enemies to deal with.
    Enemies,

    /// Map generation and terrain modification.
    Environment,

    /// New ores and resources as well as machines.
    Mining,

    /// Things related to oil and other fluids.
    Fluids,

    /// Related to roboports and logistic robots.
    LogisticNetwork,

    /// Entities which interact with the circuit network.
    CircuitNetwork,

    /// Furnaces, assembling machines, production chains.
    Manufacturing,

    /// Changes to power production and distribution.
    Power,

    /// More than just chests.
    Storage,

    /// Change blueprints behavior.
    Blueprints,

    /// Play it your way.
    Cheats,
}

/// Licenses a mod can use.
///
/// Any other license is also possible to use by way of the `Custom` variant.
///
/// If using the `Custom` variant, supply the identifier of the license in lowercase.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum License {
    /// A permissive license that is short and to the point.
    /// It lets people do anything with your code with proper attribution and without warranty.
    ///
    /// [MIT license](https://opensource.org/licenses/MIT)
    #[serde(rename = "default_mit")]
    MIT,

    /// The GNU GPL is the most widely used free software license and has a
    /// strong copyleft requirement.
    /// When distributing derived works, the source code of the work must be
    /// made available under the same license.
    ///
    /// [GNU GPLv3 license](https://opensource.org/licenses/gpl-3.0)
    #[serde(rename = "default_gnugplv3")]
    GPLv3,

    /// Version 3 of the GNU LGPL is an additional set of permissions to the
    /// GNU GPLv3 license that requires that derived works be licensed under
    /// the same license, but works that only link to it do not fall under this
    /// restriction.
    ///
    /// [GNU LGPLv3 license](https://opensource.org/licenses/lgpl-3.0)
    #[serde(rename = "default_gnulgplv3")]
    LGPLv3,

    /// The Mozilla Public License (MPL 2.0) is maintained by the Mozilla foundation.
    /// This license attempts to be a compromise between the permissive BSD
    /// license and the reciprocal GPL license.
    ///
    /// [Mozilla Public License 2.0](https://opensource.org/licenses/mpl-2.0)
    #[serde(rename = "default_mozilla2")]
    MPL2,

    /// A permissive license that also provides an express grant of patent
    /// rights from contributors to users.
    ///
    /// [Apache License 2.0](https://opensource.org/licenses/apache-2.0)
    #[serde(rename = "default_apache2")]
    Apache2,

    /// Because copyright is automatic in most countries, the Unlicense is a
    /// template to waive copyright interest in software you've written and
    /// dedicate it to the public domain. Use the Unlicense to opt out of
    /// copyright entirely.
    /// It also includes the no-warranty statement from the MIT/X11 license.
    ///
    /// [The Unlicense](https://unlicense.org/)
    #[serde(rename = "default_unlicense")]
    Unlicense,

    /// Custom license.
    ///
    /// The ID can be taken from the edit URL on the ["My licenses"][my-licenses]
    /// page on the mod portal.
    ///
    /// `mods.factorio.com/licenses/edit/$ID`
    ///
    /// [my-licenses]: https://mods.factorio.com/licenses
    #[serde(
        untagged,
        serialize_with = "custom_license_serialize",
        deserialize_with = "custom_license_deserialize"
    )]
    Custom(String),
}

impl License {
    /// Helper function to create a custom license.
    pub fn custom<T: Into<String>>(id: T) -> Self {
        License::Custom(id.into())
    }
}

/// Custom license serializer.
///
/// Used to properly serialize [`License::Custom`].
fn custom_license_serialize<S: Serializer>(id: &String, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(format!("custom_{}", id).as_str())
}

/// Custom license deserializer.
///
/// Used to properly deserialize [`License::Custom`].
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
