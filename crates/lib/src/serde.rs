use serde::{de::Visitor, Deserialize, Serialize};

use super::{dependency::Dependency, version::Version, FactorioVersion};

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str = format!("{}.{}.{}", self.major, self.minor, self.patch);
        serializer.serialize_str(str.as_str())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VersionVisitor;

        impl Visitor<'_> for VersionVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid version string ('major.minor.patch')")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Version::parse(v).map_err(|_| serde::de::Error::custom("invalid version"))
            }
        }

        deserializer.deserialize_str(VersionVisitor)
    }
}

impl Serialize for FactorioVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str = if let Some(patch) = self.patch {
            format!("{}.{}.{}", self.major, self.minor, patch)
        } else {
            format!("{}.{}", self.major, self.minor)
        };

        serializer.serialize_str(str.as_str())
    }
}

impl<'de> Deserialize<'de> for FactorioVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FactorioVersionVisitor;

        impl Visitor<'_> for FactorioVersionVisitor {
            type Value = FactorioVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid Factorio version string ('major.minor')")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                FactorioVersion::parse(v)
                    .map_err(|_| serde::de::Error::custom("invalid Factorio version"))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(FactorioVersion::default())
            }
        }

        deserializer.deserialize_str(FactorioVersionVisitor)
    }
}

impl Serialize for Dependency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DependencyVisitor;

        impl Visitor<'_> for DependencyVisitor {
            type Value = Dependency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid dependency string ('[mode] name [version-req]')")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Dependency::parse(v).map_err(|_| serde::de::Error::custom("invalid dependency"))
            }
        }

        deserializer.deserialize_str(DependencyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::version::VersionReq;

    use super::*;

    #[test]
    fn test_serialize_version() {
        let version = Version::parse("1.2.3").unwrap();
        let serialized = serde_json::to_string(&version).unwrap();
        assert_eq!(serialized, "\"1.2.3\"");
    }

    #[test]
    fn test_deserialize_version() {
        let version = Version::parse("1.2.3").unwrap();
        let deserialized: Version = serde_json::from_str("\"1.2.3\"").unwrap();
        assert_eq!(deserialized, version);
    }

    #[test]
    fn test_serialize_factorio_version() {
        let version = FactorioVersion::parse("1.2").unwrap();
        let serialized = serde_json::to_string(&version).unwrap();
        assert_eq!(serialized, "\"1.2\"");
    }

    #[test]
    fn test_deserialize_factorio_version() {
        let version = FactorioVersion::parse("1.2").unwrap();
        let deserialized: FactorioVersion = serde_json::from_str("\"1.2\"").unwrap();
        assert_eq!(deserialized, version);
    }

    #[test]
    fn test_serialize_dependency() {
        let dependency =
            Dependency::optional("facti", VersionReq::parse(">= 4.2.0").unwrap(), false);
        let serialized = serde_json::to_string(&dependency).unwrap();
        assert_eq!(serialized, "\"? facti >= 4.2.0\"");
    }

    #[test]
    fn test_deserialize_dependency() {
        let dependency = Dependency::optional("facti", VersionReq::parse("< 1.0.0").unwrap(), true);
        let deserialized: Dependency = serde_json::from_str("\"(?) facti < 1.0.0\"").unwrap();
        assert_eq!(deserialized, dependency);
    }
}
