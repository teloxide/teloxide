//! Receiving updates from Telegram.
//!
//! The key trait here is [`Updater`]. You can get it by these functions:
//!
//!  - [`polling_basic`], which returns a default long polling updater.
//!  - [`polling_advanced`], which returns a long/short polling updater with
//!    your configuration.
//!
//! And then you can pass it directly to a dispatcher.
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
//! [`Updater`]: Updater
//! [`polling_basic`]: polling_basic
//! [`polling_advanced`]: polling_advanced
//! [`Dispatcher`]: crate::dispatching::Dispatcher::dispatch
//! [`Box::get_updates`]: crate::Bot::get_updates
//! [getting updates]: https://core.telegram.org/bots/api#getting-updates
//! [long]: https://en.wikipedia.org/wiki/Push_technology#Long_polling
//! [short]: https://en.wikipedia.org/wiki/Polling_(computer_science)
//! [webhook]: https://en.wikipedia.org/wiki/Webhook

use futures::{stream, Stream, StreamExt};

use crate::{
    bot::Bot, requests::payloads::AllowedUpdate, types::Update, RequestError,
};
use std::{convert::TryInto, time::Duration};

/// A generic updater.
pub trait Updater<E>: Stream<Item = Result<Update, E>> {
    // TODO: add some methods here (.shutdown(), etc).
}
impl<S, E> Updater<E> for S where S: Stream<Item = Result<Update, E>> {}

/// Returns a long polling updater with the default configuration.
///
/// It is the same as calling [`polling_advanced`] with `timeout` of 30 seconds,
/// `limit=100` and receive all kinds of updates.
///
/// [`polling_advanced`]: polling_advanced
pub fn polling_basic(bot: &Bot) -> impl Updater<RequestError> + '_ {
    polling_advanced::<&[_]>(bot, Duration::from_secs(30), 100, &[])
}

/// Returns a long/short polling updater with some additional options.
///
/// - `bot`: Using this bot, the returned updater will receive updates.
/// - `timeout`: A timeout for polling.
/// - `limit`: Limits the number of updates to be retrieved at once. Values
///   between 1—100 are accepted.
/// - `allowed_updates`: A list the types of updates you want to receive.
pub fn polling_advanced<'a, A>(
    bot: &'a Bot,
    timeout: Duration,
    limit: u8,
    allowed_updates: A,
) -> impl Updater<RequestError> + 'a
where
    A: Into<&'a [AllowedUpdate]>,
{
    let mut allowed_updates = Some(allowed_updates.into());

    stream::unfold((bot, 0), move |(bot, mut offset)| async move {
        let updates = bot
            .get_updates()
            .offset(offset)
            .timeout(timeout.as_secs().try_into().expect("timeout is too big"))
            .limit(limit)
            .allowed_updates(allowed_updates.take().unwrap_or(&[]))
            .send()
            .await
            .map_or_else(
                |err| vec![Err(err)],
                |updates| {
                    if let Some(upd) = updates.last() {
                        offset = upd.id + 1;
                    }
                    updates.into_iter().map(Ok).collect::<Vec<_>>()
                },
            );

        Some((stream::iter(updates), (bot, offset)))
    })
    .flatten()
}

// TODO implement webhook (this actually require webserver and probably we
//   should add cargo feature that adds webhook)
//pub fn webhook<'a>(bot: &'a  cfg: WebhookConfig) -> Updater<impl
// Stream<Item=Result<Update, ???>> + 'a> {}
