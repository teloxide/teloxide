use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, MessageEntity, ParseMode, Seconds};

/// Represents a link to an animated GIF file.
///
/// By default, this animated GIF file will be sent by the user with optional
/// caption. Alternatively, you can use `input_message_content` to send a
/// message with the specified content instead of the animation.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultgif).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultGif {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the GIF file. File size must not exceed 1MB.
    pub gif_url: reqwest::Url,

    /// Width of the GIF.
    pub gif_width: Option<u32>,

    /// Height of the GIF.
    pub gif_height: Option<u32>,

    /// Duration of the GIF.
    pub gif_duration: Option<Seconds>,

    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the
    /// result
    pub thumbnail_url: reqwest::Url,

    // FIXME: maybe make dedicated enum for the mime type?
    /// MIME type of the thumbnail, must be one of “image/jpeg”,
    /// “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
    pub thumbnail_mime_type: Option<String>,

    /// Title for the result.
    pub title: Option<String>,

    /// Caption of the GIF file to be sent, 0-1024 characters.
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

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the GIF animation.
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultGif {
    pub fn new<S>(id: S, gif_url: reqwest::Url, thumbnail_url: reqwest::Url) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: id.into(),
            gif_url,
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumbnail_url,
            thumbnail_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
            caption_entities: None,
            show_caption_above_media: false,
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
    pub fn gif_url(mut self, val: reqwest::Url) -> Self {
        self.gif_url = val;
        self
    }

    #[must_use]
    pub fn gif_width(mut self, val: u32) -> Self {
        self.gif_width = Some(val);
        self
    }

    #[must_use]
    pub fn gif_height(mut self, val: u32) -> Self {
        self.gif_height = Some(val);
        self
    }

    #[must_use]
    pub fn gif_duration(mut self, val: Seconds) -> Self {
        self.gif_duration = Some(val);
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
        self.title = Some(val.into());
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
