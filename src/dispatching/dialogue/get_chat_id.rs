use crate::{types::Message, utils::command::BotCommand};

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

impl<Cmd> GetChatId for (Message, Cmd)
where
    Cmd: BotCommand,
{
    fn chat_id(&self) -> i64 {
        self.0.chat_id()
    }
}
