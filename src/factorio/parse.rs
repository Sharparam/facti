use std::{str::FromStr, sync::OnceLock};

use regex::Regex;
use semver::VersionReq;

use super::modinfo::{Compatibility, Dependency, DependencyMode};

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDependencyError;

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

        let captures = re.captures(s).ok_or(ParseDependencyError)?;

        let name = captures.name("name").unwrap().as_str().to_string();
        let version_req = captures
            .name("version_spec")
            .map_or(VersionReq::STAR, |m| VersionReq::parse(m.as_str()).unwrap());

        let compat = captures.name("mode").map_or(
            Compatibility::Compatible(version_req.clone(), DependencyMode::Required),
            |m| match m.as_str() {
                "!" => Compatibility::Incompatible,
                "?" => Compatibility::Compatible(
                    version_req,
                    DependencyMode::Optional { hidden: false },
                ),
                "(?)" => Compatibility::Compatible(
                    version_req,
                    DependencyMode::Optional { hidden: true },
                ),
                "~" => Compatibility::Compatible(version_req, DependencyMode::Independent),
                _ => unreachable!(),
            },
        );

        Ok(Self::new(name, compat))
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
                    VersionReq::parse(">=0.17.0").unwrap(),
                    DependencyMode::Required
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
                Compatibility::Compatible(VersionReq::STAR, DependencyMode::Required)
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
                    VersionReq::parse(">= 0.17.0").unwrap(),
                    DependencyMode::Optional { hidden: false }
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
                    VersionReq::STAR,
                    DependencyMode::Optional { hidden: false }
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
                    VersionReq::parse(">= 0.17.0").unwrap(),
                    DependencyMode::Optional { hidden: true }
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
                    VersionReq::STAR,
                    DependencyMode::Optional { hidden: true }
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
                    VersionReq::parse(">= 0.17.0").unwrap(),
                    DependencyMode::Independent
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
                Compatibility::Compatible(VersionReq::STAR, DependencyMode::Independent)
            )
        );
    }
}
