use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FactorioVersion {
    pub major: u64,
    pub minor: u64,
}

impl FactorioVersion {
    pub fn new(major: u64, minor: u64) -> Self {
        Self { major, minor }
    }
}

impl fmt::Display for FactorioVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
