use serde::{Deserialize, Serialize};

use crate::types::{ChecklistTaskId, MessageEntity, MessageId, Recipient};

/// Describes reply parameters for the message that is being sent.
#[serde_with::skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to in the current chat,
    /// or in the chat _chat\_id_ if it is specified
    // Issue https://github.com/teloxide/teloxide/issues/1135
    #[serde(with = "crate::types::msg_id_as_int")]
    pub message_id: MessageId,
    /// If the message to be replied to is from a different chat, unique
    /// identifier for the chat or username of the channel (in the format
    /// `@channelusername`). Not supported for messages sent on behalf of a
    /// business account.
    pub chat_id: Option<Recipient>,
    /// Pass _true_ if the message should be sent even if the specified message
    /// to be replied to is not found; can be used only for replies in the
    /// same chat and forum topic. Always `false` for replies in another chat or
    /// forum topic. Always `true` for messages sent on behalf of a business
    /// account.
    pub allow_sending_without_reply: Option<bool>,
    /// Quoted part of the message to be replied to; 0-1024 characters after
    /// entities parsing. The quote must be an exact substring of the message to
    /// be replied to, including _bold_, _italic_, _underline_, _strikethrough_,
    /// _spoiler_, and _custom_emoji_ entities. The message will fail to send if
    /// the quote isn't found in the original message.
    pub quote: Option<String>,
    /// Mode for parsing entities in the quote. See [formatting options] for
    /// more details.
    ///
    /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
    pub quote_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the quote. It
    /// can be specified instead of quote_parse_mode.
    pub quote_entities: Option<Vec<MessageEntity>>,
    /// Position of the quote in the original message in UTF-16 code units
    pub quote_position: Option<u32>,
    /// Identifier of the specific checklist task to be replied to
    pub checklist_task_id: Option<ChecklistTaskId>,
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

    /// Setter for the `checklist_task_id` field
    pub fn checklist_task_id(mut self, checklist_task_id: ChecklistTaskId) -> Self {
        self.checklist_task_id = Some(checklist_task_id);
        self
    }
}
