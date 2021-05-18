//! Receiving updates from Telegram.
//!
//! The key trait here is [`UpdateListener`]. You can get it by these functions:
//!
//!  - [`polling_default`], which returns a default long polling listener.
//!  - [`polling`], which returns a long/short polling listener with your
//!    configuration.
//!
//! And then you can extract updates from it and pass them directly to a
//! dispatcher.
//!
//! Telegram supports two ways of [getting updates]: [long]/[short] polling and
//! [webhook].
//!
//! # Long Polling
//!
//! In long polling, you just call [`Box::get_updates`] every N seconds.
//!
//! ## Example
//!
//! <pre>
//!     tg                           bot
//!      |                            |
//!      |<---------------------------| Updates? (Bot::get_updates call)
//!      ↑                            ↑
//!      |          timeout<a id="1b" href="#1">^1</a>         |
//!      ↓                            ↓
//! Nope |--------------------------->|
//!      ↑                            ↑
//!      | delay between Bot::get_updates<a id="2b" href="#2">^2</a> |
//!      ↓                            ↓
//!      |<---------------------------| Updates?
//!      ↑                            ↑
//!      |          timeout<a id="3b" href="#3">^3</a>         |
//!      ↓                            ↓
//! Yes  |-------[updates 0, 1]------>|
//!      ↑                            ↑
//!      |           delay            |
//!      ↓                            ↓
//!      |<-------[offset = 1]--------| Updates?<a id="4b" href="#4">^4</a>
//!      ↑                            ↑
//!      |           timeout          |
//!      ↓                            ↓
//! Yes  |---------[update 2]-------->|
//!      ↑                            ↑
//!      |           delay            |
//!      ↓                            ↓
//!      |<-------[offset = 2]--------| Updates?
//!      ↑                            ↑
//!      |           timeout          |
//!      ↓                            ↓
//! Nope |--------------------------->|
//!      ↑                            ↑
//!      |           delay            |
//!      ↓                            ↓
//!      |<-------[offset = 2]--------| Updates?
//!      ↑                            ↑
//!      |           timeout          |
//!      ↓                            ↓
//! Nope |--------------------------->|
//!      ↑                            ↑
//!      |           delay            |
//!      ↓                            ↓
//!      |<-------[offset = 2]--------| Updates?
//!      ↑                            ↑
//!      |           timeout          |
//!      ↓                            ↓
//! Yes  |-------[updates 2..5]------>|
//!      ↑                            ↑
//!      |           delay            |
//!      ↓                            ↓
//!      |<-------[offset = 5]--------| Updates?
//!      ↑                            ↑
//!      |           timeout          |
//!      ↓                            ↓
//! Nope |--------------------------->|
//!      |                            |
//!      ~    and so on, and so on    ~
//! </pre>
//!
//! <a id="1" href="#1b">^1</a> A timeout can be even 0
//!   (this is also called short polling),
//!   but you should use it **only** for testing purposes.
//!
//! <a id="2" href="#2b">^2</a> Large delays will cause in bot lags,
//!   so delay shouldn't exceed second.
//!
//! <a id="3" href="#3b">^3</a> Note that if Telegram already have updates for
//!   you it will answer you **without** waiting for a timeout.
//!
//! <a id="4" href="#4b">^4</a> `offset = N` means that we've already received
//!   updates `0..=N`.
//!
//! # Webhooks
//! See the [README FAQ about webhooks](https://github.com/teloxide/teloxide/blob/master/README.md#faq).
//!
//! [`UpdateListener`]: UpdateListener
//! [`polling_default`]: polling_default
//! [`polling`]: polling
//! [`Box::get_updates`]: crate::requests::Requester::get_updates
//! [getting updates]: https://core.telegram.org/bots/api#getting-updates
//! [long]: https://en.wikipedia.org/wiki/Push_technology#Long_polling
//! [short]: https://en.wikipedia.org/wiki/Polling_(computer_science)
//! [webhook]: https://en.wikipedia.org/wiki/Webhook

use futures::{stream, Stream, StreamExt};

use std::{convert::TryInto, time::Duration};
use teloxide_core::{
    requests::{HasPayload, Request, Requester},
    types::{AllowedUpdate, SemiparsedVec, Update},
};

/// An update listener.
///
/// Implementors of this trait allow getting updates from Telegram.
///
/// Currently Telegram has 2 ways of getting updates -- [polling] and
/// [webhooks]. Currently, only the former one is implemented (see [`polling`]
/// and [`polling_default`])
///
/// Some functions of this trait are located in the supertrait
/// ([`AsUpdateStream`]), see also:
/// - [`Stream`]
/// - [`as_stream`]
///
/// [polling]: self#long-polling
/// [webhooks]: self#webhooks
/// [`Stream`]: AsUpdateStream::Stream
/// [`as_stream`]: AsUpdateStream::as_stream
pub trait UpdateListener<E>: for<'a> AsUpdateStream<'a, E> {
    /// Stop listening for updates.
    ///  
    /// This function is not guaranteed to have an immidiate effect. That is
    /// some listners can return updates even after [`stop`] is called (e.g.:
    /// because of buffering).
    ///
    /// [`stop`]: UpdateListener::stop
    ///
    /// Implementors of this function are encouraged to stop listening for
    /// updates as soon as possible and return `None` from the update stream as
    /// soon as all cached updates are returned.
    fn stop(&mut self);
}

/// [`UpdateListener`]'s supertrait/extension.
///
/// This trait is a workaround to not require GAT.
pub trait AsUpdateStream<'a, E> {
    /// Stream of updates from Telegram.
    type Stream: Stream<Item = Result<Update, E>> + 'a;

    /// Creates the update [`Stream`].
    ///
    /// [`Stream`]: AsUpdateStream::Stream
    fn as_stream(&'a mut self) -> Self::Stream;
}

/// Returns a long polling update listener with `timeout` of 10 seconds.
///
/// See also: [`polling`](polling).
///
/// ## Notes
///
/// This function will automatically delete a webhook if it was set up.
pub async fn polling_default<R>(requester: R) -> impl UpdateListener<R::Err>
where
    R: Requester + 'static,
    <R as Requester>::GetUpdatesFaultTolerant: Send,
{
    delete_webhook_if_setup(&requester).await;
    polling(requester, Some(Duration::from_secs(10)), None, None)
}

/// Returns a long/short polling update listener with some additional options.
///
/// - `bot`: Using this bot, the returned update listener will receive updates.
/// - `timeout`: A timeout for polling.
/// - `limit`: Limits the number of updates to be retrieved at once. Values
///   between 1—100 are accepted.
/// - `allowed_updates`: A list the types of updates you want to receive.
/// See [`GetUpdates`] for defaults.
///
/// See also: [`polling_default`](polling_default).
///
/// [`GetUpdates`]: crate::payloads::GetUpdates
pub fn polling<R>(
    requester: R,
    timeout: Option<Duration>,
    limit: Option<u8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
) -> impl UpdateListener<R::Err>
where
    R: Requester + 'static,
    <R as Requester>::GetUpdatesFaultTolerant: Send,
{
    enum RunningState {
        Polling,
        Stopping,
        Stopped,
    }

    struct State<B> {
        bot: B,
        timeout: Option<u32>,
        limit: Option<u8>,
        allowed_updates: Option<Vec<AllowedUpdate>>,
        offset: i32,
        run_state: RunningState,
    }

    fn stream<B>(st: &mut State<B>) -> impl Stream<Item = Result<Update, B::Err>> + '_
    where
        B: Requester,
    {
        stream::unfold(st, move |state| async move {
            let State { timeout, limit, allowed_updates, bot, offset, run_state, .. } = &mut *state;

            match run_state {
                RunningState::Polling => {}
                RunningState::Stopped => return None,
                RunningState::Stopping => {
                    let mut req = bot.get_updates_fault_tolerant();

                    let payload = &mut req.payload_mut().0;
                    payload.offset = Some(*offset);
                    payload.timeout = *timeout;
                    payload.limit = Some(1);
                    payload.allowed_updates = allowed_updates.take();

                    return match req.send().await {
                        Ok(_) => {
                            *run_state = RunningState::Stopped;
                            None
                        }
                        Err(err) => Some((stream::iter(vec![Err(err)]), state)),
                    };
                }
            }

            let mut req = bot.get_updates_fault_tolerant();
            let payload = &mut req.payload_mut().0;
            payload.offset = Some(*offset);
            payload.timeout = *timeout;
            payload.limit = *limit;
            payload.allowed_updates = allowed_updates.take();

            let updates = match req.send().await {
                Err(err) => vec![Err(err)],
                Ok(SemiparsedVec(updates)) => {
                    // Set offset to the last update's id + 1
                    if let Some(upd) = updates.last() {
                        let id: i32 = match upd {
                            Ok(ok) => ok.id,
                            Err((value, _)) => value["update_id"]
                                .as_i64()
                                .expect("The 'update_id' field must always exist in Update")
                                .try_into()
                                .expect("update_id must be i32"),
                        };

                        *offset = id + 1;
                    }

                    for update in &updates {
                        if let Err((value, e)) = update {
                            log::error!(
                                "Cannot parse an update.\nError: {:?}\nValue: {}\n\
                            This is a bug in teloxide-core, please open an issue here: \
                            https://github.com/teloxide/teloxide-core/issues.",
                                e,
                                value
                            );
                        }
                    }

                    updates.into_iter().filter_map(Result::ok).map(Ok).collect::<Vec<_>>()
                }
            };

            Some((stream::iter(updates), state))
        })
        .flatten()
    }

    let timeout = timeout.map(|t| t.as_secs().try_into().expect("timeout is too big"));

    let state = State {
        bot: requester,
        timeout,
        limit,
        allowed_updates,
        offset: 0,
        run_state: RunningState::Polling,
    };

    let stop = assert_stop_fn(|st: &mut State<_>| {
        st.run_state = RunningState::Stopping;
    });

    StatefulListner { state, stream, stop }
}

async fn delete_webhook_if_setup<R>(requester: &R)
where
    R: Requester,
{
    let webhook_info = match requester.get_webhook_info().send().await {
        Ok(ok) => ok,
        Err(e) => {
            log::error!("Failed to get webhook info: {:?}", e);
            return;
        }
    };

    let is_webhook_setup = !webhook_info.url.is_empty();

    if is_webhook_setup {
        if let Err(e) = requester.delete_webhook().send().await {
            log::error!("Failed to delete a webhook: {:?}", e);
        }
    }
}

/// A listner created from `state` and `stream`/`stop` functions.
struct StatefulListner<St, Sf, F> {
    /// The state of the listner.
    state: St,

    /// Function used as `AsUpdateStream::as_stream`.
    ///
    /// Must be of type `for<'a> &'a mut St -> impl Stream + 'a` and callable by
    /// `&mut`.
    stream: Sf,

    /// Function used as `UpdateListner::stop`.
    ///
    /// Must be of type `for<'a> &'a mut St`.
    stop: Option<F>,
}

impl<'a, St, Sf, F, Strm, E> AsUpdateStream<'a, E> for StatefulListner<St, Sf, F>
where
    (St, Strm): 'a,
    Sf: FnMut(&'a mut St) -> Strm,
    Strm: Stream<Item = Result<Update, E>>,
{
    type Stream = Strm;

    fn as_stream(&'a mut self) -> Self::Stream {
        (self.stream)(&mut self.state)
    }
}

impl<St, Sf, F, E> UpdateListener<E> for StatefulListner<St, Sf, F>
where
    Self: for<'a> AsUpdateStream<'a, E>,
    F: FnOnce(&mut St),
{
    fn stop(&mut self) {
        self.stop.take().map(|stop| stop(&mut self.state));
    }
}

/// Assert (at compile tume) that `f` is fine as a stop-function (closure
/// lifetime inference workaround).
fn assert_stop_fn<F, St>(f: F) -> Option<F>
where
    F: FnOnce(&mut St),
{
    Some(f)
}
