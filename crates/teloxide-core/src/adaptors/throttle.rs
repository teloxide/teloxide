/// `ThrottlingRequest` and `ThrottlingSend` structures
mod request;
/// Lock that allows requests to wait until they are allowed to be sent
mod request_lock;
/// `impl Requester for Throttle<_>`
mod requester_impl;
/// `Settings` and `Limits` structures
mod settings;
/// "Worker" that checks the limits
mod worker;

use std::{
    future::Future,
    hash::{Hash, Hasher},
};

use tokio::sync::{
    mpsc,
    oneshot::{self},
};

use crate::{errors::AsResponseParameters, requests::Requester, types::*};

use self::{
    request_lock::{channel, RequestLock},
    worker::{worker, FreezeUntil, InfoMessage},
};

pub use request::{ThrottlingRequest, ThrottlingSend};
pub use settings::{Limits, Settings};

/// Automatic request limits respecting mechanism.
///
/// Telegram has strict [limits], which, if exceeded will sooner or later cause
/// `RequestError::RetryAfter(_)` errors. These errors can cause users of your
/// bot to never receive responses from the bot or receive them in a wrong
/// order.
///
/// This bot wrapper automatically checks for limits, suspending requests until
/// they could be sent without exceeding limits (request order in chats is not
/// changed).
///
/// It's recommended to use this wrapper before other wrappers (i.e.:
/// `SomeWrapper<Throttle<Bot>>` not `Throttle<SomeWrapper<Bot>>`) because if
/// done otherwise inner wrappers may cause `Throttle` to miscalculate limits
/// usage.
///
/// [limits]: https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this
///
/// ## Examples
///
/// ```no_run (throttle fails to spawn task without tokio runtime)
/// use teloxide_core::{adaptors::throttle::Limits, requests::RequesterExt, Bot};
///
/// let bot = Bot::new("TOKEN")
///     .throttle(Limits::default());
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
#[derive(Clone, Debug)]
pub struct Throttle<B> {
    bot: B,
    // `RequestLock` allows to unlock requests (allowing them to be sent).
    queue: mpsc::Sender<(ChatIdHash, RequestLock)>,
    info_tx: mpsc::Sender<InfoMessage>,
}

impl<B> Throttle<B> {
    /// Creates new [`Throttle`] alongside with worker future.
    ///
    /// Note: [`Throttle`] will only send requests if returned worker is
    /// polled/spawned/awaited.
    pub fn new(bot: B, limits: Limits) -> (Self, impl Future<Output = ()>)
    where
        B: Requester + Clone,
        B::Err: AsResponseParameters,
    {
        let settings = Settings { limits, ..<_>::default() };
        Self::with_settings(bot, settings)
    }

    /// Creates new [`Throttle`] alongside with worker future.
    ///
    /// Note: [`Throttle`] will only send requests if returned worker is
    /// polled/spawned/awaited.
    pub fn with_settings(bot: B, settings: Settings) -> (Self, impl Future<Output = ()>)
    where
        B: Requester + Clone,
        B::Err: AsResponseParameters,
    {
        let (tx, rx) = mpsc::channel(settings.limits.messages_per_sec_overall as usize);
        let (info_tx, info_rx) = mpsc::channel(2);

        let worker = worker(settings, rx, info_rx, bot.clone());
        let this = Self { bot, queue: tx, info_tx };

        (this, worker)
    }

    /// Creates new [`Throttle`] spawning the worker with `tokio::spawn`
    ///
    /// Note: it's recommended to use [`RequesterExt::throttle`] instead.
    ///
    /// [`RequesterExt::throttle`]: crate::requests::RequesterExt::throttle
    pub fn new_spawn(bot: B, limits: Limits) -> Self
    where
        B: Requester + Clone + Send + Sync + 'static,
        B::Err: AsResponseParameters,
        B::GetChat: Send,
    {
        let (this, worker) = Self::new(bot, limits);

        tokio::spawn(worker);

        this
    }

    /// Creates new [`Throttle`] spawning the worker with `tokio::spawn`
    pub fn spawn_with_settings(bot: B, settings: Settings) -> Self
    where
        B: Requester + Clone + Send + Sync + 'static,
        B::Err: AsResponseParameters,
        B::GetChat: Send,
    {
        let (this, worker) = Self::with_settings(bot, settings);

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

    /// Returns currently used [`Limits`].
    pub async fn limits(&self) -> Limits {
        const WORKER_DIED: &str = "worker died before last `Throttle` instance";

        let (tx, rx) = oneshot::channel();

        self.info_tx.send(InfoMessage::GetLimits { response: tx }).await.expect(WORKER_DIED);

        rx.await.expect(WORKER_DIED)
    }

    /// Sets new limits.
    ///
    /// Note: changes may not be applied immediately.
    pub async fn set_limits(&self, new: Limits) {
        let (tx, rx) = oneshot::channel();

        self.info_tx.send(InfoMessage::SetLimits { new, response: tx }).await.ok();

        rx.await.ok();
    }
}

/// An ID used in the worker.
///
/// It is used instead of `ChatId` to make copying cheap even in case of
/// usernames. (It is just a hashed username.)
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum ChatIdHash {
    Id(ChatId),
    ChannelUsernameHash(u64),
}

impl ChatIdHash {
    fn is_channel_or_supergroup(&self) -> bool {
        match self {
            &Self::Id(id) => id.is_channel_or_supergroup(),
            Self::ChannelUsernameHash(_) => true,
        }
    }
}

impl From<&ChatId> for ChatIdHash {
    fn from(value: &ChatId) -> Self {
        ChatIdHash::Id(*value)
    }
}

impl From<&Recipient> for ChatIdHash {
    fn from(value: &Recipient) -> Self {
        match value {
            Recipient::Id(id) => ChatIdHash::Id(*id),
            Recipient::ChannelUsername(username) => {
                // FIXME: this could probably use a faster hasher, `DefaultHasher` is known to
                //        be slow (it's not like we _need_ this to be fast, but still)
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                username.hash(&mut hasher);
                let hash = hasher.finish();
                ChatIdHash::ChannelUsernameHash(hash)
            }
        }
    }
}
