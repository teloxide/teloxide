pub mod error_policy;

use crate::{
    dispatcher::{
        filter::Filter,
        handler::Handler,
        updater::Updater,
    },
    types::{
        Update,
        Message,
        UpdateKind,
        CallbackQuery,
        ChosenInlineResult,
    },
};

use futures::StreamExt;
use crate::dispatcher::simple::error_policy::ErrorPolicy;


type Handlers<'a, T, E> = Vec<(Box<dyn Filter<T> + 'a>, Box<dyn Handler<'a, T, E> + 'a>)>;

/// Dispatcher that dispatches updates from telegram.
///
/// This is 'simple' implementation with following limitations:
/// - Error (`E` generic parameter) _must_ implement [`std::fmt::Debug`]
/// - All 'handlers' are boxed
/// - Handler's fututres are also boxed
/// - [Custom error policy] is also boxed
/// - All errors from [updater] are ignored (TODO: remove this limitation)
/// - All handlers executed in order (this means that in dispatcher have
///   2 upadtes it will first execute some handler into complition with
///   first update and **then** search for handler for second update,
///   this is probably wrong)
///
/// ## Examples
///
/// Simplest example:
/// ```no_run
/// # async fn run() {
/// use std::convert::Infallible;
/// use telebofr::{
///     bot::Bot,
///     types::Message,
///     dispatcher::{
///         updater::polling,
///         simple::{Dispatcher, error_policy::ErrorPolicy},
///     },
/// };
///
/// async fn handle_edited_message(mes: Message) {
///     println!("Edited message: {:?}", mes)
/// }
///
/// let bot = Bot::new("TOKEN");
///
/// // create dispatcher which handlers can't fail
/// // with error policy that just ignores all errors (that can't ever happen)
/// let mut dp = Dispatcher::<Infallible>::new(ErrorPolicy::Ignore)
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
/// [Custom error policy]: crate::dispatcher::simple::error_policy::ErrorPolicy::Custom
/// [updater]: crate::dispatcher::updater
pub struct Dispatcher<'a, E> {
    message_handlers: Handlers<'a, Message, E>,
    edited_message_handlers: Handlers<'a, Message, E>,
    channel_post_handlers: Handlers<'a, Message, E>,
    edited_channel_post_handlers: Handlers<'a, Message, E>,
    inline_query_handlers: Handlers<'a, (), E>,
    chosen_inline_result_handlers: Handlers<'a, ChosenInlineResult, E>,
    callback_query_handlers: Handlers<'a, CallbackQuery, E>,
    error_policy: ErrorPolicy<'a, E>,
}

impl<'a, E> Dispatcher<'a, E>
where
    E: std::fmt::Debug, // TODO: Is this really necessary?
{
    pub fn new(error_policy: ErrorPolicy<'a, E>) -> Self {
        Dispatcher {
            message_handlers: Vec::new(),
            edited_message_handlers: Vec::new(),
            channel_post_handlers: Vec::new(),
            edited_channel_post_handlers: Vec::new(),
            inline_query_handlers: Vec::new(),
            chosen_inline_result_handlers: Vec::new(),
            callback_query_handlers: Vec::new(),
            error_policy
        }
    }

    pub fn message_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<'a, Message, E> + 'a,
    {
        self.message_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn edited_message_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<'a, Message, E> + 'a,
    {
        self.edited_message_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn channel_post_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<'a, Message, E> + 'a,
    {
        self.channel_post_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn edited_channel_post_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'a,
        H: Handler<'a, Message, E> + 'a,
    {
        self.edited_channel_post_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn inline_query_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<()> + 'a,
        H: Handler<'a, (), E> + 'a,
    {
        self.inline_query_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn chosen_inline_result_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<ChosenInlineResult> + 'a,
        H: Handler<'a, ChosenInlineResult, E> + 'a,
    {
        self.chosen_inline_result_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn callback_query_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<CallbackQuery> + 'a,
        H: Handler<'a, CallbackQuery, E> + 'a,
    {
        self.callback_query_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    // TODO: Can someone simplify this?
    pub async fn dispatch<U, UE>(&mut self, updates: U)
    where
        U: Updater<UE> + 'a
    {
        updates.for_each(|res| {
            async {
                let res = res;
                let Update { kind, id } = match res {
                    Ok(upd) => upd,
                    _ => return // TODO: proper error handling
                };

                log::debug!("Handled update#{id:?}: {kind:?}", id = id, kind = kind);

                // TODO: can someone extract this to a function?
                macro_rules! call {
                    ($h:expr, $value:expr) => {{
                        let value = $value;
                        let handler = $h.iter().find_map(|e| {
                            let (filter, handler) = e;
                            if filter.test(&value) {
                                Some(handler)
                            } else {
                                None
                            }
                        });

                        match handler {
                            Some(handler) => {
                                 if let Err(err) = handler.handle(value).await {
                                    self.error_policy.handle_error(err).await;
                                 }
                            },
                            None => log::warn!("Unhandled update: {:?}", value)
                        }
                    }};
                }

                match kind {
                    UpdateKind::Message(mes) => call!(self.message_handlers, mes),
                    UpdateKind::EditedMessage(mes) => call!(self.edited_message_handlers, mes),
                    UpdateKind::ChannelPost(post) => call!(self.channel_post_handlers, post),
                    UpdateKind::EditedChannelPost(post) => call!(self.edited_channel_post_handlers, post),
                    UpdateKind::InlineQuery(query) => call!(self.inline_query_handlers, query),
                    UpdateKind::ChosenInlineResult(result) => call!(self.chosen_inline_result_handlers, result),
                    UpdateKind::CallbackQuery(callback) => call!(self.callback_query_handlers, callback),
                }
            }
        })
            .await;
    }
}


#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::sync::atomic::{AtomicI32, Ordering};

    use crate::{
        types::{
            Message, ChatKind, MessageKind, Sender, ForwardKind, MediaKind, Chat, User, Update, UpdateKind
        },
        dispatcher::{simple::{Dispatcher, error_policy::ErrorPolicy}, updater::StreamUpdater},
    };
    use futures::Stream;

    #[tokio::test]
    async fn first_handler_executes_1_time() {
        let counter = &AtomicI32::new(0);
        let counter2 = &AtomicI32::new(0);

        let mut dp = Dispatcher::<Infallible>::new(ErrorPolicy::Ignore)
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
        Update { id: 0, kind: UpdateKind::Message(message()) }
    }

    fn one_message_updater() -> StreamUpdater<impl Stream<Item=Result<Update, Infallible>>> {
        use futures::future::ready;
        use futures::stream;

        StreamUpdater::new(
            stream::once(ready(Ok(message_update())))
        )
    }
}
