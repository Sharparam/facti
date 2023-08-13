use std::num::ParseIntError;

use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum VersionParseError {
    #[error(
        "Wrong number of version parts, needs to have {0} parts but actual version had {1} parts"
    )]
    InvalidSize(usize, usize),

    #[error("Invalid major version")]
    InvalidMajor(#[source] ParseIntError),

    #[error("Invalid minor version")]
    InvalidMinor(#[source] ParseIntError),

    #[error("Invalid patch version")]
    InvalidPatch(#[source] ParseIntError),

    #[error("Invalid operator, must be one of <, <=, =, >=, >")]
    InvalidOp,

    #[error(
        "Incompatible semver version req, must use one of the supported operators (<, <=, =, >=, >), only have one comparator (constraint), and specify all 3 version components (major.minor.patch)"
    )]
    IncompatibleSemverReq,

    #[error("Failed to parse the string as a valid semver version or version req")]
    InvalidSemver(#[source] semver::Error),
}

#[derive(Error, Debug)]
pub enum DependencyParseError {
    #[error("The dependency string \"{0}\" does not match the RegEx")]
    RegexMismatch(String),
}
