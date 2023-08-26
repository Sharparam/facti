use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use url::Url;

use super::{dependency::Dependency, version::Version, FactorioVersion};

/// The info.json file identifies the mod and defines its version.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModInfo {
    /// The internal name of mod.
    ///
    /// The game accepts anything as a mod name, however the mod portal
    /// restricts mod names to only consist of alphanumeric characters,
    /// dashes and underscores. Note that the mod folder or mod zip file
    /// name has to contain the mod name, where the restrictions of the
    /// file system apply.
    ///
    /// The game accepts mod names with a maximum length of 100 characters.
    /// The mod portal only accepts mods with names that are longer than
    /// 3 characters and shorter than 50 characters.
    pub name: String,

    /// Defines the version of the mod.
    pub version: Version,

    /// The display name of the mod, so it is not recommended to use
    /// someUgly_pRoGrAmMeR-name here.
    ///
    /// Can be overwritten with a locale entry in the `mod-name` category,
    /// using the internal mod name as the key.
    ///
    /// The game will reject a title field that is longer than 100 characters.
    /// However, this can be worked around by using the locale entry.
    /// The mod portal does not restrict mod title length.
    pub title: String,

    /// The author of the mod.
    ///
    /// This field does not have restrictions, it can also be a list of
    /// authors etc. The mod portal ignores this field, it will simply display
    /// the uploader's name as the author.
    pub author: String,

    /// How the mod author can be contacted, for example an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,

    /// Where the mod can be found on the internet.
    ///
    /// Note that the in-game mod browser shows the mod portal link additionally
    /// to this field. Please don't put the string `"None"` here,
    /// it makes the field on the mod portal website look ugly.
    /// Just leave the field empty if the mod doesn't have
    /// a website/forum thread/discord.
    ///
    /// **Note:** The [`None`] variant is perfectly valid to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Url>,

    /// A short description of what your mod does.
    ///
    /// This is all that people get to see in-game.
    /// Can be overwritten with a locale entry in the `mod-description` category,
    /// using the internal mod name as the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The Factorio version that this mod supports.
    ///
    /// This can only be one Factorio version, not multiple.
    /// However, it includes all `.sub` (`.patch`) versions.
    /// While the field is optional, usually mods are developed for versions
    /// higher than the default 0.12, so the field has to be added anyway.
    ///
    /// Adding a sub (patch) part, e.g. "0.18.27" will make the mod portal
    /// reject the mod and the game act weirdly.
    /// That means this shouldn't be done;
    /// use only the major and minor components "major.minor", for example "1.0".
    ///
    /// Mods with the factorio_version "0.18" can also be loaded in 1.0
    /// and the mod portal will return them
    /// when queried for factorio_version 1.0 mods.
    #[serde(default)]
    pub factorio_version: FactorioVersion,

    /// Mods that this mod depends on or is incompatible with.
    ///
    /// If this mod depends on another, the other mod will load first,
    /// see [Data-Lifecycle][].
    /// An empty [`Vec`] allows to work around the default
    /// and have no dependencies at all.
    ///
    /// [data-lifecycle]: https://lua-api.factorio.com/latest/Data-Lifecycle.html
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,

    /// Unofficial extensions to the `info.json` format.
    ///
    /// Used by third party packaging tools (such as [Facti][]).
    ///
    /// [facti]: https://facti.rs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<ModPackageInfo>,
}

/// Contains unofficial extensions to the `info.json` format.
///
/// Used by third party packaging tools.
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModPackageInfo {
    /// The path to a file that should be used as the content for
    /// the "information" page on the mod portal.
    ///
    /// The mod portal supports markdown formatting, so it's recommended
    /// to specify the path to a markdown file here.
    ///
    /// Note that this path is considered relative to the `info.json` file
    /// (unless an absolute path is specified).
    #[serde(rename = "information", skip_serializing_if = "Option::is_none")]
    pub readme_path: Option<PathBuf>,

    /// The path to a file that should be used as the content for
    /// the "FAQ" page on the mod portal.
    ///
    /// The mod portal supports markdown formatting, so it's recommended
    /// to specify the path to a markdown file here.
    ///
    /// Note that this path is considered relative to the `info.json` file
    /// (unless an absolute path is specified).
    #[serde(rename = "faq", skip_serializing_if = "Option::is_none")]
    pub faq_path: Option<PathBuf>,

    /// Paths to images that should be displayed on the mod portal.
    ///
    /// If mod details are updated with Facti, the images will be displayed
    /// on the mod portal in the same order that they are specified in this
    /// [`Vec`].
    ///
    /// Note that these paths are considered relative to the `info.json` file
    /// (unless absolute paths are specified).
    #[serde(default, rename = "gallery", skip_serializing_if = "Vec::is_empty")]
    pub gallery_paths: Vec<PathBuf>,

    /// Summary to set on the mod portal page.
    ///
    /// Ignored if [`description_as_summary`][ModPackageInfo::description_as_summary]
    /// is set to `true`.
    pub summary: Option<String>,

    /// If `true`, will use the [`description`][ModInfo::description] field
    /// to populate the summary on the mod portal.
    ///
    /// This also means that the [`summary`][ModPackageInfo::summary] field will
    /// be ignored if this is set to `true`.
    pub description_as_summary: bool,
}

impl ModInfo {
    /// Creates a builder to more conveniently construct a [`ModInfo`] struct.
    pub fn builder<T, U, V>(name: T, version: Version, title: U, author: V) -> ModInfoBuilder
    where
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        ModInfoBuilder::new(name, version, title, author)
    }
}

impl Display for ModInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} v{} by {}", self.name, self.version, self.author)
    }
}

/// Contains an internal [`ModInfo`] value that is used to later construct a
/// finished [`ModInfo`]`.
pub struct ModInfoBuilder {
    info: ModInfo,
}

/// Contains methods to successively build up a [`ModInfo`] struct.
impl ModInfoBuilder {
    fn new<T, V, U, X>(name: T, version: V, title: U, author: X) -> Self
    where
        T: Into<String>,
        V: Into<Version>,
        U: Into<String>,
        X: Into<String>,
    {
        Self {
            info: ModInfo {
                name: name.into(),
                version: version.into(),
                title: title.into(),
                author: author.into(),
                contact: None,
                homepage: None,
                description: None,
                factorio_version: Default::default(),
                dependencies: Vec::new(),
                package: None,
            },
        }
    }

    /// Sets the [`contact`][ModInfo::contact] field.
    pub fn contact<T: Into<String>>(&mut self, contact: T) -> &mut Self {
        self.info.contact = Some(contact.into());
        self
    }

    /// Sets the [`homepage`][ModInfo::homepage] field.
    pub fn homepage(&mut self, homepage: Url) -> &mut Self {
        self.info.homepage = Some(homepage);
        self
    }

    /// Sets the [`description`][ModInfo::description] field.
    pub fn description<T: Into<String>>(&mut self, description: T) -> &mut Self {
        self.info.description = Some(description.into());
        self
    }

    /// Sets the [`factorio_version`][ModInfo::factorio_version] field.
    pub fn factorio_version(&mut self, factorio_version: FactorioVersion) -> &mut Self {
        self.info.factorio_version = factorio_version;
        self
    }

    /// Adds a dependency to [`dependencies`][ModInfo::dependencies].
    pub fn dependency(&mut self, dependency: Dependency) -> &mut Self {
        self.info.dependencies.push(dependency);
        self
    }

    /// Adds multiple dependencies at once to [`dependencies`][ModInfo::dependencies].
    pub fn dependencies(&mut self, dependencies: &[Dependency]) -> &mut Self {
        self.info.dependencies.extend_from_slice(dependencies);
        self
    }

    /// Sets a path to the readme file that should be displayed
    /// on the mod portal.
    ///
    /// The path is relative to where the `info.json` file is located.
    ///
    /// **Note:** This is an unofficial extension to the `info.json` format.
    pub fn information_path<T: Into<PathBuf>>(&mut self, path: T) -> &mut Self {
        self.info
            .package
            .get_or_insert_with(Default::default)
            .readme_path = Some(path.into());
        self
    }

    /// Sets a path to the FAQ file that should be displayed
    /// on the mod portal.
    ///
    /// The path is relative to where the `info.json` file is located.
    ///
    ///
    pub fn faq_path<T: Into<PathBuf>>(&mut self, path: T) -> &mut Self {
        self.info
            .package
            .get_or_insert_with(Default::default)
            .faq_path = Some(path.into());
        self
    }

    /// Adds a path to the collection of gallery images.
    pub fn gallery<T: Into<PathBuf>>(&mut self, gallery: T) -> &mut Self {
        self.info
            .package
            .get_or_insert_with(Default::default)
            .gallery_paths
            .push(gallery.into());
        self
    }

    /// Builds a finished [`ModInfo`] from the builder.
    pub fn build(&mut self) -> ModInfo {
        self.info.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{modinfo::ModPackageInfo, version::VersionReq};

    use super::*;

    #[test]
    fn test_builder() {
        let expected = ModInfo {
            name: "boblibrary".to_string(),
            version: Version::parse("0.17.0").unwrap(),
            title: "Bob's Library".to_string(),
            author: "Bob".to_string(),
            contact: None,
            homepage: None,
            description: None,
            factorio_version: Default::default(),
            dependencies: vec![Dependency::required(
                "angel".to_string(),
                VersionReq::Latest,
            )],
            package: Some(ModPackageInfo {
                readme_path: Some(PathBuf::from("README.md")),
                ..Default::default()
            }),
        };

        let mut builder = ModInfoBuilder::new(
            "boblibrary".to_string(),
            Version::parse("0.17.0").unwrap(),
            "Bob's Library".to_string(),
            "Bob".to_string(),
        );
        builder.dependency(Dependency::required(
            "angel".to_string(),
            VersionReq::Latest,
        ));
        builder.information_path("README.md");
        let built = builder.build();

        assert_eq!(built, expected);
    }
}
