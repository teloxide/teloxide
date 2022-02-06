use crate::dispatching::{
    dialogue::{
        DialogueDispatcherHandler, DialogueStage, DialogueWithCx, GetChatId, InMemStorage, Storage,
    },
    DispatcherHandler, UpdateWithCx,
};
use std::{fmt::Debug, marker::PhantomData};

use futures::{future::BoxFuture, FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::dispatching::dialogue::InMemStorageError;
use flurry::HashMap;
use std::sync::{Arc, Mutex};
use teloxide_core::requests::Requester;
use tokio_stream::wrappers::UnboundedReceiverStream;

/// A dispatcher of dialogues.
///
/// Note that it implements [`DispatcherHandler`], so you can just put an
/// instance of this dispatcher into the [`Dispatcher`]'s methods.
///
/// Note that when the storage methods [`Storage::remove_dialogue`] and
/// [`Storage::update_dialogue`] are failed, the errors are logged, but a result
/// from [`Storage::get_dialogue`] is provided to a user handler as-is so you
/// can respond to a concrete user with an error description.
///
/// See the [module-level documentation](crate::dispatching::dialogue) for the
/// design overview.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`DispatcherHandler`]: crate::dispatching::DispatcherHandler
#[deprecated(note = "Use dispatching2 instead")]
pub struct DialogueDispatcher<R, D, S, H, Upd> {
    storage: Arc<S>,
    handler: Arc<H>,
    _phantom: PhantomData<Mutex<D>>,

    /// A lock-free map to handle updates from the same chat sequentially, but
    /// concurrently from different chats.
    ///
    /// A value is the TX part of an unbounded asynchronous MPSC channel. A
    /// handler that executes updates from the same chat ID sequentially
    /// handles the RX part.
    senders: Arc<HashMap<i64, mpsc::UnboundedSender<UpdateWithCx<R, Upd>>>>,
}

impl<R, D, H, Upd> DialogueDispatcher<R, D, InMemStorage<D>, H, Upd>
where
    H: DialogueDispatcherHandler<R, Upd, D, InMemStorageError> + Send + Sync + 'static,
    Upd: GetChatId + Send + 'static,
    D: Default + Send + 'static,
{
    /// Creates a dispatcher with the specified `handler` and [`InMemStorage`]
    /// (a default storage).
    ///
    /// [`InMemStorage`]: crate::dispatching::dialogue::InMemStorage
    #[must_use]
    pub fn new(handler: H) -> Self {
        Self {
            storage: InMemStorage::new(),
            handler: Arc::new(handler),
            senders: Arc::new(HashMap::new()),
            _phantom: PhantomData,
        }
    }
}

impl<R, D, S, H, Upd> DialogueDispatcher<R, D, S, H, Upd>
where
    H: DialogueDispatcherHandler<R, Upd, D, S::Error> + Send + Sync + 'static,
    Upd: GetChatId + Send + 'static,
    D: Default + Send + 'static,
    S: Storage<D> + Send + Sync + 'static,
    S::Error: Debug + Send + 'static,
{
    /// Creates a dispatcher with the specified `handler` and `storage`.
    #[must_use]
    pub fn with_storage(handler: H, storage: Arc<S>) -> Self {
        Self {
            storage,
            handler: Arc::new(handler),
            senders: Arc::new(HashMap::new()),
            _phantom: PhantomData,
        }
    }

    #[must_use]
    fn new_tx(&self) -> mpsc::UnboundedSender<UpdateWithCx<R, Upd>>
    where
        R: Requester + Send + 'static,
    {
        let (tx, rx) = mpsc::unbounded_channel();

        let storage = Arc::clone(&self.storage);
        let handler = Arc::clone(&self.handler);
        let senders = Arc::clone(&self.senders);

        tokio::spawn(UnboundedReceiverStream::new(rx).for_each(move |cx: UpdateWithCx<R, Upd>| {
            let storage = Arc::clone(&storage);
            let handler = Arc::clone(&handler);
            let senders = Arc::clone(&senders);

            async move {
                let chat_id = cx.update.chat_id();

                let dialogue =
                    Arc::clone(&storage).get_dialogue(chat_id).await.map(Option::unwrap_or_default);

                match handler.handle(DialogueWithCx { cx, dialogue }).await {
                    DialogueStage::Next(new_dialogue) => {
                        if let Err(e) = storage.update_dialogue(chat_id, new_dialogue).await {
                            log::error!("Storage::update_dialogue failed: {:?}", e);
                        }
                    }
                    DialogueStage::Exit => {
                        // On the next .poll() call, the spawned future will
                        // return Poll::Ready, because we are dropping the
                        // sender right here:
                        senders.pin().remove(&chat_id);

                        if let Err(e) = storage.remove_dialogue(chat_id).await {
                            log::error!("Storage::remove_dialogue failed: {:?}", e);
                        }
                    }
                }
            }
        }));

        tx
    }
}

impl<R, D, S, H, Upd> DispatcherHandler<R, Upd> for DialogueDispatcher<R, D, S, H, Upd>
where
    H: DialogueDispatcherHandler<R, Upd, D, S::Error> + Send + Sync + 'static,
    Upd: GetChatId + Send + 'static,
    D: Default + Send + 'static,
    S: Storage<D> + Send + Sync + 'static,
    S::Error: Debug + Send + 'static,
    R: Requester + Send,
{
    fn handle(
        self,
        updates: mpsc::UnboundedReceiver<UpdateWithCx<R, Upd>>,
    ) -> BoxFuture<'static, ()>
    where
        UpdateWithCx<R, Upd>: 'static,
    {
        let this = Arc::new(self);

        UnboundedReceiverStream::new(updates)
            .for_each(move |cx| {
                let this = Arc::clone(&this);
                let chat_id = cx.update.chat_id();

                match this.senders.pin().get(&chat_id) {
                    // An old dialogue
                    Some(tx) => {
                        assert!(
                            tx.send(cx).is_ok(),
                            "We are not dropping a receiver or call .close() on it"
                        );
                    }
                    None => {
                        let tx = this.new_tx();
                        assert!(
                            tx.send(cx).is_ok(),
                            "We are not dropping a receiver or call .close() on it"
                        );
                        this.senders.pin().insert(chat_id, tx);
                    }
                }

                async {}
            })
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures::{stream, StreamExt};
    use lazy_static::lazy_static;
    use teloxide_core::Bot;
    use tokio::{
        sync::{mpsc, Mutex},
        time::Duration,
    };

    #[tokio::test]
    #[allow(deprecated)]
    async fn updates_from_same_chat_executed_sequentially() {
        #[derive(Debug)]
        struct MyUpdate {
            chat_id: i64,
            unique_number: u32,
        }

        impl MyUpdate {
            fn new(chat_id: i64, unique_number: u32) -> Self {
                Self { chat_id, unique_number }
            }
        }

        impl GetChatId for MyUpdate {
            fn chat_id(&self) -> i64 {
                self.chat_id
            }
        }

        lazy_static! {
            static ref SEQ1: Mutex<Vec<u32>> = Mutex::new(Vec::new());
            static ref SEQ2: Mutex<Vec<u32>> = Mutex::new(Vec::new());
            static ref SEQ3: Mutex<Vec<u32>> = Mutex::new(Vec::new());
        }

        let dispatcher = DialogueDispatcher::new(
            |cx: DialogueWithCx<Bot, MyUpdate, (), InMemStorageError>| async move {
                tokio::time::sleep(Duration::from_millis(300)).await;

                match cx.cx.update {
                    MyUpdate { chat_id: 1, unique_number } => {
                        SEQ1.lock().await.push(unique_number);
                    }
                    MyUpdate { chat_id: 2, unique_number } => {
                        SEQ2.lock().await.push(unique_number);
                    }
                    MyUpdate { chat_id: 3, unique_number } => {
                        SEQ3.lock().await.push(unique_number);
                    }
                    _ => unreachable!(),
                }

                DialogueStage::Next(())
            },
        );

        let updates = stream::iter(
            vec![
                MyUpdate::new(1, 174),
                MyUpdate::new(1, 125),
                MyUpdate::new(2, 411),
                MyUpdate::new(1, 2),
                MyUpdate::new(2, 515),
                MyUpdate::new(2, 623),
                MyUpdate::new(1, 193),
                MyUpdate::new(1, 104),
                MyUpdate::new(2, 2222),
                MyUpdate::new(2, 737),
                MyUpdate::new(3, 72782),
                MyUpdate::new(3, 2737),
                MyUpdate::new(1, 7),
                MyUpdate::new(1, 7778),
                MyUpdate::new(3, 5475),
                MyUpdate::new(3, 1096),
                MyUpdate::new(3, 872),
                MyUpdate::new(2, 10),
                MyUpdate::new(2, 55456),
                MyUpdate::new(3, 5665),
                MyUpdate::new(3, 1611),
            ]
            .into_iter()
            .map(|update| UpdateWithCx { update, requester: Bot::new("Doesn't matter here") })
            .collect::<Vec<UpdateWithCx<Bot, MyUpdate>>>(),
        );

        let (tx, rx) = mpsc::unbounded_channel();

        updates
            .for_each(move |update| {
                let tx = tx.clone();

                async move {
                    assert!(tx.send(update).is_ok(), "tx.send(update) failed");
                }
            })
            .await;

        dispatcher.handle(rx).await;

        // Wait until our futures to be finished.
        tokio::time::sleep(Duration::from_millis(3000)).await;

        assert_eq!(*SEQ1.lock().await, vec![174, 125, 2, 193, 104, 7, 7778]);
        assert_eq!(*SEQ2.lock().await, vec![411, 515, 623, 2222, 737, 10, 55456]);
        assert_eq!(*SEQ3.lock().await, vec![72782, 2737, 5475, 1096, 872, 5665, 1611]);
    }
}
