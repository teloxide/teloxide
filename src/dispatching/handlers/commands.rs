use crate::{dispatching::core::FromUpd, types::Message, utils::command::BotCommand};

impl<C> FromUpd<Message> for C
where
    C: BotCommand,
{
    fn from_upd(upd: &Message) -> Option<Self> {
        let text = upd.text()?;
        Self::parse(text, "").ok()
    }
}
