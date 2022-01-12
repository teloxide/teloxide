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
    payloads::{GetUpdates, GetUpdatesSetters as _},
    requests::{HasPayload, Request, Requester},
    types::{AllowedUpdate, Update},
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
    R: Requester + Send + 'static,
    <R as Requester>::GetUpdates: Send,
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
    R: Requester + Send + 'static,
    <R as Requester>::GetUpdates: Send,
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

    fn stream<B>(st: &mut State<B>) -> impl Stream<Item = Result<Update, B::Err>> + Send + '_
    where
        B: Requester + Send,
        <B as Requester>::GetUpdates: Send,
    {
        stream::unfold(st, move |state| async move {
            let State { timeout, limit, allowed_updates, bot, offset, flag, .. } = &mut *state;

            if flag.is_stopped() {
                let mut req = bot.get_updates().offset(*offset).timeout(0).limit(1);
                req.payload_mut().allowed_updates = allowed_updates.take();

                return match req.send().await {
                    Ok(_) => None,
                    Err(err) => Some((Either::Left(stream::once(ready(Err(err)))), state)),
                };
            }

            let mut req = bot.get_updates();
            *req.payload_mut() = GetUpdates {
                offset: Some(*offset),
                timeout: *timeout,
                limit: *limit,
                allowed_updates: allowed_updates.take(),
            };

            match req.send().await {
                Ok(updates) => {
                    // Set offset to the last update's id + 1
                    if let Some(upd) = updates.last() {
                        *offset = upd.id + 1;
                    }

                    let updates = updates.into_iter().map(Ok);
                    Some((Either::Right(stream::iter(updates)), state))
                }
                Err(err) => Some((Either::Left(stream::once(ready(Err(err)))), state)),
            }
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

    let stop_token = |st: &mut State<_>| st.token.clone();

    let hint_allowed_updates =
        Some(|state: &mut State<_>, allowed: &mut dyn Iterator<Item = AllowedUpdate>| {
            // TODO: we should probably warn if there already were different allowed updates
            // before
            state.allowed_updates = Some(allowed.collect());
        });
    let timeout_hint = Some(move |_: &State<_>| timeout);

    StatefulListener::new_with_hints(state, stream, stop_token, hint_allowed_updates, timeout_hint)
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

    let is_webhook_setup = webhook_info.url.is_some();

    if is_webhook_setup {
        if let Err(e) = requester.delete_webhook().send().await {
            log::error!("Failed to delete a webhook: {:?}", e);
        }
    }
}

#[test]
fn polling_is_send() {
    use crate::dispatching::update_listeners::AsUpdateStream;

    let bot = crate::Bot::new("TOKEN");
    let mut polling = polling(bot, None, None, None);

    assert_send(&polling);
    assert_send(&polling.as_stream());

    fn assert_send(_: &impl Send) {}
}
