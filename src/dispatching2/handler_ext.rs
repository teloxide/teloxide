use crate::{
    dispatching2::HandlerFactory,
    types::{Me, Message},
    utils::command::BotCommand,
};
use dptree::{di::DependencyMap, Handler};

/// Extension methods for working with `dptree` handlers.
pub trait HandlerExt<Output> {
    /// Returns a handler that accepts a parsed command `C`.
    ///
    /// ## Dependency requirements
    ///
    ///  - [`crate::types::Message`]
    ///  - [`crate::types::Me`]
    #[must_use]
    fn filter_command<C>(self) -> Self
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
    fn filter_command<C>(self) -> Self
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
