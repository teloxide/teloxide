use mime::Mime;
use serde::{Deserialize, Serialize};

/// This object represents a voice note.
///
/// [The official docs](https://core.telegram.org/bots/api#voice).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Voice {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// Duration of the audio in seconds as defined by sender.
    pub duration: u32,

    /// MIME type of the file as defined by sender.
    #[serde(with = "crate::types::non_telegram_types::mime::opt_deser")]
    pub mime_type: Option<Mime>,

    /// File size.
    pub file_size: Option<u64>,
}

impl Voice {
    pub fn new<S1, S2>(file_id: S1, file_unique_id: S2, duration: u32) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            duration,
            mime_type: None,
            file_size: None,
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

    pub fn duration(mut self, val: u32) -> Self {
        self.duration = val;
        self
    }

    pub fn mime_type(mut self, val: Mime) -> Self {
        self.mime_type = Some(val);
        self
    }

    pub fn file_size(mut self, val: u64) -> Self {
        self.file_size = Some(val);
        self
    }
}
