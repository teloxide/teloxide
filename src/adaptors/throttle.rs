use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    future::Future,
    hash::{Hash, Hasher},
    pin::Pin,
    time::{Duration, Instant},
};

use futures::{
    future::ready,
    task::{Context, Poll},
    FutureExt,
};
use tokio::sync::{
    mpsc,
    oneshot::{self, Receiver, Sender},
};
use url::Url;
use vecrem::VecExt;

use crate::{
    adaptors::throttle::chan_send::{ChanSend, MpscSend},
    errors::AsResponseParameters,
    requests::{HasPayload, Output, Request, Requester},
    types::*,
};

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
        let settings = Settings {
            limits,
            ..<_>::default()
        };
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
        let this = Self {
            bot,
            queue: tx,
            info_tx,
        };

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
        let (tx, rx) = oneshot::channel();

        self.info_tx
            .send(InfoMessage::GetLimits { response: tx })
            .await
            .expect(WORKER_DIED);

        rx.await.expect(WORKER_DIED)
    }

    /// Sets new limits.
    ///
    /// Note: changes may not be applied immediately.
    pub async fn set_limits(&self, new: Limits) {
        let (tx, rx) = oneshot::channel();

        self.info_tx
            .send(InfoMessage::SetLimits { new, response: tx })
            .await
            .ok();

        rx.await.ok();
    }
}

/// Telegram request limits.
///
/// This struct is used in [`Throttle`].
///
/// Note that you may ask telegram [@BotSupport] to increase limits for your
/// particular bot if it has a lot of users (but they may or may not do that).
///
/// [@BotSupport]: https://t.me/botsupport
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Limits {
    /// Allowed messages in one chat per second.
    pub messages_per_sec_chat: u32,

    /// Allowed messages in one chat per minute.
    pub messages_per_min_chat: u32,

    /// Allowed messages in one channel per minute.
    pub messages_per_min_channel: u32,

    /// Allowed messages per second.
    pub messages_per_sec_overall: u32,
}

/// Defaults are taken from [telegram documentation][tgdoc] (except for
/// `messages_per_min_channel`).
///
/// [tgdoc]: https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this
impl Default for Limits {
    fn default() -> Self {
        Self {
            messages_per_sec_chat: 1,
            messages_per_sec_overall: 30,
            messages_per_min_chat: 20,
            messages_per_min_channel: 10,
        }
    }
}

/// Settings used by [`Throttle`] adaptor.
///
/// ## Examples
///
/// ```
/// use teloxide_core::adaptors::throttle;
///
/// let settings = throttle::Settings::default()
///     .on_queue_full(|pending| async move { /* do something when internal queue is full */ });
/// // use settings in `Throttle::with_settings` or other constructors
/// # let _ = settings;
/// ```
#[non_exhaustive]
pub struct Settings {
    pub limits: Limits,
    pub on_queue_full: BoxedFnMut<usize, BoxedFuture>,
    pub retry: bool,
    pub check_slow_mode: bool,
}

impl Settings {
    pub fn limits(mut self, val: Limits) -> Self {
        self.limits = val;
        self
    }

    pub fn on_queue_full<F, Fut>(mut self, mut val: F) -> Self
    where
        F: FnMut(usize) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.on_queue_full = Box::new(move |pending| Box::pin(val(pending)));
        self
    }

    pub fn no_retry(mut self) -> Self {
        self.retry = false;
        self
    }

    pub fn check_slow_mode(mut self) -> Self {
        self.check_slow_mode = true;
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            limits: <_>::default(),
            on_queue_full: Box::new(|pending| {
                log::warn!("Throttle queue is full ({} pending requests)", pending);
                Box::pin(ready(()))
            }),
            retry: true,
            check_slow_mode: false,
        }
    }
}

// Required to not trigger `clippy::type-complexity` lint
type BoxedFnMut<I, O> = Box<dyn FnMut(I) -> O + Send>;
type BoxedFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

const WORKER_DIED: &str = "worker died before last `Throttle` instance";

const MINUTE: Duration = Duration::from_secs(60);
const SECOND: Duration = Duration::from_secs(1);

// Delay between worker iterations.
//
// For now it's `second/4`, but that number is chosen pretty randomly, we may
// want to change this.
const DELAY: Duration = Duration::from_millis(250);

/// Minimal time between calls to queue_full function
const QUEUE_FULL_DELAY: Duration = Duration::from_secs(4);

#[derive(Debug)]
enum InfoMessage {
    GetLimits { response: Sender<Limits> },
    SetLimits { new: Limits, response: Sender<()> },
}

type RequestsSent = u32;

// I wish there was special data structure for history which removed the
// need in 2 hashmaps
// (waffle)
#[derive(Default)]
struct RequestsSentToChats {
    per_min: HashMap<ChatIdHash, RequestsSent>,
    per_sec: HashMap<ChatIdHash, RequestsSent>,
}

struct FreezeUntil {
    until: Instant,
    after: Duration,
    chat: ChatIdHash,
    retry: Option<RequestLock>,
}

// Throttling is quite complicated. This comment describes the algorithm of the
// current implementation.
//
// ### Request
//
// When a throttling request is sent, it sends a tuple of `ChatId` and
// `Sender<()>` to the worker. Then the request waits for a notification from
// the worker. When notification is received, it sends the underlying request.
//
// ### Worker
//
// The worker does the most important job -- it ensures that the limits are
// never exceeded.
//
// The worker stores a history of requests sent in the last minute (and to which
// chats they were sent) and a queue of pending updates.
//
// The worker does the following algorithm loop:
//
// 1. If the queue is empty, wait for the first message in incoming channel (and
// add it to the queue).
//
// 2. Read all present messages from an incoming channel and transfer them to
// the queue.
//
// 3. Record the current time.
//
// 4. Clear the history from records whose time < (current time - minute).
//
// 5. Count all requests which were sent last second, `allowed =
// limit.messages_per_sec_overall - count`.
//
// 6. If `allowed == 0` wait a bit and `continue` to the next iteration.
//
// 7. Count how many requests were sent to which chats (i.e.: create
// `Map<ChatId, Count>`). (Note: the same map, but for last minute also exists,
// but it's updated, instead of recreation.)
//
// 8. While `allowed >= 0` search for requests which chat haven't exceed the
// limits (i.e.: map[chat] < limit), if one is found, decrease `allowed`, notify
// the request that it can be now executed, increase counts, add record to the
// history.

async fn worker<B>(
    Settings {
        mut limits,
        mut on_queue_full,
        retry,
        check_slow_mode,
    }: Settings,
    mut rx: mpsc::Receiver<(ChatIdHash, RequestLock)>,
    mut info_rx: mpsc::Receiver<InfoMessage>,
    bot: B,
) where
    B: Requester,
    B::Err: AsResponseParameters,
{
    // FIXME(waffle): Make an research about data structures for this queue.
    //                Currently this is O(n) removing (n = number of elements
    //                stayed), amortized O(1) push (vec+vecrem).
    let mut queue: Vec<(ChatIdHash, RequestLock)> =
        Vec::with_capacity(limits.messages_per_sec_overall as usize);

    let mut history: VecDeque<(ChatIdHash, Instant)> = VecDeque::new();
    let mut requests_sent = RequestsSentToChats::default();

    let mut slow_mode: Option<HashMap<ChatIdHash, (Duration, Instant)>> =
        check_slow_mode.then(HashMap::new);

    let mut rx_is_closed = false;

    let mut last_queue_full = Instant::now()
        .checked_sub(QUEUE_FULL_DELAY)
        .unwrap_or_else(Instant::now);

    let (freeze_tx, mut freeze_rx) = mpsc::channel::<FreezeUntil>(1);

    while !rx_is_closed || !queue.is_empty() {
        // FIXME(waffle):
        // 1. If the `queue` is empty, `read_from_rx` call down below will 'block'
        //    execution until a request is sent. While the execution is 'blocked' no
        //    `InfoMessage`s could be answered.
        //
        // 2. If limits are decreased, ideally we want to shrink queue.
        //
        // *blocked in asynchronous way
        answer_info(&mut info_rx, &mut limits);

        freeze(
            &mut freeze_rx,
            &freeze_tx,
            slow_mode.as_mut(),
            &mut queue,
            &bot,
            None,
        )
        .await;

        loop {
            tokio::select! {
                () = read_from_rx(&mut rx, &mut queue, &mut rx_is_closed) => break,
                freeze_until = freeze_rx.recv() => {
                    freeze(
                        &mut freeze_rx,
                        &freeze_tx,
                        slow_mode.as_mut(),
                        &mut queue,
                        &bot,
                        freeze_until
                    )
                    .await;
                },
            }
        }
        //debug_assert_eq!(queue.capacity(), limits.messages_per_sec_overall as usize);

        if queue.len() == queue.capacity() && last_queue_full.elapsed() > QUEUE_FULL_DELAY {
            last_queue_full = Instant::now();
            tokio::spawn(on_queue_full(queue.len()));
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

        // make history and requests_sent up-to-date
        while let Some((_, time)) = history.front() {
            // history is sorted, we found first up-to-date thing
            if time >= &min_back {
                break;
            }

            if let Some((chat, _)) = history.pop_front() {
                let entry = requests_sent.per_min.entry(chat).and_modify(|count| {
                    *count -= 1;
                });

                if let Entry::Occupied(entry) = entry {
                    if *entry.get() == 0 {
                        entry.remove_entry();
                    }
                }
            }
        }

        // as truncates which is ok since in case of truncation it would always be >=
        // limits.overall_s
        let used = history
            .iter()
            .take_while(|(_, time)| time > &sec_back)
            .count() as u32;
        let mut allowed = limits.messages_per_sec_overall.saturating_sub(used);

        if allowed == 0 {
            requests_sent.per_sec.clear();
            tokio::time::sleep(DELAY).await;
            continue;
        }

        for (chat, _) in history.iter().take_while(|(_, time)| time > &sec_back) {
            *requests_sent.per_sec.entry(*chat).or_insert(0) += 1;
        }

        let mut queue_removing = queue.removing();

        while let Some(entry) = queue_removing.next() {
            let chat = &entry.value().0;

            let slow_mode = slow_mode.as_mut().and_then(|sm| sm.get_mut(chat));

            if let Some(&mut (delay, last)) = slow_mode {
                if last + delay > Instant::now() {
                    continue;
                }
            }

            let requests_sent_per_sec_count = requests_sent.per_sec.get(chat).copied().unwrap_or(0);
            let requests_sent_per_min_count = requests_sent.per_min.get(chat).copied().unwrap_or(0);

            let messages_per_min_limit = if chat.is_channel() {
                limits.messages_per_min_channel
            } else {
                limits.messages_per_min_chat
            };

            let limits_not_exceeded = requests_sent_per_sec_count < limits.messages_per_sec_chat
                && requests_sent_per_min_count < messages_per_min_limit;

            if limits_not_exceeded {
                // Unlock the associated request.

                let chat = *chat;
                let (_, lock) = entry.remove();

                // Only count request as sent if the request wasn't dropped before unlocked
                if lock.unlock(retry, freeze_tx.clone()).is_ok() {
                    *requests_sent.per_sec.entry(chat).or_insert(0) += 1;
                    *requests_sent.per_min.entry(chat).or_insert(0) += 1;
                    history.push_back((chat, Instant::now()));

                    if let Some((_, last)) = slow_mode {
                        *last = Instant::now();
                    }

                    // We have "sent" one request, so now we can send one less.
                    allowed -= 1;
                    if allowed == 0 {
                        break;
                    }
                }
            }
        }

        // It's easier to just recompute last second stats, instead of keeping
        // track of it alongside with minute stats, so we just throw this away.
        requests_sent.per_sec.clear();
        tokio::time::sleep(DELAY).await;
    }
}

fn answer_info(rx: &mut mpsc::Receiver<InfoMessage>, limits: &mut Limits) {
    // FIXME(waffle): https://github.com/tokio-rs/tokio/issues/3350
    while let Some(Some(req)) = tokio::task::unconstrained(rx.recv()).now_or_never() {
        // Errors are ignored with .ok(). Error means that the response channel
        // is closed and the response isn't needed.
        match req {
            InfoMessage::GetLimits { response } => response.send(*limits).ok(),
            InfoMessage::SetLimits { new, response } => {
                *limits = new;
                response.send(()).ok()
            }
        };
    }
}

async fn freeze(
    rx: &mut mpsc::Receiver<FreezeUntil>,
    tx: &mpsc::Sender<FreezeUntil>,
    mut slow_mode: Option<&mut HashMap<ChatIdHash, (Duration, Instant)>>,
    queue: &mut Vec<(ChatIdHash, RequestLock)>,
    bot: &impl Requester,
    mut imm: Option<FreezeUntil>,
) {
    // FIXME(waffle): https://github.com/tokio-rs/tokio/issues/3350
    while let Some(FreezeUntil {
        until,
        after,
        chat,
        mut retry,
    }) = imm.take().or_else(|| {
        tokio::task::unconstrained(rx.recv())
            .now_or_never()
            .flatten()
    }) {
        if let Some(slow_mode) = slow_mode.as_deref_mut() {
            // TODO: do something with channels?...
            if let hash @ ChatIdHash::Id(id) = chat {
                // TODO: maybe not call `get_chat` every time?

                // At this point there isn't much we can do with the error besides ignoring
                if let Ok(chat) = bot.get_chat(id).send().await {
                    match chat.slow_mode_delay() {
                        Some(delay) => {
                            let now = Instant::now();
                            let new_delay = Duration::from_secs(delay.into());
                            slow_mode.insert(hash, (new_delay, now));
                        }
                        None => {
                            slow_mode.remove(&hash);
                        }
                    };
                }
            }
        }

        // slow mode is enabled and it is <= to the delay asked by telegram
        let slow_mode_enabled_and_likely_the_cause = slow_mode
            .as_ref()
            .and_then(|m| m.get(&chat).map(|(delay, _)| delay <= &after))
            .unwrap_or(false);

        // Do not sleep if slow mode is enabled since the freeze is most likely caused
        // by the said slow mode and not by the global limits.
        if slow_mode_enabled_and_likely_the_cause {
            queue.extend(Some(chat).zip(retry.take()));
        } else {
            log::warn!(
                "freezing the bot for approximately {:?} due to `RetryAfter` error from telegram",
                after
            );

            tokio::time::sleep_until(until.into()).await;

            log::warn!("unfreezing the bot");

            if let Some(lock) = retry {
                // Since we are already retrying the request, retries are obviously turned on.
                let retry = true;
                let _ = lock.unlock(retry, tx.clone());
            }
        }
    }
}

async fn read_from_rx<T>(rx: &mut mpsc::Receiver<T>, queue: &mut Vec<T>, rx_is_closed: &mut bool) {
    if queue.is_empty() {
        log::warn!("A-blocking on queue");
        match rx.recv().await {
            Some(req) => queue.push(req),
            None => *rx_is_closed = true,
        }
    }

    // Don't grow queue bigger than the capacity to limit DOS possibility
    while queue.len() < queue.capacity() {
        // FIXME(waffle): https://github.com/tokio-rs/tokio/issues/3350
        match tokio::task::unconstrained(rx.recv()).now_or_never() {
            Some(Some(req)) => queue.push(req),
            Some(None) => *rx_is_closed = true,
            // There are no items in queue.
            None => break,
        }
    }
}

macro_rules! f {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        ThrottlingRequest {
            request: Arc::new($this.inner().$m($($arg),*)),
            chat_id: |p| (&p.payload_ref().chat_id).into(),
            worker: $this.queue.clone(),
        }
    };
}

macro_rules! fty {
    ($T:ident) => {
        ThrottlingRequest<B::$T>
    };
}

macro_rules! fid {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        $this.inner().$m($($arg),*)
    };
}

macro_rules! ftyid {
    ($T:ident) => {
        B::$T
    };
}

impl<B: Requester> Requester for Throttle<B>
where
    B::Err: AsResponseParameters,

    B::SendMessage: Clone + Send + Sync,
    B::ForwardMessage: Clone + Send + Sync,
    B::CopyMessage: Clone + Send + Sync,
    B::SendPhoto: Clone + Send + Sync,
    B::SendAudio: Clone + Send + Sync,
    B::SendDocument: Clone + Send + Sync,
    B::SendVideo: Clone + Send + Sync,
    B::SendAnimation: Clone + Send + Sync,
    B::SendVoice: Clone + Send + Sync,
    B::SendVideoNote: Clone + Send + Sync,
    B::SendMediaGroup: Clone + Send + Sync,
    B::SendLocation: Clone + Send + Sync,
    B::SendVenue: Clone + Send + Sync,
    B::SendContact: Clone + Send + Sync,
    B::SendPoll: Clone + Send + Sync,
    B::SendDice: Clone + Send + Sync,
    B::SendSticker: Clone + Send + Sync,
    B::SendInvoice: Clone + Send + Sync,
{
    type Err = B::Err;

    requester_forward! {
        send_message, forward_message, copy_message, send_photo, send_audio,
        send_document, send_video, send_animation, send_voice, send_video_note,
        send_media_group, send_location, send_venue, send_contact, send_poll,
        send_dice, send_sticker, send_invoice => f, fty
    }

    requester_forward! {
        get_me, log_out, close, get_updates, set_webhook, delete_webhook, get_webhook_info,
        edit_message_live_location, edit_message_live_location_inline,
        stop_message_live_location, stop_message_live_location_inline,
        send_chat_action, get_user_profile_photos, get_file, kick_chat_member, ban_chat_member,
        unban_chat_member, restrict_chat_member, promote_chat_member,
        set_chat_administrator_custom_title,
        ban_chat_sender_chat, unban_chat_sender_chat, set_chat_permissions,
        export_chat_invite_link, create_chat_invite_link, edit_chat_invite_link,
        revoke_chat_invite_link, set_chat_photo, delete_chat_photo, set_chat_title,
        set_chat_description, pin_chat_message, unpin_chat_message, unpin_all_chat_messages,
        leave_chat, get_chat, get_chat_administrators, get_chat_members_count, get_chat_member_count,
        get_chat_member, set_chat_sticker_set, delete_chat_sticker_set,
        answer_callback_query, set_my_commands, get_my_commands, delete_my_commands, answer_inline_query,
        edit_message_text, edit_message_text_inline, edit_message_caption,
        edit_message_caption_inline, edit_message_media, edit_message_media_inline,
        edit_message_reply_markup, edit_message_reply_markup_inline, stop_poll,
        delete_message, get_sticker_set, upload_sticker_file, create_new_sticker_set,
        add_sticker_to_set, set_sticker_position_in_set, delete_sticker_from_set,
        set_sticker_set_thumb, answer_shipping_query, answer_pre_checkout_query,
        set_passport_data_errors, send_game, set_game_score, set_game_score_inline,
        approve_chat_join_request, decline_chat_join_request,
        get_game_high_scores => fid, ftyid
    }
}

download_forward! {
    'w
    B
    Throttle<B>
    { this => this.inner() }
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
    fn is_channel(&self) -> bool {
        match self {
            &Self::Id(id) => id.is_channel(),
            Self::ChannelUsernameHash(_) => true,
        }
    }
}

impl From<&Recipient> for ChatIdHash {
    fn from(value: &Recipient) -> Self {
        match value {
            Recipient::Id(id) => ChatIdHash::Id(*id),
            Recipient::ChannelUsername(username) => {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                username.hash(&mut hasher);
                let hash = hasher.finish();
                ChatIdHash::ChannelUsernameHash(hash)
            }
        }
    }
}

#[must_use = "Requests are lazy and do nothing unless sent"]
pub struct ThrottlingRequest<R: HasPayload> {
    request: Arc<R>,
    chat_id: fn(&R::Payload) -> ChatIdHash,
    worker: mpsc::Sender<(ChatIdHash, RequestLock)>,
}

impl<R: HasPayload + Clone> HasPayload for ThrottlingRequest<R> {
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        Arc::make_mut(&mut self.request).payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.request.payload_ref()
    }
}

impl<R> Request for ThrottlingRequest<R>
where
    R: Request + Clone + Send + Sync,
    R::Err: AsResponseParameters,
    Output<R>: Send,
{
    type Err = R::Err;
    type Send = ThrottlingSend<R>;
    type SendRef = ThrottlingSend<R>;

    fn send(self) -> Self::Send {
        let (tx, rx) = channel();

        let chat_id = (self.chat_id)(self.payload_ref());
        let send = self.worker.send1((chat_id, tx));

        let inner = ThrottlingSendInner::Registering {
            request: Arc::try_unwrap(self.request).into(),
            chat: chat_id,
            send,
            wait: rx,
        };
        ThrottlingSend(inner)
    }

    fn send_ref(&self) -> Self::SendRef {
        let (tx, rx) = channel();

        let chat_id = (self.chat_id)(self.payload_ref());
        let send = self.worker.clone().send1((chat_id, tx));

        // As we can't move self.0 (request) out, as we do in `send` we are
        // forced to call `send_ref()`. This may have overhead and/or lead to
        // wrong results because `R::send_ref` does the send.
        //
        // However `Request` documentation explicitly notes that `send{,_ref}`
        // should **not** do any kind of work, so it's ok.
        let request = Either::Left(Arc::clone(&self.request));

        let inner = ThrottlingSendInner::Registering {
            chat: chat_id,
            request,
            send,
            wait: rx,
        };
        ThrottlingSend(inner)
    }
}

use either::Either;
use std::sync::Arc;

#[pin_project::pin_project]
pub struct ThrottlingSend<R: Request>(#[pin] ThrottlingSendInner<R>);

#[pin_project::pin_project(project = SendProj, project_replace = SendRepl)]
enum ThrottlingSendInner<R: Request> {
    Registering {
        request: Either<Arc<R>, R>,
        chat: ChatIdHash,
        #[pin]
        send: ChanSend<(ChatIdHash, RequestLock)>,
        wait: RequestWaiter,
    },
    Freezing {
        #[pin]
        freeze_fut: ChanSend<FreezeUntil>,
        res: Result<Output<R>, R::Err>,
    },
    FreezingRetry {
        #[pin]
        freeze_fut: ChanSend<FreezeUntil>,
        request: Either<Arc<R>, R>,
        chat: ChatIdHash,
        wait: RequestWaiter,
    },
    Pending {
        request: Either<Arc<R>, R>,
        chat: ChatIdHash,
        #[pin]
        wait: RequestWaiter,
    },
    Sent {
        freeze: mpsc::Sender<FreezeUntil>,
        chat: ChatIdHash,
        #[pin]
        fut: R::Send,
    },
    SentRef {
        freeze: mpsc::Sender<FreezeUntil>,
        chat: ChatIdHash,
        #[pin]
        fut: R::SendRef,
    },
    SentRetryable {
        request: Either<Arc<R>, R>,
        chat: ChatIdHash,
        freeze: mpsc::Sender<FreezeUntil>,
        #[pin]
        fut: R::SendRef,
    },

    Done,
}

impl<R: Request> Future for ThrottlingSend<R>
where
    R::Err: AsResponseParameters,
{
    type Output = Result<Output<R>, R::Err>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project().0;

        match this.as_mut().project() {
            SendProj::Registering { send, .. } => match send.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(res) => {
                    if let SendRepl::Registering {
                        request,
                        send: _,
                        wait,
                        chat,
                    } = this.as_mut().project_replace(ThrottlingSendInner::Done)
                    {
                        match res {
                            Ok(()) => this.as_mut().set(ThrottlingSendInner::Pending {
                                request,
                                wait,
                                chat,
                            }),
                            // The worker is unlikely to drop queue before sending all requests,
                            // but just in case it has dropped the queue, we want to just send the
                            // request.
                            Err(_) => this.as_mut().set(match request {
                                Either::Left(shared) => ThrottlingSendInner::SentRef {
                                    freeze: mpsc::channel(1).0,
                                    fut: shared.send_ref(),
                                    chat,
                                },
                                Either::Right(owned) => ThrottlingSendInner::Sent {
                                    freeze: mpsc::channel(1).0,
                                    fut: owned.send(),
                                    chat,
                                },
                            }),
                        };
                    }

                    self.poll(cx)
                }
            },
            SendProj::Pending { wait, .. } => match wait.poll(cx) {
                Poll::Pending => Poll::Pending,
                // Worker pass "message" to unlock us by closing the channel,
                // and thus we can safely ignore this result as we know it will
                // always be `Err(_)` (because `Ok(Never)` is uninhibited)
                // and that's what we want.
                Poll::Ready((retry, freeze)) => {
                    if let SendRepl::Pending { request, chat, .. } =
                        this.as_mut().project_replace(ThrottlingSendInner::Done)
                    {
                        let repl = match (retry, request) {
                            (true, request) => ThrottlingSendInner::SentRetryable {
                                fut: request.as_ref().either(|r| &**r, |r| r).send_ref(),
                                chat,
                                request,
                                freeze,
                            },
                            (false, Either::Left(shared)) => ThrottlingSendInner::SentRef {
                                fut: shared.send_ref(),
                                chat,
                                freeze,
                            },
                            (false, Either::Right(owned)) => ThrottlingSendInner::Sent {
                                fut: owned.send(),
                                chat,
                                freeze,
                            },
                        };

                        this.as_mut().project_replace(repl);
                    }

                    self.poll(cx)
                }
            },
            SendProj::Freezing { freeze_fut, .. } => {
                // Error here means that the worker died, so we can't really do anything about
                // it
                let _ = futures::ready!(freeze_fut.poll(cx));
                if let SendRepl::Freezing { res, .. } =
                    this.as_mut().project_replace(ThrottlingSendInner::Done)
                {
                    Poll::Ready(res)
                } else {
                    // The match above guarantees that this is unreachable
                    unreachable!()
                }
            }
            SendProj::FreezingRetry { freeze_fut, .. } => {
                // Error here means that the worker died, so we can't really do anything about
                // it
                let _ = futures::ready!(freeze_fut.poll(cx));

                if let SendRepl::FreezingRetry {
                    request,
                    chat,
                    wait,
                    ..
                } = this.as_mut().project_replace(ThrottlingSendInner::Done)
                {
                    this.as_mut().set(ThrottlingSendInner::Pending {
                        request,
                        chat,
                        wait,
                    });

                    self.poll(cx)
                } else {
                    unreachable!()
                }
            }
            SendProj::Sent { fut, freeze, chat } => {
                let res = futures::ready!(fut.poll(cx));

                if let Err(Some(retry_after)) = res.as_ref().map_err(<_>::retry_after) {
                    let after = Duration::from_secs(retry_after.into());
                    let freeze_fut = freeze.clone().send1(FreezeUntil {
                        until: Instant::now() + after,
                        after,
                        chat: *chat,
                        retry: None,
                    });

                    this.set(ThrottlingSendInner::Freezing { freeze_fut, res });
                    self.poll(cx)
                } else {
                    this.set(ThrottlingSendInner::Done);
                    Poll::Ready(res)
                }
            }
            SendProj::SentRef { freeze, fut, chat } => {
                let res = futures::ready!(fut.poll(cx));

                if let Err(Some(retry_after)) = res.as_ref().map_err(<_>::retry_after) {
                    let after = Duration::from_secs(retry_after.into());
                    let freeze_fut = freeze.clone().send1(FreezeUntil {
                        until: Instant::now() + after,
                        after,
                        chat: *chat,
                        retry: None,
                    });

                    this.set(ThrottlingSendInner::Freezing { freeze_fut, res });
                    self.poll(cx)
                } else {
                    this.set(ThrottlingSendInner::Done);
                    Poll::Ready(res)
                }
            }
            SendProj::SentRetryable { fut, .. } => {
                let res = futures::ready!(fut.poll(cx));
                let (lock, wait) = channel();

                if let Err(Some(retry_after)) = res.as_ref().map_err(<_>::retry_after) {
                    let after = Duration::from_secs(retry_after.into());

                    if let SendRepl::SentRetryable {
                        request,
                        freeze,
                        chat,
                        ..
                    } = this.as_mut().project_replace(ThrottlingSendInner::Done)
                    {
                        log::warn!("Freezing, before retrying: {}", retry_after);
                        let freeze_fut = freeze.send1(FreezeUntil {
                            until: Instant::now(),
                            after,
                            chat,
                            retry: Some(lock),
                        });
                        this.as_mut().set(ThrottlingSendInner::FreezingRetry {
                            freeze_fut,
                            request,
                            chat,
                            wait,
                        })
                    }

                    self.poll(cx)
                } else {
                    this.set(ThrottlingSendInner::Done);
                    Poll::Ready(res)
                }
            }
            SendProj::Done => {
                log::error!("Polling done");
                Poll::Pending
            }
        }
    }
}

fn channel() -> (RequestLock, RequestWaiter) {
    let (tx, rx) = oneshot::channel();
    let tx = RequestLock(tx);
    let rx = RequestWaiter(rx);
    (tx, rx)
}

#[must_use]
struct RequestLock(Sender<(bool, mpsc::Sender<FreezeUntil>)>);

impl RequestLock {
    fn unlock(self, retry: bool, freeze: mpsc::Sender<FreezeUntil>) -> Result<(), ()> {
        self.0.send((retry, freeze)).map_err(drop)
    }
}

#[must_use]
#[pin_project::pin_project]
struct RequestWaiter(#[pin] Receiver<(bool, mpsc::Sender<FreezeUntil>)>);

impl Future for RequestWaiter {
    type Output = (bool, mpsc::Sender<FreezeUntil>);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.0.poll(cx) {
            Poll::Ready(Ok(ret)) => Poll::Ready(ret),
            Poll::Ready(Err(_)) => panic!("`RequestLock` is dropped by the throttle worker"),
            Poll::Pending => Poll::Pending,
        }
    }
}

mod chan_send {
    use std::{future::Future, pin::Pin};

    use futures::task::{Context, Poll};
    use tokio::sync::{mpsc, mpsc::error::SendError};

    pub(super) trait MpscSend<T> {
        fn send1(self, val: T) -> ChanSend<T>;
    }

    #[pin_project::pin_project]
    pub(super) struct ChanSend<T>(#[pin] Inner<T>);

    #[cfg(not(feature = "nightly"))]
    type Inner<T> = Pin<Box<dyn Future<Output = Result<(), SendError<T>>> + Send>>;
    #[cfg(feature = "nightly")]
    type Inner<T> = impl Future<Output = Result<(), SendError<T>>>;

    impl<T: Send + 'static> MpscSend<T> for mpsc::Sender<T> {
        // `return`s trick IDEA not to show errors
        #[allow(clippy::needless_return)]
        fn send1(self, val: T) -> ChanSend<T> {
            #[cfg(feature = "nightly")]
            {
                fn def<T>(sender: mpsc::Sender<T>, val: T) -> Inner<T> {
                    async move { sender.send(val).await }
                }
                return ChanSend(def(self, val));
            }
            #[cfg(not(feature = "nightly"))]
            {
                let this = self;
                return ChanSend(Box::pin(async move { this.send(val).await }));
            }
        }
    }

    impl<T> Future for ChanSend<T> {
        type Output = Result<(), SendError<T>>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.project().0.poll(cx)
        }
    }
}
