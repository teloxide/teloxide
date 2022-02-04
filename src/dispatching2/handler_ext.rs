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

use std::fmt::Debug;

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

    /// Passes [`Dialogue<D, S>`] and `D` as handler dependencies.
    ///
    /// It does so by the following steps:
    ///
    ///  1. If an incoming update has no chat ID ([`GetChatId::chat_id`] returns
    /// `None`), the rest of the chain will not be executed. Otherwise, passes
    /// `Dialogue::new(storage, chat_id)` forwards.
    ///  2. If [`Dialogue::get_or_default`] on the passed dialogue returns `Ok`,
    /// passes the dialogue state forwards. Otherwise, logs an error and the
    /// rest of the chain is not executed.
    ///
    /// ## Dependency requirements
    ///
    ///  - `Arc<S>`
    ///  - `Upd`
    ///
    /// [`Dialogue<D, S>`]: Dialogue
    #[must_use]
    fn enter_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + Send + Sync + 'static,
        <S as Storage<D>>::Error: Debug + Send,
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
            message.text().and_then(|text| C::parse(text, bot_name).ok())
        }))
    }

    fn enter_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + Send + Sync + 'static,
        <S as Storage<D>>::Error: Debug + Send,
        D: Default + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static,
    {
        self.chain(dptree::filter_map(|storage: Arc<S>, upd: Upd| {
            let chat_id = upd.chat_id()?;
            Some(Dialogue::new(storage, chat_id))
        }))
        .chain(dptree::filter_map_async(|dialogue: Dialogue<D, S>| async move {
            match dialogue.get_or_default().await {
                Ok(dialogue) => Some(dialogue),
                Err(err) => {
                    log::error!("dialogue.get_or_default() failed: {:?}", err);
                    None
                }
            }
        }))
    }

    fn dispatch_by<F>(self) -> Self
    where
        F: HandlerFactory<Out = Output>,
    {
        self.chain(F::handler())
    }
}
