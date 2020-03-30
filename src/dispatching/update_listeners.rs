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
//! [`UpdateListener`]: UpdateListener
//! [`polling_default`]: polling_default
//! [`polling`]: polling
//! [`Box::get_updates`]: crate::Bot::get_updates
//! [getting updates]: https://core.telegram.org/bots/api#getting-updates
//! [long]: https://en.wikipedia.org/wiki/Push_technology#Long_polling
//! [short]: https://en.wikipedia.org/wiki/Polling_(computer_science)
//! [webhook]: https://en.wikipedia.org/wiki/Webhook

use futures::{stream, Stream, StreamExt};
use tokio::sync::mpsc;

use warp::Filter;

use crate::{
    bot::Bot,
    prelude::ResponseResult,
    requests::Request,
    types::{AllowedUpdate, Update},
    RequestError,
};
use either::Either;
use reqwest::Url;
use std::{
    convert::{Infallible, TryInto},
    net::SocketAddr,
    path::Path,
    sync::Arc,
    time::Duration,
};

/// A generic update listener.
pub trait UpdateListener<E>: Stream<Item = Result<Update, E>> {
    // TODO: add some methods here (.shutdown(), etc).
}
impl<S, E> UpdateListener<E> for S where S: Stream<Item = Result<Update, E>> {}

/// Returns a long polling update listener with `timeout` of 1 minute.
///
/// See also: [`polling`](polling).
pub fn polling_default(bot: Arc<Bot>) -> impl UpdateListener<RequestError> {
    polling(bot, Some(Duration::from_secs(60)), None, None)
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
/// [`GetUpdates`]: crate::requests::GetUpdates
pub fn polling(
    bot: Arc<Bot>,
    timeout: Option<Duration>,
    limit: Option<u8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
) -> impl UpdateListener<RequestError> {
    let timeout =
        timeout.map(|t| t.as_secs().try_into().expect("timeout is too big"));

    stream::unfold(
        (allowed_updates, bot, 0),
        move |(mut allowed_updates, bot, mut offset)| async move {
            let mut req = bot.get_updates().offset(offset);
            req.timeout = timeout;
            req.limit = limit;
            req.allowed_updates = allowed_updates.take();

            let updates = match req.send().await {
                Err(err) => vec![Err(err)],
                Ok(updates) => {
                    // Set offset to the last update's id + 1
                    if let Some(upd) = updates.last() {
                        let id: i32 = match upd {
                            Ok(ok) => ok.id,
                            Err((value, _)) => value["update_id"]
                                .as_i64()
                                .expect(
                                    "The 'update_id' field must always exist in \
                                     Update",
                                )
                                .try_into()
                                .expect("update_id must be i32"),
                        };

                        offset = id + 1;
                    }

                    let updates = updates
                        .into_iter()
                        .filter(|update| match update {
                            Err((value, error)) => {
                                log::error!("Cannot parse an update.\nError: {:?}\nValue: {}\n\
                        This is a bug in teloxide, please open an issue here: \
                        https://github.com/teloxide/teloxide/issues.", error, value);
                                false
                            }
                            Ok(_) => true,
                        })
                        .map(|update| {
                            update.expect("See the previous .filter() call")
                        })
                        .collect::<Vec<Update>>();

                    updates.into_iter().map(Ok).collect::<Vec<_>>()
                }
            };

            Some((stream::iter(updates), (allowed_updates, bot, offset)))
        },
    )
    .flatten()
}

pub async fn webhook<KP, KB, CP, CB>(
    url: Url,
    addr: SocketAddr,
    key: Either<KP, KB>,
    cert: Either<CP, CB>,
) -> ResponseResult<impl UpdateListener<Infallible>>
where
    KP: AsRef<Path>,
    KB: AsRef<[u8]>,
    CP: AsRef<Path>,
    CB: AsRef<[u8]>,
{
    let (tx, rx) = mpsc::unbounded_channel();

    let server = warp::post().and(warp::path(url)).and(warp::body::json()).map(
        move |update: Update| {
            tx.send(Ok(update)).expect("Cannot send an update from webhook");
            ""
        },
    );

    let serve = warp::serve(server).tls();

    let serve = match key {
        Either::Left(path) => serve.key_path(path),
        Either::Right(bytes) => serve.key(bytes),
    };

    let serve = match cert {
        Either::Left(path) => serve.cert_path(path),
        Either::Right(bytes) => serve.cert(bytes),
    };

    tokio::spawn(serve.run(addr));

    Ok(rx)
}
