use std::fmt;

use super::modinfo::{Compatibility, Dependency, DependencyMode};

impl fmt::Display for DependencyMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DependencyMode::*;
        match self {
            Required => write!(f, ""),
            Optional { hidden } => write!(f, "{}", if *hidden { "(?)" } else { "?" }),
            Independent => write!(f, "~"),
        }
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Compatibility::*;
        match &self.compatibility {
            Compatible(r, DependencyMode::Required) => write!(f, "{} {}", self.name, r),
            Compatible(r, m) => write!(f, "{} {} {}", m, self.name, r),
            Incompatible => write!(f, "! {}", self.name),
        }
    }
}
