use super::version::{Op, Version, VersionReq, VersionSpec};

impl From<semver::Version> for Version {
    fn from(value: semver::Version) -> Self {
        Self {
            major: value.major,
            minor: value.minor,
            patch: value.patch,
        }
    }
}

impl From<Version> for semver::Version {
    fn from(value: Version) -> Self {
        Self::new(value.major, value.minor, value.patch)
    }
}

impl TryFrom<semver::Op> for Op {
    type Error = ();

    fn try_from(value: semver::Op) -> Result<Self, Self::Error> {
        match value {
            semver::Op::Exact => Ok(Op::Exact),
            semver::Op::Greater => Ok(Op::Greater),
            semver::Op::GreaterEq => Ok(Op::GreaterEq),
            semver::Op::Less => Ok(Op::Less),
            semver::Op::LessEq => Ok(Op::LessEq),
            _ => Err(()),
        }
    }
}

impl From<Op> for semver::Op {
    fn from(value: Op) -> Self {
        match value {
            Op::Exact => semver::Op::Exact,
            Op::Greater => semver::Op::Greater,
            Op::GreaterEq => semver::Op::GreaterEq,
            Op::Less => semver::Op::Less,
            Op::LessEq => semver::Op::LessEq,
        }
    }
}

impl TryFrom<semver::VersionReq> for VersionReq {
    type Error = ();

    fn try_from(value: semver::VersionReq) -> Result<Self, Self::Error> {
        if value == semver::VersionReq::STAR {
            return Ok(VersionReq::Latest);
        }
        if value.comparators.is_empty() {
            return Ok(VersionReq::Latest);
        }

        let spec = value.try_into().map_err(|_| ())?;

        Ok(VersionReq::Spec(spec))
    }
}

impl From<VersionReq> for semver::VersionReq {
    fn from(value: VersionReq) -> Self {
        match value {
            VersionReq::Latest => semver::VersionReq::STAR,
            VersionReq::Spec(spec) => spec.into(),
        }
    }
}

impl TryFrom<semver::VersionReq> for VersionSpec {
    type Error = ();

    fn try_from(value: semver::VersionReq) -> Result<Self, Self::Error> {
        if value.comparators.is_empty() {
            return Err(());
        }

        if value.comparators.len() > 1 {
            return Err(());
        }

        let comp = &value.comparators[0];
        let op = comp.op.try_into()?;
        let major = comp.major;
        let minor = comp.minor.ok_or(())?;
        let patch = comp.patch.ok_or(())?;
        let version = Version::new(major, minor, patch);
        Ok(VersionSpec { op, version })
    }
}

impl From<VersionSpec> for semver::VersionReq {
    fn from(value: VersionSpec) -> Self {
        semver::VersionReq {
            comparators: vec![semver::Comparator {
                op: value.op.into(),
                major: value.version.major,
                minor: Some(value.version.minor),
                patch: Some(value.version.patch),
                pre: semver::Prerelease::EMPTY,
            }],
        }
    }
}
