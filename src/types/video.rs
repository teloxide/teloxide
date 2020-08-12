use serde::{Deserialize, Serialize};

use crate::types::{MimeWrapper, PhotoSize};

/// This object represents a video file.
///
/// [The official docs](https://core.telegram.org/bots/api#video).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Video {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// Video width as defined by sender.
    pub width: u32,

    /// Video height as defined by sender.
    pub height: u32,

    /// Duration of the video in seconds as defined by sender.
    pub duration: u32,

    /// Video thumbnail.
    pub thumb: Option<PhotoSize>,

    /// Mime type of a file as defined by sender.
    pub mime_type: Option<MimeWrapper>,

    /// File size.
    pub file_size: Option<u32>,
}

impl Video {
    pub fn new<S1, S2>(
        file_id: S1,
        file_unique_id: S2,
        width: u32,
        height: u32,
        duration: u32,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            width,
            height,
            duration,
            thumb: None,
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

    pub fn width(mut self, val: u32) -> Self {
        self.width = val;
        self
    }

    pub fn height(mut self, val: u32) -> Self {
        self.height = val;
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

    pub fn mime_type(mut self, val: MimeWrapper) -> Self {
        self.mime_type = Some(val);
        self
    }

    pub fn file_size(mut self, val: u32) -> Self {
        self.file_size = Some(val);
        self
    }
}
