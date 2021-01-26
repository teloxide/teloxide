use crate::{
    dispatching::{core::FromContext, dispatcher_context::DispatcherContext},
    types::Message,
    utils::command::BotCommand,
};
use std::ops::Deref;
use crate::dispatching::core::GetCtx;

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

impl<Ctx, C> FromContext<Ctx> for Command<C>
where
    Ctx: GetCtx<DispatcherContext<Message>>,
    C: BotCommand,
{
    fn from_context(cx: &Ctx) -> Option<Self> {
        let cx = cx.get();
        let text = cx.upd.text()?;
        C::parse(text, cx.bot_name.as_ref()).ok().map(|c| Command { command: c })
    }
}
