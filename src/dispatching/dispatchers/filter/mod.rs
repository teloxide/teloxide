use futures::StreamExt;

use async_trait::async_trait;

use crate::{
    dispatching::{
        dispatchers::filter::error_policy::ErrorPolicy, filters::Filter,
        handler::Handler, updater::Updater, Dispatcher,
    },
    types::{CallbackQuery, ChosenInlineResult, Message, Update, UpdateKind},
};

pub mod error_policy;

struct FilterAndHandler<'a, T, E> {
    filter: Box<dyn Filter<T> + 'a>,
    handler: Box<dyn Handler<'a, T, E> + 'a>,
}

type FiltersAndHandlers<'a, T, E> = Vec<FilterAndHandler<'a, T, E>>;

/// Dispatcher that dispatches updates from telegram.
///
/// This is 'filter' implementation with following limitations:
/// - Error (`E` generic parameter) _must_ implement [`std::fmt::Debug`]
/// - All 'handlers' are boxed
/// - Handler's fututres are also boxed
/// - [Custom error policy] is also boxed
/// - All errors from [updater] are ignored (TODO: remove this limitation)
/// - All handlers executed in order (this means that in dispatching have 2
///   upadtes it will first execute some handler into complition with first
///   update and **then** search for handler for second update, this is probably
///   wrong)
///
/// ## Examples
///
/// Simplest example:
/// ```no_run
/// # async fn run() {
/// use std::convert::Infallible;
///
/// use telebofr::{
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
/// let mut dp = FilterDispatcher::<Infallible, _>::new(|_| async { () })
///     // Add 'handler' that will handle all messages sent to the bot
///     .message_handler(true, |mes: Message| {
///         async move { println!("New message: {:?}", mes) }
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
/// [Custom error policy]:
/// crate::dispatching::filter::error_policy::ErrorPolicy::Custom [updater]:
/// crate::dispatching::updater
pub struct FilterDispatcher<'a, E, Ep> {
    message_handlers: FiltersAndHandlers<'a, Message, E>,
    edited_message_handlers: FiltersAndHandlers<'a, Message, E>,
    channel_post_handlers: FiltersAndHandlers<'a, Message, E>,
    edited_channel_post_handlers: FiltersAndHandlers<'a, Message, E>,
    inline_query_handlers: FiltersAndHandlers<'a, (), E>,
    chosen_inline_result_handlers:
        FiltersAndHandlers<'a, ChosenInlineResult, E>,
    callback_query_handlers: FiltersAndHandlers<'a, CallbackQuery, E>,
    error_policy: Ep,
}

impl<'a, E, Ep> FilterDispatcher<'a, E, Ep>
where
    Ep: ErrorPolicy<E>,
    E: std::fmt::Debug, // TODO: Is this really necessary?
{
    pub fn new(error_policy: Ep) -> Self {
        FilterDispatcher {
            message_handlers: Vec::new(),
            edited_message_handlers: Vec::new(),
            channel_post_handlers: Vec::new(),
            edited_channel_post_handlers: Vec::new(),
            inline_query_handlers: Vec::new(),
            chosen_inline_result_handlers: Vec::new(),
            callback_query_handlers: Vec::new(),
            error_policy,
        }
    }

    pub fn message_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<'a, Message, E> + 'a,
    {
        self.message_handlers.push(FilterAndHandler {
            filter: Box::new(filter),
            handler: Box::new(handler),
        });
        self
    }

    pub fn edited_message_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<'a, Message, E> + 'a,
    {
        self.edited_message_handlers.push(FilterAndHandler {
            filter: Box::new(filter),
            handler: Box::new(handler),
        });
        self
    }

    pub fn channel_post_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<'a, Message, E> + 'a,
    {
        self.channel_post_handlers.push(FilterAndHandler {
            filter: Box::new(filter),
            handler: Box::new(handler),
        });
        self
    }

    pub fn edited_channel_post_handler<F, H>(
        mut self,
        filter: F,
        handler: H,
    ) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<'a, Message, E> + 'a,
    {
        self.edited_channel_post_handlers.push(FilterAndHandler {
            filter: Box::new(filter),
            handler: Box::new(handler),
        });
        self
    }

    pub fn inline_query_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<()> + 'a,
        H: Handler<'a, (), E> + 'a,
    {
        self.inline_query_handlers.push(FilterAndHandler {
            filter: Box::new(filter),
            handler: Box::new(handler),
        });
        self
    }

    pub fn chosen_inline_result_handler<F, H>(
        mut self,
        filter: F,
        handler: H,
    ) -> Self
    where
        F: Filter<ChosenInlineResult> + 'a,
        H: Handler<'a, ChosenInlineResult, E> + 'a,
    {
        self.chosen_inline_result_handlers.push(FilterAndHandler {
            filter: Box::new(filter),
            handler: Box::new(handler),
        });
        self
    }

    pub fn callback_query_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<CallbackQuery> + 'a,
        H: Handler<'a, CallbackQuery, E> + 'a,
    {
        self.callback_query_handlers.push(FilterAndHandler {
            filter: Box::new(filter),
            handler: Box::new(handler),
        });
        self
    }

    // TODO: Can someone simplify this?
    pub async fn dispatch<U>(&mut self, updates: U)
    where
        U: Updater + 'a,
    {
        updates
            .for_each(|res| {
                async {
                    let Update { kind, id } = match res {
                        Ok(upd) => upd,
                        _ => return, // TODO: proper error handling
                    };

                    log::debug!(
                        "Handled update#{id:?}: {kind:?}",
                        id = id,
                        kind = kind
                    );

                    match kind {
                        UpdateKind::Message(mes) => {
                            self.handle(mes, &self.message_handlers).await
                        }
                        UpdateKind::EditedMessage(mes) => {
                            self.handle(mes, &self.edited_message_handlers)
                                .await;
                        }
                        UpdateKind::ChannelPost(post) => {
                            self.handle(post, &self.channel_post_handlers)
                                .await;
                        }
                        UpdateKind::EditedChannelPost(post) => {
                            self.handle(
                                post,
                                &self.edited_channel_post_handlers,
                            )
                            .await;
                        }
                        UpdateKind::InlineQuery(query) => {
                            self.handle(query, &self.inline_query_handlers)
                                .await;
                        }
                        UpdateKind::ChosenInlineResult(result) => {
                            self.handle(
                                result,
                                &self.chosen_inline_result_handlers,
                            )
                            .await;
                        }
                        UpdateKind::CallbackQuery(callback) => {
                            self.handle(
                                callback,
                                &self.callback_query_handlers,
                            )
                            .await;
                        }
                    }
                }
            })
            .await;
    }

    async fn handle<T>(
        &self,
        update: T,
        handlers: &FiltersAndHandlers<'a, T, E>,
    ) where
        T: std::fmt::Debug,
    {
        for x in handlers {
            if x.filter.test(&update) {
                if let Err(err) = x.handler.handle(update).await {
                    self.error_policy.handle_error(err).await
                }

                return;
            }
        }

        log::warn!("unhandled update {:?}", update);
    }
}

#[async_trait(? Send)]
impl<'a, U, E, Ep> Dispatcher<'a, U> for FilterDispatcher<'a, E, Ep>
where
    E: std::fmt::Debug,
    U: Updater + 'a,
    Ep: ErrorPolicy<E>,
{
    async fn dispatch(&'a mut self, updater: U) {
        FilterDispatcher::dispatch(self, updater).await
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

        let mut dp = FilterDispatcher::<Infallible, _>::new(|_| async { () })
            .message_handler(true, |_mes: Message| {
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            })
            .message_handler(true, |_mes: Message| {
                async move {
                    counter2.fetch_add(1, Ordering::SeqCst);
                    Ok::<_, Infallible>(())
                }
            });

        dp.dispatch(one_message_updater()).await;

        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert_eq!(counter2.load(Ordering::SeqCst), 0);
    }

    fn message() -> Message {
        Message {
            id: 6534,
            date: 1567898953,
            chat: Chat {
                id: 218485655,
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
                    id: 457569668,
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
