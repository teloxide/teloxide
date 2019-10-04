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


pub type Handlers<'a, T, E> = Vec<(Box<dyn Filter<T> + 'a>, Box<dyn Handler<'a, T, E> + 'a>)>;

pub struct Dispatcher<'a, E> {
    message_handlers: Handlers<'a, Message, E>,
    edited_message_handlers: Handlers<'a, Message, E>,
    channel_post_handlers: Handlers<'a, Message, E>,
    edited_channel_post_handlers: Handlers<'a, Message, E>,
    inline_query_handlers: Handlers<'a, (), E>,
    chosen_inline_result_handlers: Handlers<'a, ChosenInlineResult, E>,
    callback_query_handlers: Handlers<'a, CallbackQuery, E>,
}

impl<'a, E> Dispatcher<'a, E> {
    pub fn new() -> Self {
        Dispatcher {
            message_handlers: Vec::new(),
            edited_message_handlers: Vec::new(),
            channel_post_handlers: Vec::new(),
            edited_channel_post_handlers: Vec::new(),
            inline_query_handlers: Vec::new(),
            chosen_inline_result_handlers: Vec::new(),
            callback_query_handlers: Vec::new()
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
                            Some(handler) => { handler.handle(value).await; /* todo */ },
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

    #[tokio::test]
    async fn test() {
        use crate::{
            types::{
                Message, ChatKind, MessageKind, Sender, ForwardKind, MediaKind, Chat, User, Update, UpdateKind
            },
            dispatcher::{simple::Dispatcher, updater::StreamUpdater},
        };

        let mes = Message {
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
        };

        async fn handler(mes: Message) {
            println!("{:#?}", mes)
        }

        async fn handler2(mes: Message) -> Result<(), Infallible>{
            println!("{:#?}", mes);

            Ok(())
        }

        let mut dp = Dispatcher::<Infallible>::new()
            .message_handler(true, handler)
            .message_handler(true, handler2);

        use futures::future::ready;
        use futures::stream;

        dp.dispatch(StreamUpdater::new(stream::once(ready(Result::<_, ()>::Ok(Update { id: 0, kind: UpdateKind::Message(mes) }))))).await;
    }
}
