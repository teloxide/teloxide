use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    future::Future,
    pin::Pin,
    time::{Duration, Instant},
};

use futures::task::{Context, Poll};
use tokio::{
    sync::{
        mpsc,
        oneshot::{channel, Receiver, Sender},
    },
    time::delay_for,
};
use vecrem::VecExt;

use crate::{
    bot::limits::chan_send::{ChanSend, SendTy},
    payloads::SendMessage,
    requests::{HasPayload, Output, Request, Requester},
    types::ChatId,
};

const MINUTE: Duration = Duration::from_secs(50); // FIXME: min = sec * 10 only in tests
const SECOND: Duration = Duration::from_secs(1);
const DELAY: Duration = Duration::from_millis(250); // second/4

/// Telegram request limits.
///
/// This struct is used in [`Throttle`]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Limits {
    /// Allowed messages in one chat per second
    pub chat_s: u32,
    /// Allowed messages per second
    pub overall_s: u32,
    /// Allowed messages in one chat per minute
    pub chat_m: u32,
}

/// Defaults are taken from [telegram documentation][tgdoc].
///
/// [tgdoc]: https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this
impl Default for Limits {
    fn default() -> Self {
        Self { chat_s: 1, overall_s: 30, chat_m: 20 }
    }
}

pub struct Throttle<B> {
    bot: B,
    queue: mpsc::Sender<(ChatId, Sender<()>)>,
}

async fn worker(limits: Limits, mut queue_rx: mpsc::Receiver<(ChatId, Sender<()>)>) {
    // FIXME: use spawn_blocking?

    // FIXME: remove unnecessary ChatId clones

    // FIXME: Make an research about data structures for this queue.
    //        Currently this is O(n) removing (n = number of elements stayed),
    //        amortized O(1) push (vec+vecrem).
    let mut queue: Vec<(ChatId, Sender<()>)> = Vec::new(); // FIXME: with_cap

    let mut history: VecDeque<(ChatId, Instant)> = VecDeque::new();
    // hchats[chat] = history.iter().filter(|(c, _)| c == chat).count()
    let mut hchats: HashMap<ChatId, u32> = HashMap::new();

    let mut hchats_s = HashMap::new();

    loop {
        // If there are no pending requests we are just waiting
        if queue.is_empty() {
            queue.push(queue_rx.recv().await.unwrap());
        }

        // update local queue with latest requests
        while let Ok(e) = queue_rx.try_recv() {
            // FIXME: properly check for errors (stop when the bot's sender is dropped?)
            queue.push(e)
        }

        let now = Instant::now();
        let min_back = now - MINUTE;
        let sec_back = now - SECOND;

        // make history and hchats up-to-date
        while let Some((_, time)) = history.front() {
            // history is sorted, we found first up-to-date thing
            if time >= &min_back {
                break;
            }

            if let Some((chat, _)) = history.pop_front() {
                if let Entry::Occupied(entry) = hchats.entry(chat).and_modify(|count| {
                    *count -= 1;
                }) {
                    if *entry.get() == 0 {
                        entry.remove_entry();
                    }
                }
            }
        }

        // as truncates which is ok since in case of truncation it would always be >=
        // limits.overall_s
        let mut allowed = limits
            .overall_s
            .saturating_sub(history.iter().take_while(|(_, time)| time > &sec_back).count() as u32);

        if allowed == 0 {
            hchats_s.clear();
            delay_for(DELAY).await;
            continue;
        }

        for (chat, _) in history.iter().take_while(|(_, time)| time > &sec_back) {
            *hchats_s.entry(chat.clone()).or_insert(0) += 1;
        }

        let mut queue_rem = queue.removing();
        while let Some(entry) = queue_rem.next() {
            let chat = &entry.value().0;
            let cond = {
                hchats_s.get(chat).copied().unwrap_or(0) < limits.chat_s
                    && hchats.get(chat).copied().unwrap_or(0) < limits.chat_m
            };

            if cond {
                {
                    *hchats_s.entry(chat.clone()).or_insert(0) += 1;
                    *hchats.entry(chat.clone()).or_insert(0) += 1;
                    history.push_back((chat.clone(), Instant::now()));
                }
                entry.remove().1.send(());

                allowed -= 1;
                if allowed == 0 {
                    break;
                }
            } else {
                entry.skip();
            }
        }
        drop(queue_rem);

        hchats_s.clear();
        delay_for(DELAY).await;
    }
}

impl<B> Throttle<B> {
    /// Creates new [`Throttle`] alongside with worker future.
    ///
    /// Note: [`Throttle`] will only send requests if returned worker is
    /// polled/spawned/awaited.
    pub fn new(bot: B, limits: Limits) -> (Self, impl Future<Output = ()>) {
        // FIXME: just a random number, currently
        let (queue_tx, queue_rx) = mpsc::channel(130);

        let worker = worker(limits, queue_rx);

        let this = Self { bot, queue: queue_tx };

        (this, worker)
    }

    /// Creates new [`Throttle`] spawning the worker with `tokio::spawn`
    ///
    /// Note: it's recommended to use [`RequesterExt::throttle`] instead.
    pub fn new_spawn(bot: B, limits: Limits) -> Self
    where
        // Basically, I hate this bound.
        // This is yet another problem caused by [rust-lang/#76882].
        // And I think it *is* a bug.
        //
        // [rust-lang/#76882]: https://github.com/rust-lang/rust/issues/76882
        //
        // Though crucially I can't think of a case with non-static bot.
        // But anyway, it doesn't change the fact that this bound is redundant.
        //
        // (waffle)
        B: 'static,
    {
        let (this, worker) = Self::new(bot, limits);
        tokio::spawn(worker);
        this
    }

    /// Allows to access inner bot
    pub fn inner(&self) -> &B {
        &self.bot
    }

    /// Unwraps inner bot
    pub fn into_inner(self) -> B {
        self.bot
    }
}

impl<B: Requester> Requester for Throttle<B> {
    type GetMe = B::GetMe;

    fn get_me(&self) -> Self::GetMe {
        self.bot.get_me()
    }

    type SendMessage = ThrottlingRequest<B::SendMessage>;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        ThrottlingRequest(self.bot.send_message(chat_id, text), self.queue.clone())
    }
}

pub trait GetChatId {
    // FIXME(waffle): add note about false negatives with ChatId::Username
    fn get_chat_id(&self) -> &ChatId;
}

impl GetChatId for SendMessage {
    fn get_chat_id(&self) -> &ChatId {
        &self.chat_id
    }
}

pub struct ThrottlingRequest<R>(R, mpsc::Sender<(ChatId, Sender<()>)>);

impl<R: HasPayload> HasPayload for ThrottlingRequest<R> {
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.0.payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.0.payload_ref()
    }
}

impl<R: Request> Request for ThrottlingRequest<R>
where
    <R as HasPayload>::Payload: GetChatId,
{
    type Err = R::Err;
    type Send = LimitedSend<R>;
    type SendRef = LimitedSend<R>;

    fn send(self) -> Self::Send {
        let (tx, rx) = channel();
        let send = self.1.send_t((self.0.payload_ref().get_chat_id().clone(), tx));
        LimitedSend::Registering { request: self.0, send, wait: rx }
    }

    fn send_ref(&self) -> Self::SendRef {
        unimplemented!()
    }
}

#[pin_project::pin_project(project = SendProj, project_replace = SendRepl)]
pub enum LimitedSend<R: Request> {
    Registering {
        request: R,
        #[pin]
        send: ChanSend,
        wait: Receiver<()>,
    },
    Pending {
        request: R,
        #[pin]
        wait: Receiver<()>,
    },
    Sent {
        #[pin]
        fut: R::Send,
    },
    Done,
}

impl<R: Request> Future for LimitedSend<R> {
    type Output = Result<Output<R>, R::Err>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.as_mut().project() {
            SendProj::Registering { request: _, send, wait: _ } => match send.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(r) => {
                    // FIXME(waffle): remove unwrap
                    r.unwrap();
                    if let SendRepl::Registering { request, send: _, wait } =
                        self.as_mut().project_replace(LimitedSend::Done)
                    {
                        self.as_mut().project_replace(LimitedSend::Pending { request, wait });
                    }

                    self.poll(cx)
                }
            },
            SendProj::Pending { request: _, wait } => match wait.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(r) => {
                    // FIXME(waffle): remove unwrap
                    r.unwrap();
                    if let SendRepl::Pending { request, wait: _ } =
                        self.as_mut().project_replace(LimitedSend::Done)
                    {
                        self.as_mut().project_replace(LimitedSend::Sent { fut: request.send() });
                    }

                    self.poll(cx)
                }
            },
            SendProj::Sent { fut } => {
                let res = futures::ready!(fut.poll(cx));
                self.set(LimitedSend::Done);
                Poll::Ready(res)
            }
            SendProj::Done => Poll::Pending,
        }
    }
}

mod chan_send {
    use crate::types::ChatId;
    use futures::task::{Context, Poll};
    use pin_project::__private::Pin;
    use std::future::Future;
    use tokio::sync::{mpsc, mpsc::error::SendError, oneshot::Sender};

    pub(crate) trait SendTy {
        fn send_t(self, val: (ChatId, Sender<()>)) -> ChanSend;
    }

    #[pin_project::pin_project]
    pub struct ChanSend(#[pin] Inner); // FIXME

    #[cfg(not(feature = "nightly"))]
    type Inner = Pin<Box<dyn Future<Output = Result<(), SendError<(ChatId, Sender<()>)>>>>>;
    #[cfg(feature = "nightly")]
    type Inner = impl Future<Output = Result<(), SendError<(ChatId, Sender<()>)>>>;

    impl SendTy for mpsc::Sender<(ChatId, Sender<()>)> {
        fn send_t(mut self, val: (ChatId, Sender<()>)) -> ChanSend {
            #[cfg(feature = "nightly")]
            {
                fn def(
                    mut sender: mpsc::Sender<(ChatId, Sender<()>)>,
                    val: (ChatId, Sender<()>),
                ) -> Inner {
                    async move { sender.send(val).await }
                }
                return ChanSend(def(self, val));
            }
            #[cfg(not(feature = "nightly"))]
            return ChanSend(Box::pin(async move { self.send(val).await }));
        }
    }

    impl Future for ChanSend {
        type Output = Result<(), SendError<(ChatId, Sender<()>)>>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.project().0.poll(cx)
        }
    }
}
