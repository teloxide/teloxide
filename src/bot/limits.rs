use crate::Bot;
use std::collections::{VecDeque, HashMap};
use std::time::Instant;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use std::cmp::max;
use crate::requests::{Requester, Request, Output, HasPayload, Payload};
use crate::payloads::{GetMe, SendMessage};
use crate::types::ChatId;
use tokio::sync::oneshot::{Sender, Receiver, channel};
use std::future::Future;
use futures::task::{Context, Poll};
use pin_project::__private::Pin;
use core::time::Duration;
use futures::{TryFutureExt, StreamExt, FutureExt};
use tokio::time::{delay_until, delay_for};
use futures::stream::FuturesUnordered;
use futures::executor::block_on;
use futures::future::join3;
use futures::future::ready;

// FIXME: rename to Throttle

const MINUTE: Duration = Duration::from_secs(50); // FIXME: min = sec * 10 only in tests
const SECOND: Duration = Duration::from_secs(1);
const DELAY: Duration = Duration::from_millis(250); // second/4

pub struct Limits {
    /// Allowed messages in one chat per second
    pub chat_s: u32,
    /// Allowed messages per second
    pub overall_s: u32,
    /// Allowed messages in one chat per minute
    pub chat_m: u32,
}

// https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this
impl Default for Limits {
    fn default() -> Self {
        Self {
            chat_s: 1,
            overall_s: 30,
            chat_m: 20,
        }
    }
}

pub struct Limited<B> {
    bot: B,
    queue: mpsc::Sender<(ChatId, Sender<()>)>,
}

async fn worker(
    limits: Limits,
    mut queue_rx: mpsc::Receiver<(ChatId, Sender<()>)>,
) {
    // FIXME: use spawn_blocking?

    // FIXME: remove unnecessary ChatId clones

    // FIXME: struct with fast random remove and append-to-the-end
    let mut queue: Vec<Option<(ChatId, Sender<()>)>> = Vec::new(); // FIXME: with_cap

    let mut history: VecDeque<(ChatId, Instant)> = VecDeque::new();
    // hchats[chat] = history.iter().filter(|(c, _)| c == chat).count()
    let mut hchats: HashMap<ChatId, u32> = HashMap::new();

    loop {
        // If there are no pending requests we are just waiting
        if queue.is_empty() {
            queue.push(Some(queue_rx.recv().await.unwrap()));
        }

        // update local queue with latest requests
        while let Ok(e) = queue_rx.try_recv() {
            // FIXME: properly check for errors (stop when the bot's sender is dropped?)
            queue.push(Some(e))
        }

        let now = Instant::now();
        let min_back = now - MINUTE;
        let sec_back = now - SECOND;

        // make history and hchats up-to-date
        while let Some((_, time)) = history.front() {
            // history is sorted, we found first up-to-date thing
            if time >= &min_back { break; }

            if let Some((chat, _)) = history.pop_front() {
                if let Entry::Occupied(entry) = hchats
                    .entry(chat)
                    .and_modify(|count| { *count -= 1; }) {
                    if *entry.get() == 0 { entry.remove_entry(); }
                }
            }
        }

        // as truncates which is ok since in case of truncation it would always be >= limits.overall_s
        let mut allowed = limits.overall_s.saturating_sub(history.iter().take_while(|(_, time)| time > &sec_back).count() as u32);

        if allowed == 0 {
            delay_for(DELAY).await;
            continue;
        }

        let mut hchats_s = HashMap::new();
        for (chat, _) in history.iter().take_while(|(_, time)| time > &sec_back) {
            *hchats_s
                .entry(chat.clone())
                .or_insert(0) += 1;
        }

        let mut empty = 0;
        for i in 0..queue.len() {
            let chat = &queue[i].as_ref().unwrap().0;
            let cond = {
                hchats_s
                    .get(chat)
                    .copied()
                    .unwrap_or(0) < limits.chat_s &&
                    hchats
                        .get(chat)
                        .copied()
                        .unwrap_or(0) < limits.chat_m
            };

            if cond {
                {
                    *hchats_s.entry(chat.clone()).or_insert(0) += 1;
                    *hchats.entry(chat.clone()).or_insert(0) += 1;
                    history.push_back((chat.clone(), Instant::now()));
                }
                queue[i].take().unwrap().1.send(());

                allowed -= 1;
                if allowed == 0 {
                    if empty != i {
                        // FIXME: this could be more optimal
                        for j in i..queue.len() {
                            queue.swap(j, empty);
                            empty += 1;
                        }
                    }
                    break;
                }
            } else {
                queue.swap(i, empty);
                empty += 1;
            }
        }
        queue.truncate(empty);

        delay_for(DELAY).await;
    }
}

impl<B> Limited<B> {
    pub fn new(bot: B, limits: Limits) -> (Self, impl Future<Output = ()>) {
        // FIXME: just a random number, currently
        let (queue_tx, queue_rx) = mpsc::channel(130);

        let worker = worker(
            limits,
            queue_rx,
        );

        let this = Self { bot, queue: queue_tx, };

        (this, worker)
    }
}

impl<B: Requester> Requester for Limited<B> {
    type GetMe = B::GetMe;

    fn get_me(&self) -> Self::GetMe {
        self.bot.get_me()
    }

    type SendMessage = LimitedRequest<B::SendMessage>;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>
    {
        LimitedRequest(self.bot.send_message(chat_id, text), self.queue.clone())
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

pub struct LimitedRequest<R>(R, mpsc::Sender<(ChatId, Sender<()>)>);

impl<R: HasPayload> HasPayload for LimitedRequest<R> {
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.0.payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.0.payload_ref()
    }
}

impl<R: Request> Request for LimitedRequest<R>
where
    <R as HasPayload>::Payload: GetChatId,
{
    type Err = R::Err;
    type Send = LimitedSend<R>;
    type SendRef = LimitedSend<R>;

    fn send(mut self) -> Self::Send {
        let (tx, rx) = channel();
        let send = self.1.send_t((self.0.payload_ref().get_chat_id().clone(), tx));
        LimitedSend::Registering {
            request: self.0,
            send,
            wait: rx,
        }
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
                    if let SendRepl::Registering { request, send: _, wait } = self.as_mut().project_replace(LimitedSend::Done) {
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
                    if let SendRepl::Pending { request, wait: _ } = self.as_mut().project_replace(LimitedSend::Done) {
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

use chan_send::{ChanSend, SendTy as _};
use crate::bot::limits::chan_send::SendTy;
use std::collections::hash_map::Entry;
use core::mem;

mod chan_send {
    use tokio::sync::mpsc;
    use crate::types::ChatId;
    use tokio::sync::oneshot::Sender;
    use std::future::Future;
    use futures::task::{Context, Poll};
    use pin_project::__private::Pin;
    use tokio::sync::mpsc::error::SendError;

    pub(crate) trait SendTy {
        fn send_t(self, val: (ChatId, Sender<()>)) -> ChanSend;
    }

    #[pin_project::pin_project]
    pub/*(crate) */struct ChanSend(#[pin] Inner); // FIXME

    #[cfg(not(feature = "nightly"))]
    type Inner = Pin<Box<dyn Future<Output = Result<(), SendError<(ChatId, Sender<()>)>>>>>;
    #[cfg(feature = "nightly")]
    type Inner = impl Future<Output = Result<(), SendError<(ChatId, Sender<()>)>>>;

    impl SendTy for mpsc::Sender<(ChatId, Sender<()>)> {
        fn send_t(mut self, val: (ChatId, Sender<()>)) -> ChanSend {
            #[cfg(feature = "nightly")]
            {
                fn def(mut sender: mpsc::Sender<(ChatId, Sender<()>)>, val: (ChatId, Sender<()>)) -> Inner {
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