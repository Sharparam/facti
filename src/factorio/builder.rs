use semver::Version;

use super::{modinfo::Dependency, FactorioVersion, ModInfo};

pub struct ModInfoBuilder {
    info: ModInfo,
}

impl ModInfoBuilder {
    pub fn new(name: String, version: Version, title: String, author: String) -> Self {
        Self {
            info: ModInfo {
                name,
                version,
                title,
                author,
                contact: None,
                homepage: None,
                description: None,
                factorio_version: FactorioVersion::new(0, 12),
                dependencies: Vec::new(),
            },
        }
    }

    pub fn contact(&mut self, contact: String) -> &mut Self {
        self.info.contact = Some(contact);
        self
    }

    pub fn homepage(&mut self, homepage: String) -> &mut Self {
        self.info.homepage = Some(homepage);
        self
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.info.description = Some(description);
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

    pub fn build(&self) -> ModInfo {
        self.info.clone()
    }
}

#[cfg(test)]
mod tests {
    use semver::VersionReq;

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
            factorio_version: FactorioVersion::new(0, 12),
            dependencies: vec![Dependency::required("angel".to_string(), VersionReq::STAR)],
        };

        let mut builder = ModInfoBuilder::new(
            "boblibrary".to_string(),
            Version::parse("0.17.0").unwrap(),
            "Bob's Library".to_string(),
            "Bob".to_string(),
        );
        builder.dependency(Dependency::required("angel".to_string(), VersionReq::STAR));
        let built = builder.build();

        assert_eq!(built, expected);
    }
}
