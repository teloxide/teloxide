use crate::dispatching::{dialogue::Storage, Dispatcher, DispatcherBuilder};
use std::marker::PhantomData;

use futures::StreamExt;
use tokio::sync::mpsc;

use crate::{
    dispatching::{
        core::{DispatchError, Handler},
        dialogue::dialogue_ctx::DialogueContext,
        error_handlers::ErrorHandler,
        update_listeners::UpdateListener,
    },
    types::{Update, UpdateKind},
    Bot,
};
use lockfree::map::Map;
use std::sync::{Arc, Mutex};

/// A dispatcher of dialogues.
///
/// Note that it implements [`DispatcherHandler`], so you can just put an
/// instance of this dispatcher into the [`Dispatcher`]'s methods.
///
/// See the [module-level documentation](crate::dispatching::dialogue) for the
/// design overview.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`DispatcherHandler`]: crate::dispatching::DispatcherHandler
pub struct DialogueDispatcher<D, S, Err, ErrHandler> {
    storage: Arc<S>,
    dispatcher: Arc<Dispatcher<Err, ErrHandler, DialogueContext<Update, D, S>>>,
    _phantom: PhantomData<Mutex<D>>,

    /// A lock-free map to handle updates from the same chat sequentially, but
    /// concurrently from different chats.
    ///
    /// A value is the TX part of an unbounded asynchronous MPSC channel. A
    /// handler that executes updates from the same chat ID sequentially
    /// handles the RX part.
    senders: Arc<Map<i64, mpsc::UnboundedSender<Update>>>,
}

pub struct DialogueDispatcherBuilder<D, S, Err, ErrHandler> {
    storage: Arc<S>,
    dispatcher: DispatcherBuilder<Err, ErrHandler, DialogueContext<Update, D, S>>,
    _phantom: PhantomData<Mutex<D>>,
}

impl<D, S, Err> DialogueDispatcherBuilder<D, S, Err, ()>
where
    S: Storage<D>,
    D: Default + Send + 'static,
{
    /// Creates a dispatcher with the specified `handler` and [`InMemStorage`]
    /// (a default storage).
    ///
    /// [`InMemStorage`]: crate::dispatching::dialogue::InMemStorage
    #[must_use]
    pub fn new(bot: Bot, bot_name: impl Into<Arc<str>>, storage: S) -> Self {
        Self {
            storage: Arc::new(storage),
            dispatcher: DispatcherBuilder::new(bot, bot_name),
            _phantom: PhantomData,
        }
    }
}

impl<D, S, Err> DialogueDispatcherBuilder<D, S, Err, ()>
where
    S: Storage<D> + Send + Sync + 'static,
    D: Default + Clone + Send + Sync + 'static,
    Err: Send + Sync + 'static,
{
    pub fn error_handler<H>(
        self,
        error_handler: H,
    ) -> DialogueDispatcherBuilder<
        D,
        S,
        Err,
        impl ErrorHandler<DispatchError<DialogueContext<Update, D, S>, Err>>,
    >
    where
        H: for<'a> ErrorHandler<DispatchError<DialogueContext<Update, D, S>, Err>>
            + Send
            + Sync
            + 'static,
    {
        let handler = Arc::new(error_handler);
        let DialogueDispatcherBuilder { storage, dispatcher, _phantom } = self;
        let s = storage.clone();
        let handler = move |err: DispatchError<DialogueContext<Update, D, S>, Err>| {
            let error_handler = handler.clone();
            let storage = s.clone();

            async move {
                let err = err;
                match err {
                    DispatchError::NoHandler(ref cx) => {
                        let dialogue = cx.dialogue.clone();
                        match dialogue {
                            Some(d) => {
                                // TODO: rework storage api and remove this call
                                storage
                                    .update_dialogue(cx.chat_id.unwrap(), d)
                                    .await
                                    .ok()
                                    .unwrap()
                                    .unwrap();
                            }
                            None => {}
                        }
                        error_handler.handle_error(err).await;
                    }
                    _ => {
                        error_handler.handle_error(err).await;
                    }
                }
            }
        };

        DialogueDispatcherBuilder {
            storage,
            dispatcher: dispatcher.error_handler(handler),
            _phantom: PhantomData,
        }
    }
}

impl<D, S, Err, ErrHandler> DialogueDispatcherBuilder<D, S, Err, ErrHandler>
where
    D: Default + Send + 'static,
{
    pub fn handle(
        mut self,
        handler: impl Handler<DialogueContext<Update, D, S>, Err> + Send + Sync + 'static,
    ) -> Self {
        self.dispatcher._add_handler(handler);
        self
    }
}

impl<D, S, Err, ErrHandler> DialogueDispatcherBuilder<D, S, Err, ErrHandler>
where
    S: Storage<D>,
    ErrHandler: ErrorHandler<DispatchError<DialogueContext<Update, D, S>, Err>>,
{
    pub fn build(self) -> DialogueDispatcher<D, S, Err, ErrHandler> {
        let DialogueDispatcherBuilder { storage, dispatcher, .. } = self;
        DialogueDispatcher {
            storage,
            dispatcher: Arc::new(dispatcher.build()),
            _phantom: PhantomData,
            senders: Arc::new(Map::new()),
        }
    }
}

impl<D, S, Err, ErrHandler> DialogueDispatcher<D, S, Err, ErrHandler>
where
    D: Default + Send + Sync + 'static,
    S: Storage<D> + Send + Sync + 'static,
    <S as Storage<D>>::Error: Send,
    ErrHandler:
        ErrorHandler<DispatchError<DialogueContext<Update, D, S>, Err>> + Send + Sync + 'static,
    Err: Send + 'static,
{
    pub async fn dispatch_one(&self, upd: Update) {
        let chat_id = upd.try_get_chat_id();
        match chat_id {
            Some(chat_id) => match self.senders.get(&chat_id) {
                Some(tx) => {
                    if tx.1.send(upd).is_err() {
                        panic!("We are not dropping a receiver or call .close() on it",);
                    }
                }
                None => {
                    let tx = self.new_tx();
                    if tx.send(upd).is_err() {
                        panic!("We are not dropping a receiver or call .close() on it",);
                    }
                    self.senders.insert(chat_id, tx);
                }
            },
            None => {
                let cx = self._make_cx(upd).await;
                self.dispatcher.dispatch_one_with_cx(cx).await;
            }
        }
    }

    pub async fn dispatch_with_listener<ListenerErr>(
        &self,
        listener: impl UpdateListener<ListenerErr>,
        listener_error_handler: &impl ErrorHandler<ListenerErr>,
    ) {
        listener
            .for_each_concurrent(None, move |res| async move {
                match res {
                    Ok(upd) => self.dispatch_one(upd).await,
                    Err(e) => listener_error_handler.handle_error(e).await,
                };
            })
            .await;
    }

    pub async fn _make_cx(&self, upd: Update) -> DialogueContext<Update, D, S> {
        let storage = self.storage.clone();
        let senders = self.senders.clone();

        let chat_id = upd.try_get_chat_id();
        let dialogue = match chat_id {
            Some(id) => {
                Some(
                    self.storage
                        .clone() // TODO: not move in remove_dialogue self
                        .remove_dialogue(id)
                        .await
                        .map(Option::unwrap_or_default)
                        .unwrap_or_else(|_| panic!("TODO: StorageError")),
                )
            }
            None => None,
        };

        let cx =
            DialogueContext::new(self.dispatcher.make_cx(upd), storage, dialogue, senders, chat_id);
        cx
    }
}

impl Update {
    fn try_get_chat_id(&self) -> Option<i64> {
        match &self.kind {
            UpdateKind::Message(m) => Some(m.chat_id()),
            UpdateKind::EditedMessage(m) => Some(m.chat_id()),
            UpdateKind::CallbackQuery(q) => q.message.as_ref().map(|mes| mes.chat_id()),
            _ => None,
        }
    }
}

impl<D, S, Err, ErrHandler> DialogueDispatcher<D, S, Err, ErrHandler>
where
    D: Default + Send + 'static,
    S: Storage<D> + Send + Sync + 'static,
    S::Error: Send + 'static,
    ErrHandler:
        ErrorHandler<DispatchError<DialogueContext<Update, D, S>, Err>> + Send + Sync + 'static,
    Err: Send + 'static,
{
    #[must_use]
    fn new_tx(&self) -> mpsc::UnboundedSender<Update> {
        let (tx, rx) = mpsc::unbounded_channel();

        let dispatcher = Arc::clone(&self.dispatcher);
        let senders = self.senders.clone();
        let storage = self.storage.clone();

        tokio::spawn(rx.for_each(move |upd: Update| {
            let cx = make_cx(dispatcher.clone(), storage.clone(), senders.clone(), upd);
            let dispatcher = dispatcher.clone();
            async move {
                let cx = cx.await;
                dispatcher.dispatch_one_with_cx(cx).await;
            }
        }));

        tx
    }
}

async fn make_cx<Err, ErrHandler, S, D>(
    dispatcher: Arc<Dispatcher<Err, ErrHandler, DialogueContext<Update, D, S>>>,
    storage: Arc<S>,
    senders: Arc<Map<i64, mpsc::UnboundedSender<Update>>>,
    upd: Update,
) -> DialogueContext<Update, D, S>
where
    S: Storage<D> + Send + Sync + 'static,
    D: Default + Send + 'static,
    ErrHandler: ErrorHandler<DispatchError<DialogueContext<Update, D, S>, Err>>,
    Err: Send + 'static,
{
    let chat_id = upd.try_get_chat_id();
    let dialogue = match chat_id {
        Some(id) => Some(
            storage
                .clone()
                .remove_dialogue(id)
                .await
                .map(Option::unwrap_or_default)
                .unwrap_or_else(|_| panic!("TODO: StorageError")),
        ),
        None => None,
    };

    let cx = DialogueContext::new(dispatcher.make_cx(upd), storage, dialogue, senders, chat_id);
    cx
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        dispatching::{
            dialogue::{DialogueStage, DialogueWithCx, InMemStorage},
            updates,
        },
        types::Message,
        Bot,
    };
    use futures::{stream, StreamExt};
    use lazy_static::lazy_static;
    use std::convert::Infallible;
    use tokio::{
        sync::Mutex,
        time::{delay_for, Duration},
    };

    #[tokio::test]
    #[allow(deprecated)]
    async fn updates_from_same_chat_executed_sequentially() {
        lazy_static! {
            static ref SEQ1: Mutex<Vec<u32>> = Mutex::new(Vec::new());
            static ref SEQ2: Mutex<Vec<u32>> = Mutex::new(Vec::new());
            static ref SEQ3: Mutex<Vec<u32>> = Mutex::new(Vec::new());
        }

        let dispatcher = DialogueDispatcherBuilder::new(Bot::new(""), "", InMemStorage::new())
            .handle(updates::message().by(
                |cx: DialogueWithCx<Message, (), Infallible>| async move {
                    delay_for(Duration::from_millis(300)).await;

                    match (cx.cx.update.chat_id(), cx.cx.update.text().unwrap()) {
                        (1, s) => {
                            SEQ1.lock().await.push(s.parse().unwrap());
                        }
                        (2, s) => {
                            SEQ2.lock().await.push(s.parse().unwrap());
                        }
                        (3, s) => {
                            SEQ3.lock().await.push(s.parse().unwrap());
                        }
                        _ => unreachable!(),
                    }
                    cx.next(|()| DialogueStage::Next(())).await;
                },
            ))
            .error_handler(|_| async move { unreachable!() })
            .build();

        let updates = stream::iter(vec![
            mes(1, "174"),
            mes(1, "125"),
            mes(2, "411"),
            mes(1, "2"),
            mes(2, "515"),
            mes(2, "623"),
            mes(1, "193"),
            mes(1, "104"),
            mes(2, "2222"),
            mes(2, "737"),
            mes(3, "72782"),
            mes(3, "2737"),
            mes(1, "7"),
            mes(1, "7778"),
            mes(3, "5475"),
            mes(3, "1096"),
            mes(3, "872"),
            mes(2, "10"),
            mes(2, "55456"),
            mes(3, "5665"),
            mes(3, "1611"),
        ]);

        dispatcher
            .dispatch_with_listener(updates.map::<Result<_, Infallible>, _>(Ok), &|_| async {
                panic!("error with listener")
            })
            .await;

        // Wait until our futures to be finished.
        delay_for(Duration::from_millis(3000)).await;

        assert_eq!(*SEQ1.lock().await, vec![174, 125, 2, 193, 104, 7, 7778]);
        assert_eq!(*SEQ2.lock().await, vec![411, 515, 623, 2222, 737, 10, 55456]);
        assert_eq!(*SEQ3.lock().await, vec![72782, 2737, 5475, 1096, 872, 5665, 1611]);
    }

    fn mes(chat_id: i64, text: impl Into<String>) -> Update {
        use crate::types::{
            ChatKind::Private, ForwardKind::Origin, MediaKind::Text, MessageKind::Common, *,
        };

        Update::new(
            0,
            UpdateKind::Message(Message {
                id: 199785,
                date: 1568289890,
                chat: Chat {
                    id: chat_id,
                    kind: Private(ChatPrivate {
                        type_: (),
                        username: Some("aka_dude".into()),
                        first_name: Some("Андрей".into()),
                        last_name: Some("Власов".into()),
                    }),
                    photo: None,
                },
                via_bot: None,
                kind: Common(MessageCommon {
                    from: Some(User {
                        id: 250918540,
                        is_bot: false,
                        first_name: "Андрей".into(),
                        last_name: Some("Власов".into()),
                        username: Some("aka_dude".into()),
                        language_code: Some("en".into()),
                    }),
                    forward_kind: Origin(ForwardOrigin { reply_to_message: None }),
                    edit_date: None,
                    media_kind: Text(MediaText { text: text.into(), entities: vec![] }),
                    reply_markup: None,
                }),
            }),
        )
    }
}
