use std::iter;

use serde::Serialize;

use crate::types::{InputFile, MessageEntity, ParseMode};

/// This object represents the content of a media message to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmedia).
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputMedia {
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
    Animation(InputMediaAnimation),
    Audio(InputMediaAudio),
    Document(InputMediaDocument),
}

/// Represents a photo to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediaphoto).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaPhoto {
    /// File to send.
    pub media: InputFile,

    /// Caption of the photo to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Pass `true`, if the caption must be shown above the message media.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub show_caption_above_media: bool,

    /// Pass `true` if the photo needs to be covered with a spoiler animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,
}

impl InputMediaPhoto {
    pub const fn new(media: InputFile) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
        }
    }

    pub fn media(mut self, val: InputFile) -> Self {
        self.media = val;
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub const fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }

    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = val;
        self
    }

    /// Sets [`has_spoiler`] to `true`.
    ///
    /// [`has_spoiler`]: InputMediaPhoto::has_spoiler
    pub fn spoiler(mut self) -> Self {
        self.has_spoiler = true;
        self
    }
}

/// Represents a video to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediavideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaVideo {
    // File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumbnail: Option<InputFile>,

    /// Caption of the video to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Pass `true`, if the caption must be shown above the message media.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub show_caption_above_media: bool,

    /// Video width.
    pub width: Option<u16>,

    /// Video height.
    pub height: Option<u16>,

    /// Video duration.
    pub duration: Option<u16>,

    /// Pass `true`, if the uploaded video is suitable for streaming.
    pub supports_streaming: Option<bool>,

    /// Pass `true` if the video needs to be covered with a spoiler animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,
}

impl InputMediaVideo {
    pub const fn new(media: InputFile) -> Self {
        Self {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: false,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
            has_spoiler: false,
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

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub const fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }

    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = val;
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

    pub const fn duration(mut self, val: u16) -> Self {
        self.duration = Some(val);
        self
    }

    pub const fn supports_streaming(mut self, val: bool) -> Self {
        self.supports_streaming = Some(val);
        self
    }

    /// Sets [`has_spoiler`] to `true`.
    ///
    /// [`has_spoiler`]: InputMediaVideo::has_spoiler
    pub fn spoiler(mut self) -> Self {
        self.has_spoiler = true;
        self
    }
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without
/// sound) to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediaanimation).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaAnimation {
    /// File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumbnail: Option<InputFile>,

    /// Caption of the animation to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Pass `true`, if the caption must be shown above the message media.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub show_caption_above_media: bool,

    /// Animation width.
    pub width: Option<u16>,

    /// Animation height.
    pub height: Option<u16>,

    /// Animation duration.
    pub duration: Option<u16>,

    /// Pass `true` if the animation needs to be covered with a spoiler
    /// animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_spoiler: bool,
}

impl InputMediaAnimation {
    pub const fn new(media: InputFile) -> Self {
        Self {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
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

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub const fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }

    pub fn show_caption_above_media(mut self, val: bool) -> Self {
        self.show_caption_above_media = val;
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

    pub const fn duration(mut self, val: u16) -> Self {
        self.duration = Some(val);
        self
    }

    /// Sets [`has_spoiler`] to `true`.
    ///
    /// [`has_spoiler`]: InputMediaAnimation::has_spoiler
    pub fn spoiler(mut self) -> Self {
        self.has_spoiler = true;
        self
    }
}

/// Represents an audio file to be treated as music to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediaaudio).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaAudio {
    /// File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumbnail: Option<InputFile>,

    /// Caption of the audio to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Duration of the audio in seconds.
    pub duration: Option<u16>,

    /// Performer of the audio.
    pub performer: Option<String>,

    /// Title of the audio.
    pub title: Option<String>,
}

impl InputMediaAudio {
    pub const fn new(media: InputFile) -> Self {
        Self {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            performer: None,
            title: None,
            duration: None,
            caption_entities: None,
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

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub const fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }

    pub const fn duration(mut self, val: u16) -> Self {
        self.duration = Some(val);
        self
    }

    pub fn performer<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.performer = Some(val.into());
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(val.into());
        self
    }
}

/// Represents a general file to be sent.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmediadocument).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct InputMediaDocument {
    /// File to send.
    pub media: InputFile,

    /// Thumbnail of the file sent; can be ignored if thumbnail generation
    /// for the file is supported server-side. The thumbnail should be in
    /// JPEG format and less than 200 kB in size. A thumbnail‘s width and
    /// height should not exceed 320. Ignored if the file is not uploaded
    /// using multipart/form-data.
    pub thumbnail: Option<InputFile>,

    /// Caption of the document to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// Disables automatic server-side content type detection for files uploaded
    /// using multipart/form-data. Always true, if the document is sent as part
    /// of an album.
    pub disable_content_type_detection: Option<bool>,
}

impl InputMediaDocument {
    pub const fn new(media: InputFile) -> Self {
        Self {
            media,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            disable_content_type_detection: None,
            caption_entities: None,
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

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub const fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }
}

impl From<InputMedia> for InputFile {
    fn from(media: InputMedia) -> InputFile {
        match media {
            InputMedia::Photo(InputMediaPhoto { media, .. })
            | InputMedia::Document(InputMediaDocument { media, .. })
            | InputMedia::Audio(InputMediaAudio { media, .. })
            | InputMedia::Animation(InputMediaAnimation { media, .. })
            | InputMedia::Video(InputMediaVideo { media, .. }) => media,
        }
    }
}

impl InputMedia {
    /// Returns an iterator of all files in this input media
    pub(crate) fn files(&self) -> impl Iterator<Item = &InputFile> {
        use InputMedia::*;

        let (media, thumbnail) = match self {
            Photo(InputMediaPhoto { media, .. }) => (media, None),
            Document(InputMediaDocument { media, thumbnail, .. })
            | Audio(InputMediaAudio { media, thumbnail, .. })
            | Animation(InputMediaAnimation { media, thumbnail, .. })
            | Video(InputMediaVideo { media, thumbnail, .. }) => (media, thumbnail.as_ref()),
        };

        iter::once(media).chain(thumbnail)
    }

    /// Returns an iterator of all files in this input media
    pub(crate) fn files_mut(&mut self) -> impl Iterator<Item = &mut InputFile> {
        use InputMedia::*;

        let (media, thumbnail) = match self {
            Photo(InputMediaPhoto { media, .. }) => (media, None),
            Document(InputMediaDocument { media, thumbnail, .. })
            | Audio(InputMediaAudio { media, thumbnail, .. })
            | Animation(InputMediaAnimation { media, thumbnail, .. })
            | Video(InputMediaVideo { media, thumbnail, .. }) => (media, thumbnail.as_mut()),
        };

        iter::once(media).chain(thumbnail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn photo_serialize() {
        let expected_json = r#"{"type":"photo","media":"123456"}"#;
        let photo = InputMedia::Photo(InputMediaPhoto {
            media: InputFile::file_id("123456"),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
        });

        let actual_json = serde_json::to_string(&photo).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn video_serialize() {
        let expected_json = r#"{"type":"video","media":"123456"}"#;
        let video = InputMedia::Video(InputMediaVideo {
            media: InputFile::file_id("123456"),
            thumbnail: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn animation_serialize() {
        let expected_json = r#"{"type":"animation","media":"123456"}"#;
        let video = InputMedia::Animation(InputMediaAnimation {
            media: InputFile::file_id("123456"),
            thumbnail: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn audio_serialize() {
        let expected_json = r#"{"type":"audio","media":"123456"}"#;
        let video = InputMedia::Audio(InputMediaAudio {
            media: InputFile::file_id("123456"),
            thumbnail: None,
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
            caption_entities: None,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn document_serialize() {
        let expected_json = r#"{"type":"document","media":"123456"}"#;
        let video = InputMedia::Document(InputMediaDocument {
            media: InputFile::file_id("123456"),
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
        });

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }
}
