#![allow(non_camel_case_types, dead_code)]
#[cfg(feature = "serde-integration")]
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    str::FromStr,
};
use thiserror::Error;

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
        let arizona = Code::US_AZ;
        let california = Code::US_CA;
        let colorado = Code::US_CO;
        let connecticut = Code::US_CT;

        let tests = [
            (arizona, "US_AZ"),
            (california, "US_CA"),
            (colorado, "US_CO"),
            (connecticut, "US_CT"),
        ];

        for (subdivision, expected) in &tests {
            let actual =
                serde_json::to_value(subdivision).expect("should serialize the subdivision");

            assert_eq!(json!(expected), actual);

            // re-convert
            let converted: Code =
                serde_json::from_value(actual).expect("should deserialize the subdivision");

            assert_eq!(subdivision, &converted);
        }
    }
}
