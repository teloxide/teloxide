use std::iter;

use serde::Serialize;

use crate::types::{InputFile, Seconds};

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

impl From<InputPaidMedia> for InputFile {
    fn from(media: InputPaidMedia) -> InputFile {
        match media {
            InputPaidMedia::Photo(InputPaidMediaPhoto { media, .. })
            | InputPaidMedia::Video(InputPaidMediaVideo { media, .. }) => media,
        }
    }
}

impl InputPaidMedia {
    /// Returns an iterator of all files in this input media
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        use InputPaidMedia::*;

        let (media, thumbnail) = match self {
            Photo(InputPaidMediaPhoto { media, .. }) => (media, None),
            Video(InputPaidMediaVideo { media, thumbnail, .. }) => (media, thumbnail.as_ref()),
        };

        iter::once(media).chain(thumbnail)
    }

    /// Returns an iterator of all files in this input media
    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        use InputPaidMedia::*;

        let (media, thumbnail) = match self {
            Photo(InputPaidMediaPhoto { media, .. }) => (media, None),
            Video(InputPaidMediaVideo { media, thumbnail, .. }) => (media, thumbnail.as_mut()),
        };

        iter::once(media).chain(thumbnail)
    }
}

/// The paid media to send is a photo.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpaidmediaphoto).
#[derive(Clone, Debug, Serialize)]
pub struct InputPaidMediaPhoto {
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file from
    /// the Internet, or pass “attach://<file_attach_name>” to upload a new one
    /// using multipart/form-data under <file_attach_name> name. [More
    /// information on Sending Files »]
    ///
    /// [More information on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub media: InputFile,
}

impl InputPaidMediaPhoto {
    pub const fn new(media: InputFile) -> Self {
        Self { media }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }
}

/// The paid media to send is a video.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpaidmediavideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputPaidMediaVideo {
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file from
    /// the Internet, or pass “attach://<file_attach_name>” to upload a new one
    /// using multipart/form-data under <file_attach_name> name. [More
    /// information on Sending Files »]
    ///
    /// [More information on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data. Thumbnails can't be reused and can be only
    /// uploaded as a new file, so you can pass “attach://<file_attach_name>” if
    /// the thumbnail was uploaded using multipart/form-data under
    /// <file_attach_name>. [More information on Sending Files »]
    ///
    /// [More information on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub thumbnail: Option<InputFile>,

    /// Video width.
    pub width: Option<u16>,

    /// Video height.
    pub height: Option<u16>,

    /// Video duration in seconds.
    pub duration: Option<Seconds>,

    /// Pass `true`, if the uploaded video is suitable for streaming.
    pub supports_streaming: Option<bool>,
}

impl InputPaidMediaVideo {
    pub const fn new(media: InputFile) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn photo_serialize() {
        let expected_json = r#"{"type":"photo","media":"123456"}"#;
        let photo = InputPaidMedia::Photo(InputPaidMediaPhoto {
            media: InputFile::file_id("123456".into()),
        });

        let actual_json = serde_json::to_string(&photo).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn video_serialize() {
        let expected_json = r#"{"type":"video","media":"123456"}"#;
        let video = InputPaidMedia::Video(InputPaidMediaVideo {
            media: InputFile::file_id("123456".into()),
            thumbnail: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }
}
