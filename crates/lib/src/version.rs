use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::error::{ParseVersionReqError, ParseVersionSpecError};

use super::error::ParseVersionError;

mod factorio_version;

pub use factorio_version::FactorioVersion;

/// Represents a mod's version, in (limited) semver format.
///
/// # Examples
///
/// ```
/// use facti_lib::version::Version;
///
/// let my_version = Version { major: 1, minor: 2, patch: 3 };
///
/// println!("My version is: {}", my_version);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Version {
    /// The major part of the version number.
    pub major: u64,

    /// The minor part of the version number.
    pub minor: u64,

    /// The patch part of the version number.
    ///
    /// Factorio documentation and resources sometimes refer to this field
    /// as the "sub" part.
    pub patch: u64,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parses a version string into a [`Version`].
    pub fn parse(s: &str) -> Result<Self, ParseVersionError> {
        s.parse()
    }

    /// Checks if this [`Version`] is compatible with the given [`VersionSpec`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use facti_lib::version::{Version, VersionSpec};
    /// assert!(Version::new(1, 2, 3).matches(VersionSpec::parse("= 1.2.3")?));
    /// assert!(Version::new(4, 1, 3).matches(VersionSpec::parse("> 4.0.0")?));
    /// assert!(Version::new(0, 5, 1).matches(VersionSpec::parse("<= 1.0.0")?));
    /// # Ok::<(), facti_lib::error::ParseVersionSpecError>(())
    /// ```
    pub fn matches(&self, spec: VersionSpec) -> bool {
        spec.matches(*self)
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split('.').map(|p| p.trim()).collect::<Vec<_>>();

        if parts.len() != 3 {
            return Err(ParseVersionError::Size(3, parts.len()));
        }

        let major = parts[0].parse().map_err(ParseVersionError::Major)?;
        let minor = parts[1].parse().map_err(ParseVersionError::Minor)?;
        let patch = parts[2].parse().map_err(ParseVersionError::Patch)?;

        Ok(Version::new(major, minor, patch))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl From<FactorioVersion> for Version {
    fn from(value: FactorioVersion) -> Self {
        Self::new(value.major, value.minor, value.patch.unwrap_or(0))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Op {
    Exact,
    Greater,
    GreaterEq,
    Less,
    LessEq,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Op::Exact => "=",
            Op::Greater => ">",
            Op::GreaterEq => ">=",
            Op::Less => "<",
            Op::LessEq => "<=",
        })
    }
}

/// Represents a version requirement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VersionReq {
    /// Used when the latest version is desired.
    Latest,

    /// Used when a specific version is required,
    /// or one that matches a predicate.
    Spec(VersionSpec),
}

impl VersionReq {
    /// Parses a string into a [`VersionReq`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use facti_lib::version::VersionReq;
    /// let exact_req = VersionReq::parse("= 1.2.3")?;
    /// let latest = VersionReq::parse("")?;
    /// # Ok::<(), facti_lib::error::ParseVersionReqError>(())
    pub fn parse(s: &str) -> Result<Self, ParseVersionReqError> {
        s.parse()
    }
}

impl FromStr for VersionReq {
    type Err = ParseVersionReqError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Ok(VersionReq::Latest);
        }

        let spec: VersionSpec = trimmed.parse()?;

        Ok(VersionReq::Spec(spec))
    }
}

impl Display for VersionReq {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VersionReq::Latest => f.write_str(""),
            VersionReq::Spec(spec) => spec.fmt(f),
        }
    }
}

/// Specifies a specific require version, or a version matching
/// a given predicate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VersionSpec {
    /// The predicate/operator to use when matching versions.
    pub op: Op,

    /// The version to use as baseline.
    pub version: Version,
}

impl VersionSpec {
    pub fn new(op: Op, version: Version) -> Self {
        Self { op, version }
    }

    /// Parses a string into a [`VersionSpec`].
    pub fn parse(s: &str) -> Result<Self, ParseVersionSpecError> {
        s.parse()
    }

    /// Checks if the given [`Version`] matches this [`VersionSpec`].
    pub fn matches(&self, version: Version) -> bool {
        match self.op {
            Op::Exact => self.version == version,
            Op::Greater => self.version < version,
            Op::GreaterEq => self.version <= version,
            Op::Less => self.version > version,
            Op::LessEq => self.version >= version,
        }
    }
}

impl FromStr for VersionSpec {
    type Err = ParseVersionSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let semver_req = semver::VersionReq::parse(s)?;
        semver_req.try_into()
    }
}

impl Display for VersionSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{} {}", self.op, self.version))
    }
}

impl From<FactorioVersion> for VersionSpec {
    fn from(value: FactorioVersion) -> Self {
        Self::new(Op::GreaterEq, value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_specs {
        ($($name:ident($spec:literal, $version:literal, $expected:expr);)*) => {
            $(
                #[test]
                fn $name() {
                    let spec = VersionSpec::parse($spec).unwrap();
                    let version = Version::parse($version).unwrap();
                    assert_eq!(spec.matches(version), $expected, "expected {} when matching {version} against {spec}", $expected);
                }
            )*
        };
    }

    test_specs! {
        same_version_matches_exact("= 1.2.3", "1.2.3", true);
        diff_major_does_not_match_exact("= 1.2.3", "2.2.3", false);
        diff_minor_does_not_match_exact("= 1.2.3", "1.3.3", false);
        diff_patch_does_not_match_exact("= 1.2.3", "1.2.4", false);
        larger_major_matches_greater("> 1.2.3", "5.2.3", true);
        larger_minor_matches_greater("> 1.2.3", "1.5.3", true);
        larger_patch_matches_greater("> 1.2.3", "1.2.5", true);
        smaller_major_does_not_match_greater("> 1.2.3", "0.2.3", false);
        smaller_minor_greater_major_matches_greater("> 1.2.3", "3.1.3", true);
        smaller_patch_greater_major_matches_greater("> 1.2.3", "4.1.0", true);
    }
}
