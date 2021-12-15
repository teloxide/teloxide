use crate::types::CallbackQuery;
use teloxide_core::types::Message;

/// Something that maybe has a chat ID.
pub trait GetChatId {
    #[must_use]
    fn chat_id(&self) -> Option<i64>;
}

impl GetChatId for Message {
    fn chat_id(&self) -> Option<i64> {
        Some(self.chat.id)
    }
}

impl GetChatId for CallbackQuery {
    fn chat_id(&self) -> Option<i64> {
        self.message.as_ref().map(|mes| mes.chat.id)
    }
}
