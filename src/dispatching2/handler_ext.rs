use crate::{dispatching2::HandlerFactory, types::Message, utils::command::BotCommand};
use dptree::{di::DependencyMap, Handler};
use teloxide_core::types::Me;

pub trait HandlerExt<Output> {
    #[must_use]
    fn add_command<C>(self) -> Self
    where
        C: BotCommand + Send + Sync + 'static;

    #[must_use]
    fn dispatch_by<F>(self) -> Self
    where
        F: HandlerFactory<Out = Output>;
}

impl<Output> HandlerExt<Output> for Handler<'static, DependencyMap, Output>
where
    Output: Send + Sync + 'static,
{
    fn add_command<C>(self) -> Self
    where
        C: BotCommand + Send + Sync + 'static,
    {
        self.chain(dptree::filter_map(move |message: Message, me: Me| {
            let bot_name = me.user.username.expect("Bots must have a username");
            async move { message.text().and_then(|text| C::parse(text, bot_name).ok()) }
        }))
    }

    fn dispatch_by<F>(self) -> Self
    where
        F: HandlerFactory<Out = Output>,
    {
        self.chain(F::handler())
    }
}
