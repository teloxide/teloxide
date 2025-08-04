use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, MessageEntity, ParseMode, Seconds};

/// Represents a link to a page containing an embedded video player or a video
/// file.
///
/// By default, this video file will be sent by the user with an optional
/// caption. Alternatively, you can use `input_messaage_content` to send a
/// message with the specified content instead of the video.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultvideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the embedded video player or video file.
    pub video_url: reqwest::Url,

    /// Mime type of the content of video url, `text/html` or `video/mp4`.
    #[serde(with = "crate::types::non_telegram_types::mime::deser")]
    #[cfg_attr(test, schemars(with = "String"))]
    pub mime_type: Mime,

    /// URL of the thumbnail (jpeg only) for the video.
    pub thumbnail_url: reqwest::Url,

    /// Title for the result.
    pub title: String,

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
    pub video_width: Option<u32>,

    /// Video height.
    pub video_height: Option<u32>,

    /// Video duration in seconds.
    pub video_duration: Option<Seconds>,

    /// Short description of the result.
    pub description: Option<String>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the video. This field is
    /// **required** if [`InlineQueryResultVideo`] is used to send an HTML-page
    /// as a result (e.g., a YouTube video).
    ///
    /// [`InlineQueryResultVideo`]:
    /// crate::types::InlineQueryResultVideo
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVideo {
    pub fn new<S1, S2>(
        id: S1,
        video_url: reqwest::Url,
        mime_type: Mime,
        thumbnail_url: reqwest::Url,
        title: S2,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            video_url,
            mime_type,
            thumbnail_url,
            title: title.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: false,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    #[must_use]
    pub fn video_url(mut self, val: reqwest::Url) -> Self {
        self.video_url = val;
        self
    }

    #[must_use]
    pub fn mime_type(mut self, val: Mime) -> Self {
        self.mime_type = val;
        self
    }

    #[must_use]
    pub fn thumbnail_url(mut self, val: reqwest::Url) -> Self {
        self.thumbnail_url = val;
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    #[must_use]
    pub fn parse_mode(mut self, val: ParseMode) -> Self {
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

    #[must_use]
    pub fn video_width(mut self, val: u32) -> Self {
        self.video_width = Some(val);
        self
    }

    #[must_use]
    pub fn video_height(mut self, val: u32) -> Self {
        self.video_height = Some(val);
        self
    }

    #[must_use]
    pub fn video_duration(mut self, val: Seconds) -> Self {
        self.video_duration = Some(val);
        self
    }

    pub fn description<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.description = Some(val.into());
        self
    }

    #[must_use]
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    #[must_use]
    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}
