use std::fmt::{self, Display, Formatter};

use super::{
    dependency::{Compatibility, Dependency, DependencyMode},
    version::{Op, Version, VersionReq, VersionSpec},
    FactorioVersion,
};

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Display for FactorioVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
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

impl Display for VersionReq {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VersionReq::Latest => f.write_str(""),
            VersionReq::Spec(spec) => spec.fmt(f),
        }
    }
}

impl Display for VersionSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{} {}", self.op, self.version))
    }
}

impl fmt::Display for DependencyMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use DependencyMode::*;
        match self {
            Required => write!(f, ""),
            Optional { hidden } => write!(f, "{}", if *hidden { "(?)" } else { "?" }),
            Independent => write!(f, "~"),
        }
    }
}

impl fmt::Display for Dependency {
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
