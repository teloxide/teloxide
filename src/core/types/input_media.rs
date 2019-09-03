use serde::Deserialize;
use crate::core::types::{InputFile, ParseMode};

// TODO: should variants use new-type?
#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
/// This object represents the content of a media message to be sent.
/// [More](https://core.telegram.org/bots/api#inputmedia)
pub enum InputMedia {
    /// Represents a photo to be sent.
    Photo {
        /// File to send.
        media: InputFile,
        /// Caption of the photo to be sent, 0-1024 characters
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Send [Markdown] or [HTML],
        /// if you want Telegram apps to show [bold, italic, fixed-width text or inline URLs]
        /// in the media caption.
        ///
        /// [Markdown]: crate::core::types::ParseMode::Markdown
        /// [Html]: crate::core::types::ParseMode::Html
        /// [bold, italic, fixed-width text or inline URLs]: crate::core::types::ParseMode
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
    },
    Video {
        /// File to send.File to send.
        media: InputFile,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
        /// supported server-side.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail‘s width and height should not exceed 320.
        /// Ignored if the file is not uploaded using [InputFile::File].
        ///
        /// [InputFile::File]: crate::core::types::InputFile::File
        thumb: Option<InputFile>,
        /// Caption of the video to be sent, 0-1024 characters.
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Send [Markdown] or [HTML],
        /// if you want Telegram apps to show [bold, italic, fixed-width text or inline URLs]
        /// in the media caption.
        ///
        /// [Markdown]: crate::core::types::ParseMode::Markdown
        /// [Html]: crate::core::types::ParseMode::Html
        /// [bold, italic, fixed-width text or inline URLs]: crate::core::types::ParseMode
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// Video width
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u16>,
        /// Video height
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u16>,
        /// Video duration
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u16>,
        /// Pass `true`, if the uploaded video is suitable for streaming
        #[serde(skip_serializing_if = "Option::is_none")]
        supports_streaming: Option<bool>,
    },
    /// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
    Animation {
        /// File to send.
        media: InputFile,
        /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
        /// supported server-side.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail‘s width and height should not exceed 320.
        /// Ignored if the file is not uploaded using [InputFile::File].
        ///
        /// [InputFile::File]: crate::core::types::InputFile::File
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<InputFile>,
        /// Caption of the animation to be sent, 0-1024 characters
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Send [Markdown] or [HTML],
        /// if you want Telegram apps to show [bold, italic, fixed-width text or inline URLs]
        /// in the media caption.
        ///
        /// [Markdown]: crate::core::types::ParseMode::Markdown
        /// [Html]: crate::core::types::ParseMode::Html
        /// [bold, italic, fixed-width text or inline URLs]: crate::core::types::ParseMode
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        /// Animation width
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u16>,
        /// Animation height
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u16>,
        /// Animation duration
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u16>,
    },
    /// Represents an audio file to be treated as music to be sent.
    Audio {
        /// File to send,
        media: InputFile,
        /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
        /// supported server-side.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail‘s width and height should not exceed 320.
        /// Ignored if the file is not uploaded using [InputFile::File].
        ///
        /// [InputFile::File]: crate::core::types::InputFile::File
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<InputFile>,
        /// Caption of the audio to be sent, 0-1024 characters
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Send [Markdown] or [HTML],
        /// if you want Telegram apps to show [bold, italic, fixed-width text or inline URLs]
        /// in the media caption.
        ///
        /// [Markdown]: crate::core::types::ParseMode::Markdown
        /// [Html]: crate::core::types::ParseMode::Html
        /// [bold, italic, fixed-width text or inline URLs]: crate::core::types::ParseMode
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        /// Duration of the audio in seconds
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<u16>,
        /// Performer of the audio
        #[serde(skip_serializing_if = "Option::is_none")]
        performer: Option<String>,
        /// Title of the audio
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>
    },
    /// Represents a general file to be sent.
    Document {
        /// File to send.
        media: InputFile,
        /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is
        /// supported server-side.
        /// The thumbnail should be in JPEG format and less than 200 kB in size.
        /// A thumbnail‘s width and height should not exceed 320.
        /// Ignored if the file is not uploaded using [InputFile::File].
        ///
        /// [InputFile::File]: crate::core::types::InputFile::File
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb: Option<InputFile>,
        /// Caption of the document to be sent, 0-1024 characters
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        /// Send [Markdown] or [HTML],
        /// if you want Telegram apps to show [bold, italic, fixed-width text or inline URLs]
        /// in the media caption.
        ///
        /// [Markdown]: crate::core::types::ParseMode::Markdown
        /// [Html]: crate::core::types::ParseMode::Html
        /// [bold, italic, fixed-width text or inline URLs]: crate::core::types::ParseMode
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn photo_serialize() {
        let expected_json = r#"{"type":"photo","media":"123456"}"#;
        let photo = InputMedia::Photo {
            media: InputFile::FileId(String::from("123456")),
            caption: None,
            parse_mode: None,
        };

        let actual_json = serde_json::to_string(&photo).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn video_serialize() {
        let expected_json = r#"{"type":"video","media":"123456"}"#;
        let video = InputMedia::Video {
            media: InputFile::FileId(String::from("123456")),
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        };

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn animation_serialize() {
        let expected_json = r#"{"type":"animation","media":"123456"}"#;
        let video = InputMedia::Animation {
            media: InputFile::FileId(String::from("123456")),
            thumb: None,
            caption: None,
            parse_mode: None,
            width: None,
            height: None,
            duration: None,
        };

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn audio_serialize() {
        let expected_json = r#"{"type":"audio","media":"123456"}"#;
        let video = InputMedia::Audio {
            media: InputFile::FileId(String::from("123456")),
            thumb: None,
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None
        };

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn document_serialize() {
        let expected_json = r#"{"type":"document","media":"123456"}"#;
        let video = InputMedia::Document {
            media: InputFile::FileId(String::from("123456")),
            thumb: None,
            caption: None,
            parse_mode: None,
        };

        let actual_json = serde_json::to_string(&video).unwrap();
        assert_eq!(expected_json, actual_json);
    }
}
