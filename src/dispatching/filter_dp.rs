//! A dispatcher based on filters.

use futures::StreamExt;

use crate::{
    dispatching::{filters::Filter, ErrorHandler, Handler, Updater},
    types::{
        CallbackQuery, ChosenInlineResult, InlineQuery, Message, Update,
        UpdateKind,
    },
};
use either::Either;

type FilterWithHandler<'a, T, E> =
    (Box<dyn Filter<T> + 'a>, Box<dyn Handler<T, E> + 'a>);
type FiltersWithHandlers<'a, T, E> = Vec<FilterWithHandler<'a, T, E>>;

/// A dispatcher based on filters.
///
/// It consists of:
///  1. [`ErrorHandler`] than handles errors both from [`Updater`] and
/// [`Handler`].
///  2. Filters and handlers.
///
/// First you register filters and handlers using the methods defined below, and
/// then you call [`.dispatch(updater)`]. Filters and handlers are executed in
/// order of registering. The following flowchart represents how this dispatcher
/// acts:
///
/// <div align="center">
///     <img src="https://github.com/teloxide/teloxide/blob/dev/media/FILTER_DP_FLOWCHART.png" width="700" />
/// </div>
///
/// ## Examples
///
/// The simplest example:
/// ```no_run
/// # async fn run() {
/// use std::convert::Infallible;
///
/// use teloxide::{
///     dispatching::{updaters::polling_default, FilterDispatcher},
///     types::Message,
///     Bot,
/// };
///
/// async fn handle_edited_message(mes: Message) -> Result<(), Infallible> {
///     println!("Edited message: {:?}", mes);
///     Ok(())
/// }
///
/// let bot = Bot::new("TOKEN");
///
/// // Create a dispatcher which handlers can't fail with the
/// // error handler that just ignores all errors (that can't ever happen).
/// let mut dp = FilterDispatcher::<Infallible, _>::new(|_| async {})
///     // Add a handler, which handles all messages sent to the bot.
///     .message_handler(true, |mes: Message| async move {
///         println!("New message: {:?}", mes);
///         Ok(())
///     })
///     // Add a handler, which handles all messages edited in a chat
///     // with the bot.
///     .edited_message_handler(true, handle_edited_message);
///
/// // Start dispatching updates using long polling.
/// dp.dispatch(polling_default(&bot)).await;
/// # }
/// ```
///
/// [`std::fmt::Debug`]: std::fmt::Debug
/// [updater]: crate::dispatching::updater
/// [`.dispatch(updater)`]: FilterDispatcher::dispatch
pub struct FilterDispatcher<'a, E, Eh> {
    message_handlers: FiltersWithHandlers<'a, Message, E>,
    edited_message_handlers: FiltersWithHandlers<'a, Message, E>,
    channel_post_handlers: FiltersWithHandlers<'a, Message, E>,
    edited_channel_post_handlers: FiltersWithHandlers<'a, Message, E>,
    inline_query_handlers: FiltersWithHandlers<'a, InlineQuery, E>,
    chosen_inline_result_handlers:
        FiltersWithHandlers<'a, ChosenInlineResult, E>,
    callback_query_handlers: FiltersWithHandlers<'a, CallbackQuery, E>,
    error_handler: Eh,
}

impl<'a, HandlerE, Eh> FilterDispatcher<'a, HandlerE, Eh> {
    pub fn new<UpdaterE>(error_handler: Eh) -> Self
    where
        Eh: ErrorHandler<Either<UpdaterE, HandlerE>>,
    {
        FilterDispatcher {
            message_handlers: Vec::new(),
            edited_message_handlers: Vec::new(),
            channel_post_handlers: Vec::new(),
            edited_channel_post_handlers: Vec::new(),
            inline_query_handlers: Vec::new(),
            chosen_inline_result_handlers: Vec::new(),
            callback_query_handlers: Vec::new(),
            error_handler,
        }
    }

    pub fn message_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<Message, HandlerE> + 'a,
    {
        self.message_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn edited_message_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<Message, HandlerE> + 'a,
    {
        self.edited_message_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn channel_post_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<Message, HandlerE> + 'a,
    {
        self.channel_post_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn edited_channel_post_handler<F, H>(
        mut self,
        filter: F,
        handler: H,
    ) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<Message, HandlerE> + 'a,
    {
        self.edited_channel_post_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn inline_query_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<InlineQuery> + 'a,
        H: Handler<InlineQuery, HandlerE> + 'a,
    {
        self.inline_query_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn chosen_inline_result_handler<F, H>(
        mut self,
        filter: F,
        handler: H,
    ) -> Self
    where
        F: Filter<ChosenInlineResult> + 'a,
        H: Handler<ChosenInlineResult, HandlerE> + 'a,
    {
        self.chosen_inline_result_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn callback_query_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<CallbackQuery> + 'a,
        H: Handler<CallbackQuery, HandlerE> + 'a,
    {
        self.callback_query_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub async fn dispatch<UpdaterE, U>(&mut self, updater: U)
    where
        U: Updater<UpdaterE> + 'a,
        Eh: ErrorHandler<Either<UpdaterE, HandlerE>>,
    {
        updater
            .for_each_concurrent(None, |res| async {
                let Update { kind, id } = match res {
                    Ok(upd) => upd,
                    Err(err) => {
                        self.error_handler
                            .handle_error(Either::Left(err))
                            .await;
                        return;
                    }
                };

                log::debug!(
                    "Handled update#{id:?}: {kind:?}",
                    id = id,
                    kind = kind
                );

                match kind {
                    UpdateKind::Message(mes) => {
                        Self::handle(
                            mes,
                            &self.message_handlers,
                            &self.error_handler,
                        )
                        .await
                    }
                    UpdateKind::EditedMessage(mes) => {
                        Self::handle(
                            mes,
                            &self.edited_message_handlers,
                            &self.error_handler,
                        )
                        .await;
                    }
                    UpdateKind::ChannelPost(post) => {
                        Self::handle(
                            post,
                            &self.channel_post_handlers,
                            &self.error_handler,
                        )
                        .await;
                    }
                    UpdateKind::EditedChannelPost(post) => {
                        Self::handle(
                            post,
                            &self.edited_channel_post_handlers,
                            &self.error_handler,
                        )
                        .await;
                    }
                    UpdateKind::InlineQuery(query) => {
                        Self::handle(
                            query,
                            &self.inline_query_handlers,
                            &self.error_handler,
                        )
                        .await;
                    }
                    UpdateKind::ChosenInlineResult(result) => {
                        Self::handle(
                            result,
                            &self.chosen_inline_result_handlers,
                            &self.error_handler,
                        )
                        .await;
                    }
                    UpdateKind::CallbackQuery(callback) => {
                        Self::handle(
                            callback,
                            &self.callback_query_handlers,
                            &self.error_handler,
                        )
                        .await;
                    }
                }
            })
            .await;
    }

    async fn handle<T, UpdaterE>(
        update: T,
        handlers: &FiltersWithHandlers<'a, T, HandlerE>,
        error_handler: &Eh,
    ) where
        T: std::fmt::Debug,
        Eh: ErrorHandler<Either<UpdaterE, HandlerE>>,
    {
        for x in handlers {
            if x.0.test(&update) {
                if let Err(err) = x.1.handle(update).await {
                    error_handler.handle_error(Either::Right(err)).await
                }

                return;
            }
        }

        log::warn!("unhandled update {:?}", update);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        convert::Infallible,
        sync::atomic::{AtomicI32, Ordering},
    };

    use crate::{
        dispatching::{FilterDispatcher, Updater},
        types::{
            Chat, ChatKind, ForwardKind, MediaKind, Message, MessageKind,
            Sender, Update, UpdateKind, User,
        },
    };

    #[tokio::test]
    async fn first_handler_executes_1_time() {
        let counter = &AtomicI32::new(0);
        let counter2 = &AtomicI32::new(0);

        let mut dp = FilterDispatcher::<Infallible, _>::new(|_| async {})
            .message_handler(true, |_mes: Message| async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok::<_, Infallible>(())
            })
            .message_handler(true, |_mes: Message| async move {
                counter2.fetch_add(1, Ordering::SeqCst);
                Ok::<_, Infallible>(())
            });

        dp.dispatch(one_message_updater()).await;

        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert_eq!(counter2.load(Ordering::SeqCst), 0);
    }

    fn message() -> Message {
        Message {
            id: 6534,
            date: 1_567_898_953,
            chat: Chat {
                id: 218_485_655,
                photo: None,
                kind: ChatKind::Private {
                    type_: (),
                    first_name: Some("W".to_string()),
                    last_name: None,
                    username: Some("WaffleLapkin".to_string()),
                },
            },
            kind: MessageKind::Common {
                from: Sender::User(User {
                    id: 457_569_668,
                    is_bot: true,
                    first_name: "BT".to_string(),
                    last_name: None,
                    username: Some("BloodyTestBot".to_string()),
                    language_code: None,
                }),
                forward_kind: ForwardKind::Origin {
                    reply_to_message: None,
                },
                edit_date: None,
                media_kind: MediaKind::Text {
                    text: "text".to_string(),
                    entities: vec![],
                },
                reply_markup: None,
            },
        }
    }

    fn message_update() -> Update {
        Update {
            id: 0,
            kind: UpdateKind::Message(message()),
        }
    }

    fn one_message_updater() -> impl Updater<Infallible> {
        use futures::{future::ready, stream};

        stream::once(ready(Ok(message_update())))
    }
}
