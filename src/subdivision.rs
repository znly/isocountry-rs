#![allow(non_camel_case_types, dead_code)]

#[cfg(feature = "serde1")]
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize, Serializer,
};
use std::{
    fmt::{self, Display},
    str::FromStr,
};
use thiserror::Error;

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

// -----------------------------------------------------------------------------

include!(concat!(env!("OUT_DIR"), "/subdivision.rs"));

// -----------------------------------------------------------------------------

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
struct SubdivisionVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for SubdivisionVisitor {
    type Value = Code;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an ISO 3166-1 compliant alpha-2 country code")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Code::from_str(v) {
            Ok(x) => Ok(x),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Code::from_str(v) {
            Ok(x) => Ok(x),
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
