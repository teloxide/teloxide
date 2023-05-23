use serde::{Deserialize, Serialize};

/// A wrapper around `u32` which represents duration in seconds.
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Debug, derive_more::Display)]
#[display(fmt = "{_0}s")]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Seconds(u32);

impl Seconds {
    /// Creates a new duration with a given number of `seconds`.
    pub const fn from_seconds(seconds: u32) -> Self {
        Self(seconds)
    }

    /// Returns the number of seconds in this duration
    pub const fn seconds(self) -> u32 {
        self.0
    }

    /// Returns [`std::time::Duration`] equivalent of this duration.
    pub const fn duration(self) -> std::time::Duration {
        std::time::Duration::from_secs(self.seconds() as u64)
    }

    /// Returns [`chrono::Duration`] equivalent of this duration.
    pub fn chrono_duration(self) -> chrono::Duration {
        chrono::Duration::seconds(self.seconds() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct W {
        seconds: Seconds,
    }

    #[test]
    fn deserialization() {
        let expected = Seconds::from_seconds(123456);
        let W { seconds: actual } = serde_json::from_str(r#"{"seconds":123456}"#).unwrap();

        assert_eq!(expected, actual);
        assert_eq!(actual.seconds(), 123456);
    }

    #[test]
    fn serialization() {
        let expected = r#"{"seconds":123456}"#;
        let actual = serde_json::to_string(&W { seconds: Seconds::from_seconds(123456) }).unwrap();

        assert_eq!(expected, actual);
    }
}
