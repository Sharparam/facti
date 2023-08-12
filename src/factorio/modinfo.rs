use serde::{Deserialize, Serialize};

use super::{dependency::Dependency, version::Version, FactorioVersion};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModInfo {
    pub name: String,

    pub version: Version,

    pub title: String,

    pub author: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub factorio_version: FactorioVersion,

    pub dependencies: Vec<Dependency>,
}
