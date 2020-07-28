use serde::{Deserialize, Serialize};

/// This object represents a file uploaded to Telegram Passport.
///
/// Currently all Telegram Passport files are in JPEG format when decrypted and
/// don't exceed 10MB.
///
/// [The official docs](https://core.telegram.org/bots/api#passportfile).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PassportFile {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// File size.
    pub file_size: u64,

    /// Unix time when the file was uploaded.
    pub file_date: u64,
}

impl PassportFile {
    pub fn new<S1, S2>(file_id: S1, file_unique_id: S2, file_size: u64, file_date: u64) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            file_size,
            file_date,
        }
    }

    pub fn file_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_id = val.into();
        self
    }

    pub fn file_unique_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_unique_id = val.into();
        self
    }

    pub fn file_size(mut self, val: u64) -> Self {
        self.file_size = val;
        self
    }

    pub fn file_date(mut self, val: u64) -> Self {
        self.file_date = val;
        self
    }
}
