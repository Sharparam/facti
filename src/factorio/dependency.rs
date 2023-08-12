use super::{parse::ParseDependencyError, version::VersionReq};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DependencyMode {
    Required,
    Optional { hidden: bool },
    Independent,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Compatibility {
    Compatible(DependencyMode, VersionReq),
    Incompatible,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub compatibility: Compatibility,
}

impl Dependency {
    pub fn new<T: Into<String>>(name: T, compatibility: Compatibility) -> Self {
        Self {
            name: name.into(),
            compatibility,
        }
    }

    pub fn required<T: Into<String>>(name: T, version_req: VersionReq) -> Self {
        Self::new(
            name,
            Compatibility::Compatible(DependencyMode::Required, version_req),
        )
    }

    pub fn optional<T: Into<String>>(name: T, version_req: VersionReq, hidden: bool) -> Self {
        Self::new(
            name,
            Compatibility::Compatible(DependencyMode::Optional { hidden }, version_req),
        )
    }

    pub fn independent<T: Into<String>>(name: T, version_req: VersionReq) -> Self {
        Self::new(
            name,
            Compatibility::Compatible(DependencyMode::Independent, version_req),
        )
    }

    pub fn incompatible<T: Into<String>>(name: T) -> Self {
        Self::new(name, Compatibility::Incompatible)
    }

    pub fn parse(s: &str) -> Result<Self, ParseDependencyError> {
        s.parse()
    }
}
