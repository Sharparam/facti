use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseVersionError {
    #[error(
        "Wrong number of version parts, needs to have {0} parts but actual version had {1} parts"
    )]
    Size(usize, usize),

    #[error("Invalid major version")]
    Major(#[source] ParseIntError),

    #[error("Invalid minor version")]
    Minor(#[source] ParseIntError),

    #[error("Invalid patch version")]
    Patch(#[source] ParseIntError),

    #[error(transparent)]
    Semver(#[from] semver::Error),
}

#[derive(Error, Debug)]
pub enum ParseVersionSpecError {
    #[error(transparent)]
    Op(#[from] ParseOpError),

    #[error("Semver version req has no comparator, must have exactly one")]
    SemverReqMissingComparator,

    #[error("Semver version req has too many comparators, must have exactly one")]
    SemverReqTooManyComparators,

    #[error("Invalid or missing minor version")]
    Minor,

    #[error("Invalid or missing patch version")]
    Patch,

    #[error(transparent)]
    Semver(#[from] semver::Error),
}

#[derive(Error, Debug)]
#[error("Failed to parse the string as a valid version req")]
pub struct ParseVersionReqError(#[from] ParseVersionSpecError);

#[derive(Error, Debug)]
pub enum ParseOpError {
    #[error("{0:?} is not a supported operator, must be one of <, <=, =, >=, >")]
    Op(semver::Op),
}

#[derive(Error, Debug)]
pub enum ParseDependencyError {
    #[error("The dependency string \"{0}\" does not match the RegEx")]
    RegexMismatch(String),
}
