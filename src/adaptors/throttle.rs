use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    future::Future,
    hash::{Hash, Hasher},
    pin::Pin,
    time::{Duration, Instant},
};

use futures::task::{Context, Poll};
use never::Never;
use tokio::{
    sync::{
        mpsc::{self, error::TryRecvError},
        oneshot::{channel, Receiver, Sender},
    },
    time::delay_for,
};
use vecrem::VecExt;

use crate::{
    adaptors::throttle::chan_send::{ChanSend, SendTy},
    payloads::SendMessage,
    requests::{HasPayload, Output, Request, Requester},
    types::ChatId,
};

// Throttling is quite complicated this comment describes the algorithm of
// current implementation. NOTE: this only describes CURRENT implementation.
// Implementation may change at any time.
//
// ### Request
//
// When throttling request is sent, it sends a tuple of `ChatId` (more
// accurately, just local `Id`) and `Sender<()>` to the worker. Then the request
// waits for notification from worker. When notification is received it sends
// underlying request.
//
// ### Worker
//
// Worker does the most important job - it checks for limit exceed.
//
// The worker stores "history" of requests sent in last minute (and to which
// chats the were sent) and queue of pending updates.
//
// The worker does the following algorithm loop:
//
// 1. If queue is empty wait for the first message in incoming channel (and adds
//    it to queue).
//
// 2. Read all present messages from incoming channel and transfer them to
//    queue.
//
// 3. Record current time.
//
// 4. Clear history from records which time < (current - minute)
//
// 5. Count all requests in which were sent last second,
//    `allowed = limit.overall_s - count`
//
// 6. If `allowed == 0` wait a bit and `continue` to the next iteration
//
// 7. Count how many requests were sent to which chats (i.e.: create
//    `Map<ChatId, Count>`)    (note: the same map, but for last minute also
//    exists, but it's updated, instead of recreation)
//
// 8. While `allowed >= 0` search for requests which chat hasn't exceed limits
//    (i.e.: map[chat] < limit), if one is found, decrease `allowed`, notify
//    request that it can be now executed, increase counts, add record to
//    history.

const MINUTE: Duration = Duration::from_secs(60);
const SECOND: Duration = Duration::from_secs(1);

// Delay between worker iterations.
//
// For now it's `second/4`, but that number is chosen pretty randomly, we may
// want to change this.
const DELAY: Duration = Duration::from_millis(250);

/// Telegram request limits.
///
/// This struct is used in [`Throttle`].
///
/// Note that you may ask telegram [@BotSupport] to increase limits for your
/// particular bot if it has a lot of users (but they may or may not do that).
///
/// [@BotSupport]: https://t.me/botsupport
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

/// Automatic request limits respecting mechanism.
///
/// Telegram has strict [limits], which, if exceeded will sooner or later cause
/// `RequestError::RetryAfter(_)` errors. These errors can cause users of your
/// bot to never receive responds from the bot or receive them in wrong order.
///
/// This bot wrapper automatically checks for limits, suspending requests until
/// they could be sent without exceeding limits (request order in chats is not
/// changed).
///
/// It's recommended to use this wrapper before other wrappers (i.e.:
/// `SomeWrapper<Throttle<Bot>>`) because if done otherwise inner wrappers may
/// cause `Throttle` to miscalculate limits usage.
///
/// [limits]: https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this
///
/// ## Examples
///
/// ```no_run (throttle fails to spawn task without tokio runtime)
/// use teloxide_core::{adaptors::throttle::Limits, requests::RequesterExt, Bot};
///
/// # #[allow(deprecated)]
/// let bot = Bot::new("TOKEN").throttle(Limits::default());
///
/// /* send many requests here */
/// ```
///
/// ## Note about send-by-@channelusername
///
/// Telegram have limits on sending messages to _the same chat_. To check them
/// we store `chat_id`s of several last requests. _However_ there is no good way
/// to tell if given `ChatId::Id(x)` corresponds to the same chat as
/// `ChatId::ChannelUsername(u)`.
///
/// Our current approach is to just give up and check `chat_id_a == chat_id_b`.
/// This may give incorrect results.
///
/// As such, we encourage not to use `ChatId::ChannelUsername(u)` with this bot
/// wrapper.
pub struct Throttle<B> {
    bot: B,
    // Sender<Never> is used to pass the signal to unlock by closing the channel.
    queue: mpsc::Sender<(Id, Sender<Never>)>,
}

async fn worker(limits: Limits, mut queue_rx: mpsc::Receiver<(Id, Sender<Never>)>) {
    // +- Same idea as in `Throttle::new`
    let cap = limits.overall_s + (limits.overall_s / 4);
    // FIXME(waffle): Make an research about data structures for this queue.
    //                Currently this is O(n) removing (n = number of elements
    //                stayed), amortized O(1) push (vec+vecrem).
    let mut queue: Vec<(Id, Sender<Never>)> = Vec::with_capacity(cap as usize);

    // I wish there was special data structure for history which removed the
    // need in 2 hashmaps
    // (waffle)
    let mut history: VecDeque<(Id, Instant)> = VecDeque::new();
    // hchats[chat] = history.iter().filter(|(c, _)| c == chat).count()
    let mut hchats: HashMap<Id, u32> = HashMap::new();
    let mut hchats_s = HashMap::new();

    // set to true when `queue_rx` is closed
    let mut close = false;

    while !close || !queue.is_empty() {
        // If there are no pending requests we are just waiting
        if queue.is_empty() {
            match queue_rx.recv().await {
                Some(req) => queue.push(req),
                None => close = true,
            }
        }

        // update local queue with latest requests
        loop {
            match queue_rx.try_recv() {
                Ok(req) => queue.push(req),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Closed) => close = true,
            }
        }

        // _Maybe_ we need to use `spawn_blocking` here, because there is
        // decent amount of blocking work. However _for now_ I've decided not
        // to use it here.
        //
        // Reasons (not to use `spawn_blocking`):
        //
        // 1. The work seems not very CPU-bound, it's not heavy computations,
        //    it's more like light computations.
        //
        // 2. `spawn_blocking` is not zero-cost — it spawns a new system thread
        //    + do so other work. This may actually be *worse* then current
        //    "just do everything in this async fn" approach.
        //
        // 3. With `rt-threaded` feature, tokio uses [`num_cpus()`] threads
        //    which should be enough to work fine with one a-bit-blocking task.
        //    Crucially current behaviour will be problem mostly with
        //    single-threaded runtimes (and in case you're using one, you
        //    probably don't want to spawn unnecessary threads anyway).
        //
        // I think if we'll ever change this behaviour, we need to make it
        // _configurable_.
        //
        // See also [discussion (ru)].
        //
        // NOTE: If you are reading this because you have any problems because
        // of this worker, open an [issue on github]
        //
        // [`num_cpus()`]: https://vee.gg/JGwq2
        // [discussion (ru)]: https://t.me/rust_async/27891
        // [issue on github]: https://github.com/teloxide/teloxide/issues/new
        //
        // (waffle)

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
                let ent = hchats.entry(chat).and_modify(|count| {
                    *count -= 1;
                });

                if let Entry::Occupied(entry) = ent {
                    if *entry.get() == 0 {
                        entry.remove_entry();
                    }
                }
            }
        }

        // as truncates which is ok since in case of truncation it would always be >=
        // limits.overall_s
        let used = history.iter().take_while(|(_, time)| time > &sec_back).count() as u32;
        let mut allowed = limits.overall_s.saturating_sub(used);

        if allowed == 0 {
            hchats_s.clear();
            delay_for(DELAY).await;
            continue;
        }

        for (chat, _) in history.iter().take_while(|(_, time)| time > &sec_back) {
            *hchats_s.entry(*chat).or_insert(0) += 1;
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
                    *hchats_s.entry(*chat).or_insert(0) += 1;
                    *hchats.entry(*chat).or_insert(0) += 1;
                    history.push_back((*chat, Instant::now()));
                }

                // This will close the channel unlocking associated request
                drop(entry.remove());

                // We've "sent" 1 request, so now we can send 1 less
                allowed -= 1;
                if allowed == 0 {
                    break;
                }
            } else {
                entry.skip();
            }
        }
        drop(queue_rem);

        // It's easier to just recompute last second stats, instead of keeping
        // track of it alongside with minute stats, so we just throw this away.
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
        // A buffer made slightly bigger (112.5%) than overall limit
        // so we won't lose performance when hitting limits.
        //
        // (I hope this makes sense) (waffle)
        let buffer = limits.overall_s + (limits.overall_s / 8);
        let (queue_tx, queue_rx) = mpsc::channel(buffer as usize);

        let worker = worker(limits, queue_rx);
        let this = Self { bot, queue: queue_tx };

        (this, worker)
    }

    /// Creates new [`Throttle`] spawning the worker with `tokio::spawn`
    ///
    /// Note: it's recommended to use [`RequesterExt::throttle`] instead.
    ///
    /// [`RequesterExt::throttle`]: crate::requests::RequesterExt::throttle
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

/// Id used in worker.
///
/// It is used instead of `ChatId` to make copying cheap even in case of
/// usernames. (It just hashes username)
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Id {
    Id(i64),
    Ch(u64),
}

impl From<&ChatId> for Id {
    fn from(value: &ChatId) -> Self {
        match value {
            ChatId::Id(id) => Id::Id(*id),
            ChatId::ChannelUsername(username) => {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                username.hash(&mut hasher);
                let hash = hasher.finish();
                Id::Ch(hash)
            }
        }
    }
}

pub trait GetChatId {
    fn get_chat_id(&self) -> &ChatId;
}

impl GetChatId for SendMessage {
    fn get_chat_id(&self) -> &ChatId {
        &self.chat_id
    }
}

pub struct ThrottlingRequest<R>(R, mpsc::Sender<(Id, Sender<Never>)>);

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
    type Send = ThrottlingSend<R>;
    type SendRef = ThrottlingSendRef<R>;

    fn send(self) -> Self::Send {
        let (tx, rx) = channel();
        let send = self.1.send_t((self.0.payload_ref().get_chat_id().into(), tx));
        ThrottlingSend(ThrottlingSendInner::Registering { request: self.0, send, wait: rx })
    }

    fn send_ref(&self) -> Self::SendRef {
        let (tx, rx) = channel();
        let send = self.1.clone().send_t((self.0.payload_ref().get_chat_id().into(), tx));

        // As we can't move self.0 (request) out, as we do in `send` we are
        // forced to call `send_ref()`. This may have overhead and/or lead to
        // wrong results because `R::send_ref` does the send.
        //
        // However `Request` documentation explicitly notes that `send{,_ref}`
        // should **not** do any kind of work, so it's ok.
        let request = self.0.send_ref();

        ThrottlingSendRef(ThrottlingSendRefInner::Registering { request, send, wait: rx })
    }
}

#[pin_project::pin_project]
pub struct ThrottlingSend<R: Request>(#[pin] ThrottlingSendInner<R>);

#[pin_project::pin_project(project = SendProj, project_replace = SendRepl)]
enum ThrottlingSendInner<R: Request> {
    Registering {
        request: R,
        #[pin]
        send: ChanSend,
        wait: Receiver<Never>,
    },
    Pending {
        request: R,
        #[pin]
        wait: Receiver<Never>,
    },
    Sent {
        #[pin]
        fut: R::Send,
    },
    Done,
}

impl<R: Request> Future for ThrottlingSend<R> {
    type Output = Result<Output<R>, R::Err>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project().0;

        match this.as_mut().project() {
            SendProj::Registering { request: _, send, wait: _ } => match send.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(res) => {
                    if let SendRepl::Registering { request, send: _, wait } =
                        this.as_mut().project_replace(ThrottlingSendInner::Done)
                    {
                        match res {
                            Ok(()) => this
                                .as_mut()
                                .project_replace(ThrottlingSendInner::Pending { request, wait }),
                            // The worker is unlikely to drop queue before sending all requests,
                            // but just in case it has dropped the queue, we want to just send the
                            // request.
                            Err(_) => this
                                .as_mut()
                                .project_replace(ThrottlingSendInner::Sent { fut: request.send() }),
                        };
                    }

                    self.poll(cx)
                }
            },
            SendProj::Pending { request: _, wait } => match wait.poll(cx) {
                Poll::Pending => Poll::Pending,
                // Worker pass "message" to unlock us by closing the channel,
                // and thus we can safely ignore this result as we know it will
                // always be `Err(_)` (because `Ok(Never)` is uninhibited)
                // and that's what we want.
                Poll::Ready(_) => {
                    if let SendRepl::Pending { request, wait: _ } =
                        this.as_mut().project_replace(ThrottlingSendInner::Done)
                    {
                        this.as_mut()
                            .project_replace(ThrottlingSendInner::Sent { fut: request.send() });
                    }

                    self.poll(cx)
                }
            },
            SendProj::Sent { fut } => {
                let res = futures::ready!(fut.poll(cx));
                this.set(ThrottlingSendInner::Done);
                Poll::Ready(res)
            }
            SendProj::Done => Poll::Pending,
        }
    }
}

#[pin_project::pin_project]
pub struct ThrottlingSendRef<R: Request>(#[pin] ThrottlingSendRefInner<R>);

#[pin_project::pin_project(project = SendRefProj, project_replace = SendRefRepl)]
enum ThrottlingSendRefInner<R: Request> {
    Registering {
        request: R::SendRef,
        #[pin]
        send: ChanSend,
        wait: Receiver<Never>,
    },
    Pending {
        request: R::SendRef,
        #[pin]
        wait: Receiver<Never>,
    },
    Sent {
        #[pin]
        fut: R::SendRef,
    },
    Done,
}

impl<R: Request> Future for ThrottlingSendRef<R> {
    type Output = Result<Output<R>, R::Err>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project().0;

        match this.as_mut().project() {
            SendRefProj::Registering { request: _, send, wait: _ } => match send.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(res) => {
                    if let SendRefRepl::Registering { request, send: _, wait } =
                        this.as_mut().project_replace(ThrottlingSendRefInner::Done)
                    {
                        match res {
                            Ok(()) => this
                                .as_mut()
                                .project_replace(ThrottlingSendRefInner::Pending { request, wait }),
                            // The worker is unlikely to drop queue before sending all requests,
                            // but just in case it has dropped the queue, we want to just send the
                            // request.
                            Err(_) => this
                                .as_mut()
                                .project_replace(ThrottlingSendRefInner::Sent { fut: request }),
                        };
                    }

                    self.poll(cx)
                }
            },
            SendRefProj::Pending { request: _, wait } => match wait.poll(cx) {
                Poll::Pending => Poll::Pending,
                // Worker pass "message" to unlock us by closing the channel,
                // and thus we can safely ignore this result as we know it will
                // always be `Err(_)` (because `Ok(Never)` is uninhibited)
                // and that's what we want.
                Poll::Ready(_) => {
                    if let SendRefRepl::Pending { request, wait: _ } =
                        this.as_mut().project_replace(ThrottlingSendRefInner::Done)
                    {
                        this.as_mut()
                            .project_replace(ThrottlingSendRefInner::Sent { fut: request });
                    }

                    self.poll(cx)
                }
            },
            SendRefProj::Sent { fut } => {
                let res = futures::ready!(fut.poll(cx));
                this.set(ThrottlingSendRefInner::Done);
                Poll::Ready(res)
            }
            SendRefProj::Done => Poll::Pending,
        }
    }
}

mod chan_send {
    use std::{future::Future, pin::Pin};

    use futures::task::{Context, Poll};
    use never::Never;
    use tokio::sync::{mpsc, mpsc::error::SendError, oneshot::Sender};

    use crate::adaptors::throttle::Id;

    pub(super) trait SendTy {
        fn send_t(self, val: (Id, Sender<Never>)) -> ChanSend;
    }

    #[pin_project::pin_project]
    pub(super) struct ChanSend(#[pin] Inner);

    #[cfg(not(feature = "nightly"))]
    type Inner = Pin<Box<dyn Future<Output = Result<(), SendError<(Id, Sender<Never>)>>>>>;
    #[cfg(feature = "nightly")]
    type Inner = impl Future<Output = Result<(), SendError<(Id, Sender<Never>)>>>;

    impl SendTy for mpsc::Sender<(Id, Sender<Never>)> {
        // `return`s trick IDEA not to show errors
        #[allow(clippy::needless_return)]
        fn send_t(self, val: (Id, Sender<Never>)) -> ChanSend {
            #[cfg(feature = "nightly")]
            {
                fn def(
                    mut sender: mpsc::Sender<(Id, Sender<Never>)>,
                    val: (Id, Sender<Never>),
                ) -> Inner {
                    async move { sender.send(val).await }
                }
                return ChanSend(def(self, val));
            }
            #[cfg(not(feature = "nightly"))]
            {
                let mut this = self;
                return ChanSend(Box::pin(async move { this.send(val).await }));
            }
        }
    }

    impl Future for ChanSend {
        type Output = Result<(), SendError<(Id, Sender<Never>)>>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.project().0.poll(cx)
        }
    }
}
