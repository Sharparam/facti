use super::parse::{
    FactorioVersionParseError, VersionParseError, VersionReqParseError, VersionSpecParseError,
};

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

    pub fn parse(s: &str) -> Result<Self, FactorioVersionParseError> {
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
    pub fn parse(s: &str) -> Result<Self, VersionReqParseError> {
        s.parse()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VersionSpec {
    pub op: Op,
    pub version: Version,
}

impl VersionSpec {
    pub fn parse(s: &str) -> Result<Self, VersionSpecParseError> {
        s.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("1.80".parse(), Ok(FactorioVersion::new(1, 80)));
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
}
