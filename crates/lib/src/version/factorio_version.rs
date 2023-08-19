use std::{
    cmp,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::error::ParseVersionError;

/// Represents a version of Factorio (the game).
///
/// In most cases, the [`patch`][`FactorioVersion::patch`] field
/// should be left as [`None`].
///
/// The game and its APIs may sometimes return a patch component,
/// and some wrongly configured mods on the mod portal may also have it
/// set (in error).
///
/// If you're constructing a [`ModInfo`][`facti_lib::ModInfo`] struct,
/// you **MUST NOT** set the patch component, as that is considered invalid
/// and the mod portal will reject your mod. It may also make the game behave
/// in unexpected ways.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FactorioVersion {
    /// The major component of the version.
    pub major: u64,

    /// The minor component of the version.
    pub minor: u64,

    /// The patch component of the version, if any.
    pub patch: Option<u64>,
}

impl FactorioVersion {
    /// Constructs a [`FactorioVersion`] with just the [`major`][`FactorioVersion::major`]
    /// and [`minor`][`FactorioVersion::minor`] fields set.
    ///
    /// This is most often the correct method to use.
    pub fn new(major: u64, minor: u64) -> Self {
        Self {
            major,
            minor,
            patch: None,
        }
    }

    pub fn with_patch(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch: Some(patch),
        }
    }

    /// Parses a [`FactorioVersion`] from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use facti_lib::version::FactorioVersion;
    /// let version = FactorioVersion::parse("1.2")?;
    /// assert_eq!(version.major, 1);
    /// assert_eq!(version.minor, 2);
    /// assert!(version.patch.is_none());
    ///
    /// let with_patch = FactorioVersion::parse("1.2.3")?;
    /// assert_eq!(with_patch.major, 1);
    /// assert_eq!(with_patch.minor, 2);
    /// assert_eq!(with_patch.patch, Some(3));
    /// # Ok::<(), facti_lib::error::ParseVersionError>(())
    /// ```
    pub fn parse(s: &str) -> Result<Self, ParseVersionError> {
        s.parse()
    }

    /// Constructs a potentially invalid Factorio version, which may include
    /// a patch version.
    ///
    /// Normally this should not be possible, but some mods on the portal have
    /// a patch version specified and will fail to parse if we don't allow it.
    pub(crate) fn create(major: u64, minor: u64, patch: Option<u64>) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Default for FactorioVersion {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 12,
            patch: None,
        }
    }
}

impl Ord for FactorioVersion {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use cmp::Ordering::*;
        match self.major.cmp(&other.major) {
            Equal => match self.minor.cmp(&other.minor) {
                Equal => match (self.patch, other.patch) {
                    (Some(self_patch), Some(other_patch)) => self_patch.cmp(&other_patch),
                    (Some(_), None) => Less,
                    (None, Some(_)) => Greater,
                    (None, None) => Equal,
                },
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd for FactorioVersion {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for FactorioVersion {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split('.').map(|p| p.trim()).collect::<Vec<_>>();

        if parts.len() > 3 {
            return Err(ParseVersionError::Size(2, parts.len()));
        }

        let major = parts[0].parse().map_err(ParseVersionError::Major)?;
        let minor = parts[1].parse().map_err(ParseVersionError::Minor)?;

        let patch: Option<u64> = if parts.len() == 3 {
            Some(parts[2].parse().map_err(ParseVersionError::Patch)?)
        } else {
            None
        };

        Ok(FactorioVersion::create(major, minor, patch))
    }
}

impl Display for FactorioVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)?;

        if let Some(patch) = self.patch {
            write!(f, ".{}", patch)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_super {
    use std::cmp;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            FactorioVersion::parse("1.80").unwrap(),
            FactorioVersion::new(1, 80)
        );
    }

    #[test]
    fn test_parse_patch() {
        assert_eq!(
            FactorioVersion::parse("1.80.66").unwrap(),
            FactorioVersion::with_patch(1, 80, 66)
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", FactorioVersion::new(1, 2)), "1.2");
    }

    #[test]
    fn test_display_patch() {
        assert_eq!(format!("{}", FactorioVersion::with_patch(1, 2, 3)), "1.2.3");
    }

    #[test]
    fn test_ordering() {
        let mut major_differs = vec![FactorioVersion::new(5, 1), FactorioVersion::new(1, 2)];
        major_differs.sort();
        assert_eq!(
            major_differs,
            vec![FactorioVersion::new(1, 2), FactorioVersion::new(5, 1)]
        );
        let mut minor_differs = vec![FactorioVersion::new(1, 5), FactorioVersion::new(1, 2)];
        minor_differs.sort();
        assert_eq!(
            minor_differs,
            vec![FactorioVersion::new(1, 2), FactorioVersion::new(1, 5)]
        );
    }

    /// Test that a version with a patch is greater than one without.
    ///
    /// Rationale: A [`FactorioVersion`] that has specified [`None`] for its
    /// patch version means it wants the latest version of Factorio that'
    /// matches the specified `major` and `minor` components. Thus the patch
    /// ([`None`]) will always be the latest available, and no explicitly
    /// stated `patch` value would be greater than that.
    #[test]
    fn test_nopatch_gt_haspatch() {
        let no_patch = FactorioVersion::new(1, 2);
        let has_patch = FactorioVersion::with_patch(1, 2, 0);

        assert_eq!(no_patch.cmp(&has_patch), cmp::Ordering::Greater)
    }
}
