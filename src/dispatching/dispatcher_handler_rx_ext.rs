use crate::{dispatching::UpdateWithCx, utils::command::BotCommand};
use futures::{stream::BoxStream, Stream, StreamExt};
use teloxide_core::types::Message;

/// An extension trait to be used with [`DispatcherHandlerRx`].
///
/// See the [module-level documentation](crate::dispatching) for the design
/// overview.
///
/// [`DispatcherHandlerRx`]: crate::dispatching::DispatcherHandlerRx
pub trait DispatcherHandlerRxExt<R> {
    /// Extracts only text messages from this stream of arbitrary messages.
    fn text_messages(self) -> BoxStream<'static, (UpdateWithCx<R, Message>, String)>
    where
        Self: Stream<Item = UpdateWithCx<R, Message>>,
        R: Send + 'static;

    /// Extracts only commands with their arguments from this stream of
    /// arbitrary messages.
    fn commands<C, N>(self, bot_name: N) -> BoxStream<'static, (UpdateWithCx<R, Message>, C)>
    where
        Self: Stream<Item = UpdateWithCx<R, Message>>,
        C: BotCommand,
        N: Into<String> + Send,
        R: Send + 'static;
}

impl<R, T> DispatcherHandlerRxExt<R> for T
where
    T: Send + 'static,
{
    fn text_messages(self) -> BoxStream<'static, (UpdateWithCx<R, Message>, String)>
    where
        Self: Stream<Item = UpdateWithCx<R, Message>>,
        R: Send + 'static,
    {
        self.filter_map(|cx| async move { cx.update.text_owned().map(|text| (cx, text)) }).boxed()
    }

    fn commands<C, N>(self, bot_name: N) -> BoxStream<'static, (UpdateWithCx<R, Message>, C)>
    where
        Self: Stream<Item = UpdateWithCx<R, Message>>,
        C: BotCommand,
        N: Into<String> + Send,
        R: Send + 'static,
    {
        let bot_name = bot_name.into();

        self.text_messages()
            .filter_map(move |(cx, text)| {
                let bot_name = bot_name.clone();

                async move { C::parse(&text, &bot_name).map(|command| (cx, command)).ok() }
            })
            .boxed()
    }
}
