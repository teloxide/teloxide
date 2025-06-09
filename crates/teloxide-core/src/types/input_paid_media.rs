use crate::types::{InputFile, Seconds};
use serde::Serialize;

/// This object describes the paid media to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpaidmedia).
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputPaidMedia {
    Photo(InputPaidMediaPhoto),
    Video(InputPaidMediaVideo),
}

/// The paid media to send is a photo.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpaidmediaphoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputPaidMediaPhoto {
    /// File to send.
    pub media: InputFile,
}

impl InputPaidMediaPhoto {
    pub fn new(media: InputFile) -> Self {
        Self { media }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }
}

/// Represents a video to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediavideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputPaidMediaVideo {
    /// File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumbnail: Option<InputFile>,

    /// Video width.
    pub width: Option<u16>,

    /// Video height.
    pub height: Option<u16>,

    /// Video duration.
    pub duration: Option<Seconds>,

    /// Pass `true`, if the uploaded video is suitable for streaming.
    pub supports_streaming: Option<bool>,
}

impl InputPaidMediaVideo {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumbnail: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn thumbnail(mut self, val: InputFile) -> Self {
        self.thumbnail = Some(val);
        self
    }

    pub const fn width(mut self, val: u16) -> Self {
        self.width = Some(val);
        self
    }

    pub const fn height(mut self, val: u16) -> Self {
        self.height = Some(val);
        self
    }

    pub const fn duration(mut self, val: Seconds) -> Self {
        self.duration = Some(val);
        self
    }

    pub const fn supports_streaming(mut self, val: bool) -> Self {
        self.supports_streaming = Some(val);
        self
    }
}
