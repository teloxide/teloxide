use crate::Bot;
use std::collections::{VecDeque, HashMap};
use std::time::Instant;
use std::sync::Arc;
use tokio::sync::Mutex;
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

const MINUTE: Duration = Duration::from_secs(10);
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
    // Some fields are probably only needed by the worker
    //limits: Limits,
    bot: B,
    queue: Arc<Mutex<VecDeque<(ChatId, Sender<()>)>>>, // FIXME: struct with fast remove and add
    history: Arc<Mutex<VecDeque<(ChatId, Instant)>>>,
    hchats:  Arc<Mutex<HashMap<ChatId, u32>>>,
}

async fn worker(
    limits: Limits,
    queue: Arc<Mutex<VecDeque<(ChatId, Sender<()>)>>>,
    history: Arc<Mutex<VecDeque<(ChatId, Instant)>>>,
    hchats:  Arc<Mutex<HashMap<ChatId, u32>>>,
) {
    // FIXME: remove unnecessary ChatId clones
    loop {
        println!("1");
        let mut history = history.lock().await;
        let mut hchats = hchats.lock().await;
        let mut queue = queue.lock().await;

        let now = dbg!(Instant::now());
        let min_back = now - MINUTE;
        let sec_back = now - SECOND;

        println!("2");

        // make history and hchats up-to-date
        while let Some((_, time)) = history.front() {
            // history is sorted, we found first up-to-date thing
            if time >= &min_back { break; }

            if let Some((chat, _)) = history.pop_front() {
                hchats
                    .entry(chat)
                    .and_modify(|count| { *count -= 1; }); // TODO: remove entries with count == 0
            }
        }

        // as truncates which is ok since in case of truncation it would always be >= limits.overall_s
        let mut allowed = limits.overall_s.saturating_sub(dbg!(&history).iter().take_while(|(_, time)| time > &sec_back).count() as u32);

        if allowed == 0 {
            delay_for(DELAY).await;
            continue;
        }

        println!("3");

        let mut hchats_s = HashMap::new();
        for (chat, _) in history.iter().take_while(|(_, time)| time > &sec_back) {
            *hchats_s
                .entry(chat.clone())
                .or_insert(0) += 1;
        }


        dbg!(&hchats_s);
        dbg!(&hchats);

        dbg!(allowed);

        let mut i = 0;
        while allowed > 0 && i < queue.len() {
            let chat = &queue[i].0;

            if dbg!(hchats_s
                .get(chat)
                .copied()
                .unwrap_or(0) < limits.chat_s) &&
                dbg!(hchats
                    .get(chat)
                    .copied()
                    .unwrap_or(0) < limits.chat_m)
            {
                let chat = chat.clone();
                *hchats_s.entry(chat.clone()).or_insert(0) += 1;
                *hchats.entry(chat.clone()).or_insert(0) += 1;

                println!("worker send");
                dbg!(&hchats_s);
                dbg!(&hchats);
                history.push_back((chat, Instant::now()));
                queue.remove(i).unwrap().1.send(());
                allowed -= 1;
                dbg!(allowed);
            } else {
                i += 1;
            }
        }

        delay_for(DELAY).await;
    }
}

impl<B> Limited<B> {
    pub fn new(bot: B, limits: Limits) -> (Self, impl Future<Output = ()>) {
        let history = Arc::new(Mutex::new(VecDeque::with_capacity(
            max(limits.chat_s, max(limits.overall_s, limits.chat_m)) as _
        )));

        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let hchats = Arc::new(Mutex::new(HashMap::new()));

        let worker = worker(
            limits,
            Arc::clone(&queue),
            Arc::clone(&history),
            Arc::clone(&hchats),
        );

        let this = Self {
            //limits,
            bot,
            history,
            queue,
            hchats,
        };

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
        LimitedRequest(self.bot.send_message(chat_id, text), Arc::clone(&self.queue))
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

pub struct LimitedRequest<R>(R, Arc<Mutex<VecDeque<(ChatId, Sender<()>)>>>);

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

    fn send(self) -> Self::Send {
        let (tx, rx) = channel();
        // FIXME
        let mut g = block_on(self.1.lock());
        g.push_back((self.0.payload_ref().get_chat_id().clone(), tx));
        LimitedSend::Pending {
            request: self.0,
            wait: rx,
        }
    }

    fn send_ref(&self) -> Self::SendRef {
        unimplemented!()
    }
}

#[pin_project::pin_project(project = SendProj, project_replace = SendRepl)]
pub enum LimitedSend<R: Request> {
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
        println!("poll");
        match self.as_mut().project() {
            SendProj::Pending { request: _, wait } => match wait.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(r) => {
                    println!("pending-ready");
                    // FIXME(waffle): remove unwrap
                    r.unwrap();
                    if let SendRepl::Pending { request, wait: _ } = self.as_mut().project_replace(LimitedSend::Done) {
                        self.as_mut().project_replace(LimitedSend::Sent { fut: request.send() });
                    }

                    self.poll(cx)
                }
            },
            SendProj::Sent { fut } => {
                println!("sent");
                let res = futures::ready!(fut.poll(cx));
                println!("sent-ready");
                self.set(LimitedSend::Done);
                Poll::Ready(res)
            }
            SendProj::Done => Poll::Pending,
        }
    }
}
