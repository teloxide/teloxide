use std::sync::Arc;

use crate::{
    dispatching2::{
        dialogue::{Dialogue, GetChatId, Storage},
        HandlerFactory,
    },
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
    fn add_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + Send + Sync + 'static,
        <S as Storage<D>>::Error: Send,
        D: Default + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static;

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

    fn add_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + Send + Sync + 'static,
        <S as Storage<D>>::Error: Send,
        D: Default + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static,
    {
        self.chain(dptree::filter_map(|storage: Arc<S>, upd: Upd| async move {
            let chat_id = upd.chat_id()?;
            Some(Dialogue::new(storage, chat_id))
        }))
        .chain(dptree::filter_map(|dialogue: Dialogue<D, S>| async move {
            dialogue.get_or_default().await.ok()
        }))
    }

    fn dispatch_by<F>(self) -> Self
    where
        F: HandlerFactory<Out = Output>,
    {
        self.chain(F::handler())
    }
}
