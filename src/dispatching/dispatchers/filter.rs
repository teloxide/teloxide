//! A dispatcher based on filters.

use futures::StreamExt;

use crate::{
    dispatching::{filters::Filter, ErrorHandler, Handler, Updater},
    types::{
        CallbackQuery, ChosenInlineResult, InlineQuery, Message, Update,
        UpdateKind,
    },
};

type FilterWithHandler<'a, T, E> =
    (Box<dyn Filter<T> + 'a>, Box<dyn Handler<T, E> + 'a>);
type FiltersWithHandlers<'a, T, E> = Vec<FilterWithHandler<'a, T, E>>;

/// A dispatcher based on filters.
///
/// Filters and handlers are executed in order of registering. The pseudocode
/// looks like this:
///
/// ```
/// for pair in handlers_and_filters {
///     if pair.filter.test(update) {
///         pair.handle(update);
///         return;
///     }
/// }
///
/// log("unhandeled update: " + update);
/// ```
///
/// ## Examples
///
/// Simplest example:
/// ```no_run
/// # async fn run() {
/// use std::convert::Infallible;
///
/// use teloxide::{
///     dispatching::{
///         dispatchers::filter::{
///             error_policy::ErrorPolicy, FilterDispatcher,
///         },
///         updater::polling,
///     },
///     types::Message,
///     Bot,
/// };
///
/// async fn handle_edited_message(mes: Message) {
///     println!("Edited message: {:?}", mes)
/// }
///
/// let bot = Bot::new("TOKEN");
///
/// // create dispatching which handlers can't fail
/// // with error policy that just ignores all errors (that can't ever happen)
/// let mut dp = FilterDispatcher::<Infallible, _>::new(|_| async {})
///     // Add 'handler' that will handle all messages sent to the bot
///     .message_handler(true, |mes: Message| async move {
///         println!("New message: {:?}", mes)
///     })
///     // Add 'handler' that will handle all
///     // messages edited in chat with the bot
///     .edited_message_handler(true, handle_edited_message);
///
/// // Start dispatching updates from long polling
/// dp.dispatch(polling(&bot)).await;
/// # }
/// ```
///
/// [`std::fmt::Debug`]: std::fmt::Debug
/// [updater]: crate::dispatching::updater
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

/// An error produced either from [`Updater`] or [`Handler`].
///
/// [`Updater`]: crate::dispatching::Updater
/// [`Handler`]: crate::dispatching::Handler
pub enum ErrorKind<E1, E2> {
    FromUpdater(E1),
    FromHandler(E2),
}

impl<'a, E2, Eh> FilterDispatcher<'a, E2, Eh> {
    pub fn new<E1>(error_handler: Eh) -> Self
    where
        Eh: ErrorHandler<ErrorKind<E1, E2>>,
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
        H: Handler<Message, E2> + 'a,
    {
        self.message_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn edited_message_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<Message, E2> + 'a,
    {
        self.edited_message_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn channel_post_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<Message, E2> + 'a,
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
        H: Handler<Message, E2> + 'a,
    {
        self.edited_channel_post_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn inline_query_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<InlineQuery> + 'a,
        H: Handler<InlineQuery, E2> + 'a,
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
        H: Handler<ChosenInlineResult, E2> + 'a,
    {
        self.chosen_inline_result_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn callback_query_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<CallbackQuery> + 'a,
        H: Handler<CallbackQuery, E2> + 'a,
    {
        self.callback_query_handlers
            .push((Box::new(filter), Box::new(handler)));
        self
    }

    pub async fn dispatch<E1, U>(&mut self, updater: U)
    where
        U: Updater<E1> + 'a,
        Eh: ErrorHandler<ErrorKind<E1, E2>>,
    {
        updater
            .for_each(|res| async {
                let Update { kind, id } = match res {
                    Ok(upd) => upd,
                    Err(err) => {
                        self.error_handler
                            .handle_error(ErrorKind::FromUpdater(err));
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
                            &mut self.message_handlers,
                            &mut self.error_handler,
                        )
                        .await
                    }
                    UpdateKind::EditedMessage(mes) => {
                        Self::handle(
                            mes,
                            &mut self.edited_message_handlers,
                            &mut self.error_handler,
                        )
                        .await;
                    }
                    UpdateKind::ChannelPost(post) => {
                        Self::handle(
                            post,
                            &mut self.channel_post_handlers,
                            &mut self.error_handler,
                        )
                        .await;
                    }
                    UpdateKind::EditedChannelPost(post) => {
                        Self::handle(
                            post,
                            &mut self.edited_channel_post_handlers,
                            &mut self.error_handler,
                        )
                        .await;
                    }
                    UpdateKind::InlineQuery(query) => {
                        Self::handle(
                            query,
                            &mut self.inline_query_handlers,
                            &mut self.error_handler,
                        )
                        .await;
                    }
                    UpdateKind::ChosenInlineResult(result) => {
                        Self::handle(
                            result,
                            &mut self.chosen_inline_result_handlers,
                            &mut self.error_handler,
                        )
                        .await;
                    }
                    UpdateKind::CallbackQuery(callback) => {
                        Self::handle(
                            callback,
                            &mut self.callback_query_handlers,
                            &mut self.error_handler,
                        )
                        .await;
                    }
                }
            })
            .await;
    }

    async fn handle<T, E1>(
        update: T,
        handlers: &mut FiltersWithHandlers<'a, T, E2>,
        error_handler: &mut Eh,
    ) where
        T: std::fmt::Debug,
        Eh: ErrorHandler<ErrorKind<E1, E2>>,
    {
        for x in handlers {
            if x.0.test(&update) {
                if let Err(err) = x.1.handle(update).await {
                    error_handler
                        .handle_error(ErrorKind::FromHandler(err))
                        .await
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

    use futures::Stream;

    use crate::{
        dispatching::{
            dispatchers::filter::FilterDispatcher, updater::StreamUpdater,
        },
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

    fn one_message_updater(
    ) -> StreamUpdater<impl Stream<Item = Result<Update, Infallible>>> {
        use futures::{future::ready, stream};

        StreamUpdater::new(stream::once(ready(Ok(message_update()))))
    }
}
