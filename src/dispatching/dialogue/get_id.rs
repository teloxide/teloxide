use crate::{types::Message, utils::command::BotCommand};

/// Something that has an ID.
pub trait GetId {
    #[must_use]
    fn id(&self) -> i32;
}

impl GetId for Message {
    fn id(&self) -> i32 {
        self.id
    }
}

impl<Cmd> GetId for (Message, Cmd)
where
    Cmd: BotCommand,
{
    fn id(&self) -> i32 {
        self.0.id()
    }
}
