use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// This object represents a file uploaded to Telegram Passport.
///
/// Currently all Telegram Passport files are in JPEG format when decrypted and
/// don't exceed 10MB.
///
/// [The official docs](https://core.telegram.org/bots/api#passportfile).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PassportFile {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// File size in bytes.
    pub file_size: u64, // FIXME: should be u32

    /// Time when the file was uploaded.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub file_date: DateTime<Utc>,
}
