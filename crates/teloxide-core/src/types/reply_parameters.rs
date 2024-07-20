use serde::{Deserialize, Serialize};

use crate::types::{MessageId, Recipient};

/// Describes reply parameters for the message that is being sent.
#[serde_with::skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to in the current chat,
    /// or in the chat _chat\_id_ if it is specified
    #[serde(flatten)]
    pub message_id: MessageId,
    /// If the message to be replied to is from a different chat, unique
    /// identifier for the chat or username of the channel (in the format
    /// `@channelusername`)
    pub chat_id: Option<Recipient>,
    /// Pass _true_ if the message should be sent even if the specified message
    /// to be replied to is not found; can be used only for replies in the
    /// same chat and forum topic.
    pub allow_sending_without_reply: Option<bool>,
    /// Quoted part of the message to be replied to; 0-1024 characters after
    /// entities parsing. The quote must be an exact substring of the message to
    /// be replied to, including _bold_, _italic_, _underline_, _strikethrough_,
    /// _spoiler_, and _custom_emoji_ entities. The message will fail to send if
    /// the quote isn't found in the original message.
    pub quote: Option<String>,
}

impl ReplyParameters {
    pub fn new(message_id: MessageId) -> Self {
        Self { message_id, ..Self::default() }
    }

    /// Setter for the `chat_id` field
    pub fn chat_id(mut self, chat_id: Recipient) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    /// Sets the `allow_sending_without_reply_field` to _true_
    pub fn allow_sending_without_reply(mut self) -> Self {
        self.allow_sending_without_reply = Some(true);
        self
    }

    /// Setter for the `quote` field
    pub fn quote(mut self, quote: String) -> Self {
        self.quote = Some(quote);
        self
    }
}
