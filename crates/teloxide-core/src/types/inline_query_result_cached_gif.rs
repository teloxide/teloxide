use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, MessageEntity, ParseMode};

/// Represents a link to an animated GIF file stored on the Telegram servers.
///
/// By default, this animated GIF file will be sent by the user with an optional
/// caption. Alternatively, you can use `input_message_content` to send a
/// message with specified content instead of the animation.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultcachedgif).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultCachedGif {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid file identifier for the GIF file.
    pub gif_file_id: String,

    /// Title for the result.
    pub title: Option<String>,

    /// Caption of the GIF file to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [`ParseMode::Markdown`] or [`ParseMode::Html`], if you want
    /// Telegram apps to show [bold, italic, fixed-width text or inline
    /// URLs] in the media caption.
    ///
    /// [`ParseMode::Markdown`]: crate::types::ParseMode::Markdown
    /// [`ParseMode::Html`]: crate::types::ParseMode::Html
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

impl InlineQueryResultCachedGif {
    pub fn new<S1, S2>(id: S1, gif_file_id: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            gif_file_id: gif_file_id.into(),
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: false,
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

    pub fn gif_file_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.gif_file_id = val.into();
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
