use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
    sync::OnceLock,
};

use regex::Regex;

use super::{error::ParseDependencyError, version::VersionReq};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DependencyMode {
    Required,
    Optional { hidden: bool },
    Independent,
}

impl Display for DependencyMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use DependencyMode::*;
        match self {
            Required => write!(f, ""),
            Optional { hidden } => write!(f, "{}", if *hidden { "(?)" } else { "?" }),
            Independent => write!(f, "~"),
        }
    }
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

impl FromStr for Dependency {
    type Err = ParseDependencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| {
            Regex::new(
                r"(?sx)
                \A\s*
                (?<mode>!|\?|\(\?\)|~)?\s*
                (?<name>[a-zA-Z0-9\-_\ ]+?)\s*
                (?<version_spec>
                    (?: < | <= | = | >= | > )\s*
                    \d+\.\d+\.\d+
                )?\s*\z",
            )
            .unwrap()
        });

        let captures = re
            .captures(s)
            .ok_or(ParseDependencyError::RegexMismatch(s.to_string()))?;

        let name = captures.name("name").unwrap().as_str().to_string();
        let version_req = captures
            .name("version_spec")
            .map_or(VersionReq::Latest, |m| {
                // Given that the regex succeeded, we know it's safe to `unwrap` here
                VersionReq::parse(m.as_str()).unwrap()
            });

        let compat = captures.name("mode").map_or(
            Compatibility::Compatible(DependencyMode::Required, version_req),
            |m| match m.as_str() {
                "!" => Compatibility::Incompatible,
                "?" => Compatibility::Compatible(
                    DependencyMode::Optional { hidden: false },
                    version_req,
                ),
                "(?)" => Compatibility::Compatible(
                    DependencyMode::Optional { hidden: true },
                    version_req,
                ),
                "~" => Compatibility::Compatible(DependencyMode::Independent, version_req),
                _ => unreachable!(),
            },
        );

        Ok(Self::new(name, compat))
    }
}

impl Display for Dependency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Compatibility::*;
        match &self.compatibility {
            Compatible(m, r) => {
                if *m != DependencyMode::Required {
                    write!(f, "{} ", m)?;
                }

                match r {
                    VersionReq::Latest => f.write_str(&self.name),
                    VersionReq::Spec(spec) => write!(f, "{} {}", self.name, spec),
                }
            }
            Incompatible => write!(f, "! {}", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_required_versioned_dep() {
        let s = "boblibrary >= 0.17.0";
        let d: Dependency = s.parse().unwrap();
        assert_eq!(
            d,
            Dependency::new(
                "boblibrary".to_string(),
                Compatibility::Compatible(
                    DependencyMode::Required,
                    VersionReq::parse(">= 0.17.0").unwrap()
                )
            )
        );
    }

    #[test]
    fn parse_required_unversioned_dep() {
        let s = "boblibrary";
        let d: Dependency = s.parse().unwrap();
        assert_eq!(
            d,
            Dependency::new(
                "boblibrary".to_string(),
                Compatibility::Compatible(DependencyMode::Required, VersionReq::Latest)
            )
        );
    }

    #[test]
    fn parse_optional_versioned_dep() {
        let s = "? boblibrary >= 0.17.0";
        let d: Dependency = s.parse().unwrap();
        assert_eq!(
            d,
            Dependency::new(
                "boblibrary".to_string(),
                Compatibility::Compatible(
                    DependencyMode::Optional { hidden: false },
                    VersionReq::parse(">= 0.17.0").unwrap()
                )
            )
        );
    }

    #[test]
    fn parse_optional_unversioned_dep() {
        let s = "? boblibrary";
        let d: Dependency = s.parse().unwrap();
        assert_eq!(
            d,
            Dependency::new(
                "boblibrary".to_string(),
                Compatibility::Compatible(
                    DependencyMode::Optional { hidden: false },
                    VersionReq::Latest
                )
            )
        );
    }

    #[test]
    fn parse_hidden_optional_versioned_dep() {
        let s = "(?) boblibrary >= 0.17.0";
        let d: Dependency = s.parse().unwrap();
        assert_eq!(
            d,
            Dependency::new(
                "boblibrary".to_string(),
                Compatibility::Compatible(
                    DependencyMode::Optional { hidden: true },
                    VersionReq::parse(">= 0.17.0").unwrap()
                )
            )
        );
    }

    #[test]
    fn parse_hidden_optional_unversioned_dep() {
        let s = "(?) boblibrary";
        let d: Dependency = s.parse().unwrap();
        assert_eq!(
            d,
            Dependency::new(
                "boblibrary".to_string(),
                Compatibility::Compatible(
                    DependencyMode::Optional { hidden: true },
                    VersionReq::Latest
                )
            )
        );
    }

    #[test]
    fn parse_incompatible_dep() {
        let s = "! boblibrary";
        let d: Dependency = s.parse().unwrap();
        assert_eq!(
            d,
            Dependency::new("boblibrary".to_string(), Compatibility::Incompatible)
        );
    }

    #[test]
    fn parse_independent_versioned_dep() {
        let s = "~ boblibrary >= 0.17.0";
        let d: Dependency = s.parse().unwrap();
        assert_eq!(
            d,
            Dependency::new(
                "boblibrary".to_string(),
                Compatibility::Compatible(
                    DependencyMode::Independent,
                    VersionReq::parse(">= 0.17.0").unwrap()
                )
            )
        );
    }

    #[test]
    fn parse_independent_unversioned_dep() {
        let s = "~ boblibrary";
        let d: Dependency = s.parse().unwrap();
        assert_eq!(
            d,
            Dependency::new(
                "boblibrary".to_string(),
                Compatibility::Compatible(DependencyMode::Independent, VersionReq::Latest)
            )
        );
    }
}
