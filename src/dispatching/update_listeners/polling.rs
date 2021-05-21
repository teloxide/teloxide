use std::{convert::TryInto, time::Duration};

use futures::{
    future::{ready, Either},
    stream::{self, Stream, StreamExt},
};

use crate::{
    dispatching::{
        stop_token::{AsyncStopFlag, AsyncStopToken},
        update_listeners::{stateful_listener::StatefulListener, UpdateListener},
    },
    payloads::GetUpdates,
    requests::{HasPayload, Request, Requester},
    types::{AllowedUpdate, SemiparsedVec, Update},
};

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
    struct State<B: Requester> {
        bot: B,
        timeout: Option<u32>,
        limit: Option<u8>,
        allowed_updates: Option<Vec<AllowedUpdate>>,
        offset: i32,
        flag: AsyncStopFlag,
        token: AsyncStopToken,
    }

    fn stream<B>(st: &mut State<B>) -> impl Stream<Item = Result<Update, B::Err>> + '_
    where
        B: Requester,
    {
        stream::unfold(st, move |state| async move {
            let State { timeout, limit, allowed_updates, bot, offset, flag, .. } = &mut *state;

            if flag.is_stopped() {
                let mut req = bot.get_updates_fault_tolerant();

                req.payload_mut().0 = GetUpdates {
                    offset: Some(*offset),
                    timeout: Some(0),
                    limit: Some(1),
                    allowed_updates: allowed_updates.take(),
                };

                return match req.send().await {
                    Ok(_) => None,
                    Err(err) => Some((Either::Left(stream::once(ready(Err(err)))), state)),
                };
            }

            let mut req = bot.get_updates_fault_tolerant();
            req.payload_mut().0 = GetUpdates {
                offset: Some(*offset),
                timeout: *timeout,
                limit: *limit,
                allowed_updates: allowed_updates.take(),
            };

            let updates = match req.send().await {
                Err(err) => return Some((Either::Left(stream::once(ready(Err(err)))), state)),
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

                    updates.into_iter().filter_map(Result::ok).map(Ok)
                }
            };

            Some((Either::Right(stream::iter(updates)), state))
        })
        .flatten()
    }

    let (token, flag) = AsyncStopToken::new_pair();

    let state = State {
        bot: requester,
        timeout: timeout.map(|t| t.as_secs().try_into().expect("timeout is too big")),
        limit,
        allowed_updates,
        offset: 0,
        flag,
        token,
    };

    let stop = |st: &mut State<_>| st.token.clone();

    let timeout_hint = Some(move |_: &State<_>| timeout);

    StatefulListener { state, stream, stop_token: stop, timeout_hint }
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
