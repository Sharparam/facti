use url::Url;

use super::{dependency::Dependency, version::Version, FactorioVersion, ModInfo};

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
