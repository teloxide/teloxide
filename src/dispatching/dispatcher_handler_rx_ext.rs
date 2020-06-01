use crate::{
    prelude::DispatcherHandlerCx, types::Message, utils::command::BotCommand,
};
use futures::{stream::BoxStream, Stream, StreamExt};

/// An extension trait to be used with [`DispatcherHandlerRx`].
///
/// [`DispatcherHandlerRx`]: crate::dispatching::DispatcherHandlerRx
pub trait DispatcherHandlerRxExt {
    /// Extracts only text messages from this stream of arbitrary messages.
    fn text_messages(
        self,
    ) -> BoxStream<'static, (DispatcherHandlerCx<Message>, String)>
    where
        Self: Stream<Item = DispatcherHandlerCx<Message>>;

    /// Extracts only commands with their arguments from this stream of
    /// arbitrary messages.
    fn commands<C, N>(
        self,
        bot_name: N,
    ) -> BoxStream<'static, (DispatcherHandlerCx<Message>, C)>
    where
        Self: Stream<Item = DispatcherHandlerCx<Message>>,
        C: BotCommand,
        N: Into<String> + Send;
}

impl<T> DispatcherHandlerRxExt for T
where
    T: Send + 'static,
{
    fn text_messages(
        self,
    ) -> BoxStream<'static, (DispatcherHandlerCx<Message>, String)>
    where
        Self: Stream<Item = DispatcherHandlerCx<Message>>,
    {
        Box::pin(self.filter_map(|cx| async move {
            cx.update.text_owned().map(|text| (cx, text))
        }))
    }

    fn commands<C, N>(
        self,
        bot_name: N,
    ) -> BoxStream<'static, (DispatcherHandlerCx<Message>, C)>
    where
        Self: Stream<Item = DispatcherHandlerCx<Message>>,
        C: BotCommand,
        N: Into<String> + Send,
    {
        let bot_name = bot_name.into();

        Box::pin(self.text_messages().filter_map(move |(cx, text)| {
            let bot_name = bot_name.clone();

            async move {
                C::parse(&text, &bot_name).map(|command| (cx, command)).ok()
            }
        }))
    }
}
