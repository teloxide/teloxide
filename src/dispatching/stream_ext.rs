use super::UpdateWithCx;
use crate::{
    error_handlers::{ErrorHandler, LoggingErrorHandler},
    prelude::{Message, Update},
    types::UpdateKind,
    utils::command::BotCommand,
    Bot,
};
use futures::{stream::BoxStream, Stream, StreamExt as _};
use std::{fmt::Debug, sync::Arc};

/// Common methods for working with streams.
pub trait StreamExt {
    fn with_bot<Upd, E>(
        self,
        bot: Arc<Bot>,
    ) -> BoxStream<'static, Result<UpdateWithCx<Upd>, E>>
    where
        Self: Stream<Item = Result<Upd, E>>;

    fn error_handler<Upd, Eh, E>(
        self,
        error_handler: Arc<Eh>,
    ) -> BoxStream<'static, Upd>
    where
        Self: Stream<Item = Result<Upd, E>>,
        Eh: ErrorHandler<E> + Send + Sync + 'static,
        E: Send + 'static,
        Upd: Send + 'static;

    /// Sets a default logging handler and returns only messages.
    fn basic_config<E>(self) -> BoxStream<'static, UpdateWithCx<Message>>
    where
        Self: Stream<Item = Result<UpdateWithCx<Update>, E>>,
        E: Debug + Send + 'static;

    /// Extracts only text messages from this stream of arbitrary messages.
    fn text_messages(
        self,
    ) -> BoxStream<'static, (UpdateWithCx<Message>, String)>
    where
        Self: Stream<Item = UpdateWithCx<Message>>;

    /// Extracts only commands with their arguments from this stream of
    /// arbitrary messages.
    fn commands<C, N>(
        self,
        bot_name: N,
    ) -> BoxStream<'static, (UpdateWithCx<Message>, C, Vec<String>)>
    where
        Self: Stream<Item = UpdateWithCx<Message>>,
        C: BotCommand,
        N: Into<String> + Send;
}

impl<S> StreamExt for S
where
    S: Send + 'static,
{
    fn with_bot<Upd, E>(
        self,
        bot: Arc<Bot>,
    ) -> BoxStream<'static, Result<UpdateWithCx<Upd>, E>>
    where
        Self: Stream<Item = Result<Upd, E>>,
    {
        self.map(move |result| {
            result.map(|update| UpdateWithCx::new(Arc::clone(&bot), update))
        })
        .boxed()
    }

    fn error_handler<Upd, Eh, E>(
        self,
        error_handler: Arc<Eh>,
    ) -> BoxStream<'static, Upd>
    where
        Self: Stream<Item = Result<Upd, E>>,
        Eh: ErrorHandler<E> + Send + Sync + 'static,
        E: Send + 'static,
        Upd: Send + 'static,
    {
        self.filter_map(move |result| {
            let error_handler = Arc::clone(&error_handler);

            async {
                match result {
                    Err(error) => {
                        error_handler.handle_error(error).await;
                        None
                    }
                    Ok(ok) => Some(ok),
                }
            }
        })
        .boxed()
    }
    fn basic_config<E>(self) -> BoxStream<'static, UpdateWithCx<Message>>
    where
        Self: Stream<Item = Result<UpdateWithCx<Update>, E>>,
        E: Debug + Send + 'static,
    {
        self.error_handler(LoggingErrorHandler::with_custom_text(
            "An error from UpdateListener",
        ))
        .filter_map(|update_with_cx| async {
            let UpdateWithCx { bot, update } = update_with_cx;

            match update.kind {
                UpdateKind::Message(message) => {
                    Some(UpdateWithCx::new(bot, message))
                }
                _ => None,
            }
        })
        .boxed()
    }

    fn text_messages(
        self,
    ) -> BoxStream<'static, (UpdateWithCx<Message>, String)>
    where
        Self: Stream<Item = UpdateWithCx<Message>>,
    {
        self.filter_map(|cx| async move {
            cx.update.text_owned().map(|text| (cx, text))
        })
        .boxed()
    }

    fn commands<C, N>(
        self,
        bot_name: N,
    ) -> BoxStream<'static, (UpdateWithCx<Message>, C, Vec<String>)>
    where
        Self: Stream<Item = UpdateWithCx<Message>>,
        C: BotCommand,
        N: Into<String> + Send,
    {
        let bot_name = bot_name.into();

        self.text_messages()
            .filter_map(move |(cx, text)| {
                let bot_name = bot_name.clone();

                async move {
                    C::parse(&text, &bot_name).map(|(command, args)| {
                        (
                            cx,
                            command,
                            args.into_iter()
                                .map(ToOwned::to_owned)
                                .collect::<Vec<String>>(),
                        )
                    })
                }
            })
            .boxed()
    }
}
