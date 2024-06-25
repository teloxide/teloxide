use crate::{
    dispatching::{
        dialogue::{GetChatId, Storage},
        DpHandlerDescription,
    },
    types::{Me, Message},
    utils::command::BotCommands,
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
        C: BotCommands + Send + Sync + 'static;

    /// Passes [`Dialogue<D, S>`] and `D` as handler dependencies.
    ///
    /// It does so by the following steps:
    ///
    ///  1. If an incoming update has no chat ID ([`GetChatId::chat_id`] returns
    ///     `None`), the rest of the chain will not be executed. Otherwise,
    ///     passes `Dialogue::new(storage, chat_id)` forwards.
    ///  2. If [`Dialogue::get_or_default`] on the passed dialogue returns `Ok`,
    ///     passes the dialogue state forwards. Otherwise, logs an error and the
    ///     rest of the chain is not executed.
    ///
    /// ## Dependency requirements
    ///
    ///  - `Arc<S>`
    ///  - `Upd`
    ///
    /// [`Dialogue<D, S>`]: super::dialogue::Dialogue
    /// [`Dialogue::get_or_default`]: super::dialogue::Dialogue::get_or_default
    #[must_use]
    fn enter_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + ?Sized + Send + Sync + 'static,
        <S as Storage<D>>::Error: Debug + Send,
        D: Default + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static;
}

impl<Output> HandlerExt<Output> for Handler<'static, DependencyMap, Output, DpHandlerDescription>
where
    Output: Send + Sync + 'static,
{
    fn filter_command<C>(self) -> Self
    where
        C: BotCommands + Send + Sync + 'static,
    {
        self.chain(filter_command::<C, Output>())
    }

    fn enter_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + ?Sized + Send + Sync + 'static,
        <S as Storage<D>>::Error: Debug + Send,
        D: Default + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static,
    {
        self.chain(super::dialogue::enter::<Upd, S, D, Output>())
    }
}

/// Returns a handler that accepts a parsed command `C`.
///
/// A call to this function is the same as `dptree::entry().filter_command()`.
///
/// See [`HandlerExt::filter_command`].
///
/// ## Dependency requirements
///
///  - [`crate::types::Message`]
///  - [`crate::types::Me`]
#[must_use]
pub fn filter_command<C, Output>() -> Handler<'static, DependencyMap, Output, DpHandlerDescription>
where
    C: BotCommands + Send + Sync + 'static,
    Output: Send + Sync + 'static,
{
    dptree::filter_map(move |message: Message, me: Me| {
        let bot_name = me.user.username.expect("Bots must have a username");
        message.text().and_then(|text| C::parse(text, &bot_name).ok())
    })
}
