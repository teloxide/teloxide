use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a link to an article or web page.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultarticle).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 Bytes.
    pub id: String,

    /// Title of the result.
    pub title: String,

    /// Content of the message to be sent.
    pub input_message_content: InputMessageContent,

    /// Inline keyboard attached to the message.
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// URL of the result.
    pub url: Option<reqwest::Url>,

    /// Short description of the result.
    pub description: Option<String>,

    /// Url of the thumbnail for the result.
    pub thumbnail_url: Option<reqwest::Url>,

    /// Thumbnail width.
    pub thumbnail_width: Option<u32>,

    /// Thumbnail height.
    pub thumbnail_height: Option<u32>,
}

impl InlineQueryResultArticle {
    pub fn new<S1, S2>(id: S1, title: S2, input_message_content: InputMessageContent) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            input_message_content,
            reply_markup: None,
            url: None,
            description: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    #[must_use]
    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = val;
        self
    }

    #[must_use]
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    #[must_use]
    pub fn url(mut self, val: reqwest::Url) -> Self {
        self.url = Some(val);
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
    pub fn thumbnail_url(mut self, val: reqwest::Url) -> Self {
        self.thumbnail_url = Some(val);
        self
    }

    #[must_use]
    pub fn thumbnail_width(mut self, val: u32) -> Self {
        self.thumbnail_width = Some(val);
        self
    }

    #[must_use]
    pub fn thumbnail_height(mut self, val: u32) -> Self {
        self.thumbnail_height = Some(val);
        self
    }
}
