#![allow(non_camel_case_types, dead_code)]
#[cfg(feature = "serde1")]
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    fmt::{self, Display},
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, PartialEq)]
/// An administrative subdivision as specified by the ISO 3166-2 standard
pub struct Subdivision {
    code: Code,
    name: &'static str,
    ty: &'static str,
}

impl Subdivision {
    /// Returns the subdivision code
    pub fn code(&self) -> Code {
        self.code
    }

    /// Returns the subdivision name in the language corresponding to the given country
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the subdivision type specific to the given country
    pub fn ty(&self) -> &'static str {
        self.ty
    }
}

#[cfg(feature = "serde1")]
impl Serialize for Subdivision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.code)
    }
}

#[cfg(feature = "serde1")]
impl<'de> Deserialize<'de> for Subdivision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SubdivisionVisitor)
    }
}

#[cfg(feature = "serde1")]
struct SubdivisionVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for SubdivisionVisitor {
    type Value = Subdivision;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an ISO 3166-1 compliant alpha-2 country code")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Code::from_str(v) {
            Ok(x) => Ok(x.into()),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Code::from_str(v) {
            Ok(x) => Ok(x.into()),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }
}

/// Errors that might arise when converting raw data into a subdivision code
#[derive(Debug, Error, PartialEq)]
pub enum SubdivisionCodeParseError {
    #[error("invalid iso subdivison code string")]
    InvalidCode,
}

// -----------------------------------------------------------------------------

include!(concat!(env!("OUT_DIR"), "/subdivision.rs"));

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{self, json};

    #[test]
    fn convert_from_and_to_string() {
        let tests = [
            ("US-NY", Ok(Code::US_NY)),
            ("FR-75", Ok(Code::FR_75)),
            ("GB-ABC", Ok(Code::GB_ABC)),
            // Invalid inputs
            ("FR_75", Err(SubdivisionCodeParseError::InvalidCode)), // Underscore instead of dash
            ("fr-75", Err(SubdivisionCodeParseError::InvalidCode)), // Lowercase
            ("invalid", Err(SubdivisionCodeParseError::InvalidCode)),
        ];

        for (raw, expected) in &tests {
            let actual = Code::from_str(raw);
            assert_eq!(expected, &actual);

            // re-convert
            if let Ok(actual) = actual {
                let str = actual.to_string();
                assert_eq!(str, raw.to_string());
            }
        }
    }

    #[test]
    #[cfg(feature = "serde1")]
    fn serialize_subdivision() {
        let arizona: Subdivision = Subdivision {
            name: "Arizona",
            ty: "State",
            code: Code::US_AZ,
        };

        let california: Subdivision = Subdivision {
            name: "California",
            ty: "State",
            code: Code::US_CA,
        };

        let colorado: Subdivision = Subdivision {
            name: "Colorado",
            ty: "State",
            code: Code::US_CO,
        };

        let connecticut: Subdivision = Subdivision {
            name: "Connecticut",
            ty: "State",
            code: Code::US_CT,
        };

        let tests = [
            (arizona, "US-AZ"),
            (california, "US-CA"),
            (colorado, "US-CO"),
            (connecticut, "US-CT"),
        ];

        for (subdivision, expected) in &tests {
            let actual =
                serde_json::to_value(subdivision).expect("should serialize the subdivision");

            assert_eq!(json!(expected), actual);

            // re-convert
            let converted: Subdivision =
                serde_json::from_value(actual).expect("should deserialize the subdivision");

            assert_eq!(subdivision, &converted);
        }
    }
}
