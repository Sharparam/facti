use super::error::VersionParseError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
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

    pub fn parse(s: &str) -> Result<Self, VersionParseError> {
        s.parse()
    }

    pub fn matches(&self, spec: VersionSpec) -> bool {
        spec.matches(*self)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FactorioVersion {
    pub major: u64,
    pub minor: u64,
}

impl FactorioVersion {
    pub fn new(major: u64, minor: u64) -> Self {
        Self { major, minor }
    }

    pub fn parse(s: &str) -> Result<Self, VersionParseError> {
        s.parse()
    }
}

impl Default for FactorioVersion {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 12,
        }
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VersionReq {
    Latest,
    Spec(VersionSpec),
}

impl VersionReq {
    pub fn parse(s: &str) -> Result<Self, VersionParseError> {
        s.parse()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VersionSpec {
    pub op: Op,
    pub version: Version,
}

impl VersionSpec {
    pub fn parse(s: &str) -> Result<Self, VersionParseError> {
        s.parse()
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            FactorioVersion::parse("1.80").unwrap(),
            FactorioVersion::new(1, 80)
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", FactorioVersion::new(1, 2)), "1.2");
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
