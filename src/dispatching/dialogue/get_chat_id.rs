use crate::types::Message;

/// Something that has a chat ID.
pub trait GetChatId {
    #[must_use]
    fn chat_id(&self) -> i64;
}

impl GetChatId for Message {
    fn chat_id(&self) -> i64 {
        self.chat.id
    }
}
