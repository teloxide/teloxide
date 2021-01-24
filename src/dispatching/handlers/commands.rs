use crate::{
    dispatching::{core::FromContext, dispatcher_context::DispatcherContext},
    types::Message,
    utils::command::BotCommand,
};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct Command<C> {
    pub command: C,
}

impl<C> Deref for Command<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.command
    }
}

impl<C> FromContext<Message> for Command<C>
where
    C: BotCommand,
{
    fn from_context(cx: &DispatcherContext<Message>) -> Option<Self> {
        let text = cx.upd.text()?;
        C::parse(text, cx.bot_name.as_ref()).ok().map(|c| Command { command: c })
    }
}
