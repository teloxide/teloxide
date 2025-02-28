use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// Percentage from 0 to 100
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Debug, derive_more::Display)]
#[display("{_0}%")]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Percentage(u8);

impl Percentage {
    /// Creates a new [Percentage] with a given number.
    ///
    /// ## Panics
    ///
    /// If `percentage` is greater than 100.
    pub const fn from_u8(percentage: u8) -> Self {
        assert!(percentage <= 100);

        Self(percentage)
    }

    /// Returns the percentage
    pub const fn value(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Percentage {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 100 {
            Ok(Self(value))
        } else {
            Err("Percentage only accepts values less than or equal to 100!")
        }
    }
}

impl Deref for Percentage {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct S {
        intensity: Percentage,
    }

    #[test]
    fn deserialization() {
        let expected = Percentage::from_u8(69);
        let S { intensity: actual } = serde_json::from_str(r#"{"intensity":69}"#).unwrap();

        assert_eq!(expected, actual);
        assert_eq!(actual.value(), 69);
    }

    #[test]
    fn serialization() {
        let expected = r#"{"intensity":69}"#;
        let actual = serde_json::to_string(&S { intensity: Percentage::from_u8(69) }).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn above_limit() {
        Percentage::from_u8(101);
    }
}
