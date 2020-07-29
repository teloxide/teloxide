use serde::{Deserialize, Serialize};

use crate::types::PhotoSize;

/// This object represents a [video message] (available in Telegram apps as of
/// [v.4.0]).
///
/// [The official docs](https://core.telegram.org/bots/api#videonote).
///
/// [video message]: https://telegram.org/blog/video-messages-and-telescope
/// [v4.0]: https://telegram.org/blog/video-messages-and-telescope
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VideoNote {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// Video width and height (diameter of the video message) as defined by
    /// sender.
    pub length: u32,

    /// Duration of the video in seconds as defined by sender.
    pub duration: u32,

    /// Video thumbnail.
    pub thumb: Option<PhotoSize>,

    /// File size.
    pub file_size: Option<u32>,
}

impl VideoNote {
    pub fn new<S1, S2>(file_id: S1, file_unique_id: S2, length: u32, duration: u32) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            length,
            duration,
            thumb: None,
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

    pub fn length(mut self, val: u32) -> Self {
        self.length = val;
        self
    }

    pub fn duration(mut self, val: u32) -> Self {
        self.duration = val;
        self
    }

    pub fn thumb(mut self, val: PhotoSize) -> Self {
        self.thumb = Some(val);
        self
    }

    pub fn file_size(mut self, val: u32) -> Self {
        self.file_size = Some(val);
        self
    }
}
