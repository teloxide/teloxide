use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, MessageEntity, ParseMode, Seconds};

/// Represents a link to an MP3 audio file. By default, this audio file will be
/// sent by the user.
///
/// Alternatively, you can use `input_message_content` to send a message with
/// the specified content instead of the audio.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultaudio).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultAudio {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the audio file.
    pub audio_url: reqwest::Url,

    /// Title.
    pub title: String,

    /// Caption, 0-1024 characters.
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

    /// Performer.
    pub performer: Option<String>,

    /// Audio duration in seconds.
    pub audio_duration: Option<Seconds>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the audio.
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultAudio {
    pub fn new<S1, S2>(id: S1, audio_url: reqwest::Url, title: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            audio_url,
            title: title.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            performer: None,
            audio_duration: None,
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
    pub fn audio_url(mut self, val: reqwest::Url) -> Self {
        self.audio_url = val;
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

    pub fn performer<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.performer = Some(val.into());
        self
    }

    pub fn audio_duration(mut self, val: Seconds) -> Self {
        self.audio_duration = Some(val);
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
