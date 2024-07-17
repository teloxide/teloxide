use chrono::{DateTime, Utc};
use derive_more::Deref;
use serde::{Deserialize, Serialize};

use crate::types::FileMeta;

/// This object represents a file uploaded to Telegram Passport.
///
/// Currently all Telegram Passport files are in JPEG format when decrypted and
/// don't exceed 10MB.
///
/// [The official docs](https://core.telegram.org/bots/api#passportfile).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Deref)]
pub struct PassportFile {
    /// Metadata of the passport file.
    #[deref]
    #[serde(flatten)]
    pub file: FileMeta,

    /// Time when the file was uploaded.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[serde(rename = "file_date")]
    pub date: DateTime<Utc>,
}
