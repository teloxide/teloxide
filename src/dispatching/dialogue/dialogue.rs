use crate::dispatching::dialogue::{Storage};
use std::sync::Arc;
use lockfree::map::Map;
use tokio::sync::mpsc;
use crate::types::Update;

pub struct Dialogue<D, E, Cur=D> {
    pub data: Cur,
    pub chat_id: i64,
    pub(crate) storage: Arc<dyn Storage<D, Error = E> + Send + Sync>,
    pub(crate) senders: Arc<Map<i64, mpsc::UnboundedSender<Update>>>,
}

impl<D, E, Cur> Dialogue<D, E, Cur> {
    pub fn map<DN>(self, f: impl Fn(Cur) -> DN) -> Dialogue<D, E, DN> {
        let Dialogue { data, chat_id, storage, senders } = self;
        Dialogue {
            data: f(data),
            chat_id,
            storage,
            senders,
        }
    }
}

impl<D, E, Cur> Dialogue<D, E, Cur>
where
    D: Default + Send + 'static,
    Cur: Into<D>,
{
    pub async fn stay(self) {
        self.next(|x| x).await;
    }
}

impl<D, E, Cur> Dialogue<D, E, Cur>
where
    D: Default + Send + 'static,
{
    pub async fn next<Out: Into<D>>(self, factory: impl FnOnce(Cur) -> Out) {
        let Dialogue { data, chat_id, storage, .. } = self;
        let next = factory(data).into();

        if let Ok(Some(_)) = storage.update_dialogue(chat_id, next).await {
            panic!(
                "Oops, you have an bug in your Storage: update_dialogue returns Some after \
                 remove_dialogue"
            );
        }
    }

    pub async fn exit(self) {
        let Dialogue { chat_id, senders, .. } = self;

        // On the next .poll() call, the spawned future will
        // return Poll::Ready, because we are dropping the
        // sender right here:
        senders.remove(&chat_id);
    }
}
