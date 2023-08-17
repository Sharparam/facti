use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use url::Url;

use super::{dependency::Dependency, version::Version, FactorioVersion};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModInfo {
    pub name: String,

    pub version: Version,

    pub title: String,

    pub author: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub factorio_version: FactorioVersion,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<ModPackageInfo>,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModPackageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub faq: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gallery: Vec<String>,
}

impl ModInfo {
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

pub struct ModInfoBuilder {
    info: ModInfo,
}

impl ModInfoBuilder {
    pub(crate) fn new<T, V, U, X>(name: T, version: V, title: U, author: X) -> Self
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

    pub fn contact<T: Into<String>>(&mut self, contact: T) -> &mut Self {
        self.info.contact = Some(contact.into());
        self
    }

    pub fn homepage(&mut self, homepage: Url) -> &mut Self {
        self.info.homepage = Some(homepage);
        self
    }

    pub fn description<T: Into<String>>(&mut self, description: T) -> &mut Self {
        self.info.description = Some(description.into());
        self
    }

    pub fn factorio_version(&mut self, factorio_version: FactorioVersion) -> &mut Self {
        self.info.factorio_version = factorio_version;
        self
    }

    pub fn dependency(&mut self, dependency: Dependency) -> &mut Self {
        self.info.dependencies.push(dependency);
        self
    }

    pub fn dependencies(&mut self, dependencies: &[Dependency]) -> &mut Self {
        self.info.dependencies.extend_from_slice(dependencies);
        self
    }

    pub fn readme<T: Into<String>>(&mut self, readme: T) -> &mut Self {
        self.info
            .package
            .get_or_insert_with(Default::default)
            .readme = Some(readme.into());
        self
    }

    pub fn faq<T: Into<String>>(&mut self, faq: T) -> &mut Self {
        self.info.package.get_or_insert_with(Default::default).faq = Some(faq.into());
        self
    }

    pub fn gallery<T: Into<String>>(&mut self, gallery: T) -> &mut Self {
        self.info
            .package
            .get_or_insert_with(Default::default)
            .gallery
            .push(gallery.into());
        self
    }

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
                readme: Some("README.md".to_string()),
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
        builder.readme("README.md");
        let built = builder.build();

        assert_eq!(built, expected);
    }
}
