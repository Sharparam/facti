use std::{str::FromStr, sync::OnceLock};

use regex::Regex;

use super::{
    dependency::{Compatibility, Dependency, DependencyMode},
    error::{DependencyParseError, VersionParseError},
    version::{Version, VersionReq, VersionSpec},
    FactorioVersion,
};

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('.').collect::<Vec<_>>();

        if parts.len() != 3 {
            return Err(VersionParseError::InvalidSize(3, parts.len()));
        }

        let major = parts[0].parse().map_err(VersionParseError::InvalidMajor)?;
        let minor = parts[1].parse().map_err(VersionParseError::InvalidMinor)?;
        let patch = parts[2].parse().map_err(VersionParseError::InvalidPatch)?;

        Ok(Version::new(major, minor, patch))
    }
}

impl FromStr for FactorioVersion {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('.').collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(VersionParseError::InvalidSize(2, parts.len()));
        }

        let major = parts[0].parse().map_err(VersionParseError::InvalidMajor)?;
        let minor = parts[1].parse().map_err(VersionParseError::InvalidMinor)?;

        Ok(FactorioVersion::new(major, minor))
    }
}

impl FromStr for VersionReq {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Ok(VersionReq::Latest);
        }

        let spec: VersionSpec = trimmed.parse()?;

        Ok(VersionReq::Spec(spec))
    }
}

impl FromStr for VersionSpec {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let semver_req = semver::VersionReq::parse(s).map_err(VersionParseError::InvalidSemver)?;
        semver_req.try_into()
    }
}

impl FromStr for Dependency {
    type Err = DependencyParseError;

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
            .ok_or(DependencyParseError::RegexMismatch(s.to_string()))?;

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

#[cfg(test)]
mod tests {
    use crate::factorio::dependency::{Compatibility, DependencyMode};

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
