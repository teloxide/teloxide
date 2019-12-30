use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a link to an article or web page.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultarticle).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 Bytes.
    pub id: String,

    /// Title of the result.
    pub title: String,

    /// Content of the message to be sent.
    pub input_message_content: InputMessageContent,

    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// URL of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Pass `true`, if you don't want the URL to be shown in the
    /// message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_url: Option<bool>,

    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Url of the thumbnail for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    /// Thumbnail width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i32>,

    /// Thumbnail height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i32>,
}
