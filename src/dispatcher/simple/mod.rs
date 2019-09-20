use std::{future::Future, pin::Pin};

use crate::{
    dispatcher::{
        filter::Filter,
        handler::Handler,
    },
    types::{
        Update,
        Message,
        UpdateKind,
        CallbackQuery,
        ChosenInlineResult,
    },
};

use tokio::stream::Stream;


pub type Handlers<T> = Vec<(Box<dyn Filter<T>>, Box<dyn Handler<T>>)>;

pub struct Dispatcher {
    message_handlers: Handlers<Message>,
    edited_message_handlers: Handlers<Message>,
    channel_post_handlers: Handlers<Message>,
    edited_channel_post_handlers: Handlers<Message>,
    inline_query_handlers: Handlers<()>,
    chosen_inline_result_handlers: Handlers<ChosenInlineResult>,
    callback_query_handlers: Handlers<CallbackQuery>,
}

impl Dispatcher {
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
        F: Filter<Message> + 'static,
        H: Handler<Message> + 'static,
    {
        self.message_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn edited_message_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'static,
        H: Handler<Message> + 'static,
    {
        self.edited_message_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn channel_post_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'static,
        H: Handler<Message> + 'static,
    {
        self.channel_post_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn edited_channel_post_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<Message> + 'static,
        H: Handler<Message> + 'static,
    {
        self.edited_channel_post_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn inline_query_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<()> + 'static,
        H: Handler<()> + 'static,
    {
        self.inline_query_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn chosen_inline_result_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<ChosenInlineResult> + 'static,
        H: Handler<ChosenInlineResult> + 'static,
    {
        self.chosen_inline_result_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    pub fn callback_query_handler<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter<CallbackQuery> + 'static,
        H: Handler<CallbackQuery> + 'static,
    {
        self.callback_query_handlers.push((Box::new(filter), Box::new(handler)));
        self
    }

    // TODO: Can someone simplify this?
    pub async fn dispatch<S>(&mut self, updates: S)
    where
        S: Stream<Item=Update>
    {
        use futures::StreamExt;

        let dp = &*self;

        updates.for_each(|Update { id, kind }| async move {
            log::debug!("Handled update#{id:?}: {kind:?}", id = id, kind = kind);

            match kind {
                UpdateKind::Message(mes) => {
                    call_handler(
                        find_handler(&dp.message_handlers, &mes),
                        mes
                    )
                        .await;
                },
                UpdateKind::EditedMessage(mes) => {
                    call_handler(
                        find_handler(&dp.edited_message_handlers, &mes),
                        mes
                    )
                        .await;
                },
                UpdateKind::ChannelPost(post) => {
                    call_handler(
                        find_handler(&dp.channel_post_handlers, &post),
                        post
                    )
                        .await;
                },
                UpdateKind::EditedChannelPost(post) => {
                    call_handler(
                        find_handler(&dp.edited_channel_post_handlers, &post),
                        post
                    )
                        .await;
                },
                UpdateKind::InlineQuery(query) => {
                    call_handler(
                        find_handler(&dp.inline_query_handlers, &query),
                        query
                    )
                        .await;
                },
                UpdateKind::ChosenInlineResult(result) => {
                    call_handler(
                        find_handler(&dp.chosen_inline_result_handlers, &result),
                        result
                    )
                        .await;
                },
                UpdateKind::CallbackQuery(callback) => {
                    call_handler(
                        find_handler(&dp.callback_query_handlers, &callback),
                        callback
                    )
                        .await;
                },
            }
        })
            .await;
    }
}

/// Helper function
fn find_handler<'a, T: std::fmt::Debug>(handlers: &'a Handlers<T>, value: &T) -> Option<&'a Box<Handler<T>>> {
    let handler = handlers.iter().find_map(|e| {
        let (filter, handler) = e;
        if filter.test(value) {
            Some(handler)
        } else {
            None
        }
    });

    handler
}

/// Helper function
async fn call_handler<T: std::fmt::Debug>(handler: Option<&Box<Handler<T>>>, value: T) {
    match handler {
        Some(handler) => handler.handle(value).await,
        None => log::warn!("Unhandled update: {:?}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        use crate::{
            types::{
                Message, ChatKind, MessageKind, Sender, ForwardKind, MediaKind, Chat, User, Update, UpdateKind
            },
            dispatcher::simple::Dispatcher,
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

        let mut dp = Dispatcher::new()
            .message_handler(true, handler);

        dp.dispatch(tokio::stream::iter(vec![Update { id: 0, kind: UpdateKind::Message(mes) }])).await;
    }
}
