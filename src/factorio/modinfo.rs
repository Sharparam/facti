use semver::{Version, VersionReq};

use super::FactorioVersion;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DependencyMode {
    Required,
    Optional { hidden: bool },
    Independent,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Compatibility {
    Compatible(VersionReq, DependencyMode),
    Incompatible,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub compatibility: Compatibility,
}

impl Dependency {
    pub fn new(name: String, compatibility: Compatibility) -> Self {
        Self {
            name,
            compatibility,
        }
    }

    pub fn required(name: String, version_req: VersionReq) -> Self {
        Self::new(
            name,
            Compatibility::Compatible(version_req, DependencyMode::Required),
        )
    }

    pub fn optional(name: String, version_req: VersionReq, hidden: bool) -> Self {
        Self::new(
            name,
            Compatibility::Compatible(version_req, DependencyMode::Optional { hidden }),
        )
    }

    pub fn independent(name: String, version_req: VersionReq) -> Self {
        Self::new(
            name,
            Compatibility::Compatible(version_req, DependencyMode::Independent),
        )
    }

    pub fn incompatible(name: String) -> Self {
        Self::new(name, Compatibility::Incompatible)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModInfo {
    pub name: String,
    pub version: Version,
    pub title: String,
    pub author: String,
    pub contact: Option<String>,
    pub homepage: Option<String>,
    pub description: Option<String>,
    pub factorio_version: FactorioVersion,
    pub dependencies: Vec<Dependency>,
}
