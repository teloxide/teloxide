use crate::types::CallbackQuery;
use teloxide_core::types::{ChatId, Message};

/// Something that may has a chat ID.
pub trait GetChatId {
    #[must_use]
    fn chat_id(&self) -> Option<ChatId>;
}

impl GetChatId for Message {
    fn chat_id(&self) -> Option<ChatId> {
        Some(self.chat.id)
    }
}

impl GetChatId for CallbackQuery {
    fn chat_id(&self) -> Option<ChatId> {
        self.message.as_ref().map(|mes| mes.chat.id)
    }
}
