#![allow(non_camel_case_types, dead_code)]
#[cfg(feature = "serde-integration")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::{self, Display},
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, PartialEq)]
/// An administrative subdivision as specified by the ISO 3166-2 standard
pub struct Subdivision {
    code: Code,
}

impl Subdivision {
    /// Returns the subdivision code
    pub fn code(&self) -> Code {
        self.code
    }
}

#[cfg(feature = "serde-integration")]
impl Serialize for Subdivision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.code)
    }
}

#[cfg(feature = "serde-integration")]
impl<'de> Deserialize<'de> for Subdivision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Code::from_str(&s)
            .map_err(de::Error::custom)
            .map(|code| Self { code })
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
    #[cfg(feature = "serde-integration")]
    fn serialize_subdivision() {
        let arizona: Subdivision = Subdivision { code: Code::US_AZ };

        let california: Subdivision = Subdivision { code: Code::US_CA };

        let colorado: Subdivision = Subdivision { code: Code::US_CO };

        let connecticut: Subdivision = Subdivision { code: Code::US_CT };

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
