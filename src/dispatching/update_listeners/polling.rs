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

/// Builder for polling update listener.
pub struct PollingBuilder<R> {
    bot: R,
    timeout: Option<Duration>,
    limit: Option<u8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
}

impl<R> PollingBuilder<R>
where
    R: Requester + Send + 'static,
    <R as Requester>::GetUpdates: Send,
{
    /// Set timeout.
    pub fn timeout(self, timeout: Duration) -> Self {
        Self { timeout: Some(timeout), ..self }
    }

    /// Set limit.
    ///
    /// ## Panics
    ///
    /// If `limit` is greater than 100.
    #[track_caller]
    pub fn limit(self, limit: u8) -> Self {
        assert!(limit <= 100, "Maximum limit is 100");

        Self { limit: Some(limit), ..self }
    }

    /// Set allowed updates.
    ///
    /// ## Note
    ///
    /// Teloxide normally (when using [`Dispatcher`] or repls) sets this
    /// automatically.
    ///
    /// [`Dispatcher`]: crate::dispatching::Dispatcher
    pub fn allowed_updates(self, allowed_updates: Vec<AllowedUpdate>) -> Self {
        Self { allowed_updates: Some(allowed_updates), ..self }
    }

    /// Deletes webhook if it was set up.
    pub async fn delete_webhook(self) -> Self {
        delete_webhook_if_setup(&self.bot).await;

        self
    }

    /// Creates a polling update listener.
    pub fn build(self) -> impl UpdateListener<R::Err> {
        let Self { bot, timeout, limit, allowed_updates } = self;
        polling(bot, timeout, limit, allowed_updates)
    }
}

/// Returns a builder for polling update listener.
pub fn polling_builder<R>(bot: R) -> PollingBuilder<R>
where
    R: Requester + Send + 'static,
    <R as Requester>::GetUpdates: Send,
{
    PollingBuilder { bot, timeout: None, limit: None, allowed_updates: None }
}

/// Returns a long polling update listener with `timeout` of 10 seconds.
///
/// See also: [`polling_builder`].
///
/// ## Notes
///
/// This function will automatically delete a webhook if it was set up.
pub async fn polling_default<R>(bot: R) -> impl UpdateListener<R::Err>
where
    R: Requester + Send + 'static,
    <R as Requester>::GetUpdates: Send,
{
    polling_builder(bot).timeout(Duration::from_secs(10)).delete_webhook().await.build()
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Returns a long polling update listener with some additional options.
///
/// - `bot`: Using this bot, the returned update listener will receive updates.
/// - `timeout`: A timeout in seconds for polling.
/// - `limit`: Limits the number of updates to be retrieved at once. Values
///   between 1—100 are accepted.
/// - `allowed_updates`: A list the types of updates you want to receive.
///
/// See [`GetUpdates`] for defaults.
///
/// See also: [`polling_default`](polling_default).
///
/// ## Notes
///
/// - `timeout` should not be bigger than http client timeout, see
///   [`default_reqwest_settings`] for default http client settings.
/// - [`repl`]s and [`Dispatcher`] use [`hint_allowed_updates`] to set
///   `allowed_updates`, so you rarely need to pass `allowed_updates`
///   explicitly.
///
/// [`default_reqwest_settings`]: teloxide::net::default_reqwest_settings
/// [`repl`]: fn@crate::repl
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`hint_allowed_updates`]:
/// crate::dispatching::update_listeners::UpdateListener::hint_allowed_updates
///
/// ## How it works
///
/// Long polling works by repeatedly calling [`Bot::get_updates`][get_updates].
/// If telegram has any updates, it returns them immediately, otherwise it waits
/// until either it has any updates or `timeout` expires.
///
/// Each [`get_updates`][get_updates] call includes an `offset` parameter equal
/// to the latest update id + one, that allows to only receive updates that has
/// not been received before.
///
/// When telegram receives a [`get_updates`][get_updates] request with `offset =
/// N` it forgets any updates with id < `N`. When `polling` listener is stopped,
/// it sends [`get_updates`][get_updates] with `timeout = 0, limit = 1` and
/// appropriate `offset`, so future bot restarts won't see updates that were
/// already seen.
///
/// Consumers of a `polling` update listener then need to repeatedly call
/// [`futures::StreamExt::next`] to get the updates.
///
/// Here is an example diagram that shows these interactions between consumers
/// like [`Dispatcher`], `polling` update listener and telegram.
///
/// ```mermaid
/// sequenceDiagram    
///     participant C as Consumer
///     participant P as polling
///     participant T as Telegram
///
///     link C: Dispatcher @ ../struct.Dispatcher.html
///     link C: repl @ ../../fn.repl.html
///     
///     C->>P: next
///
///     P->>+T: Updates? (offset = 0)
///     Note right of T: timeout
///     T->>-P: None
///     
///     P->>+T: Updates? (offset = 0)
///     Note right of T: <= timeout
///     T->>-P: updates with ids [3, 4]
///
///     P->>C: update(3)
///
///     C->>P: next
///     P->>C: update(4)
///     
///     C->>P: next
///
///     P->>+T: Updates? (offset = 5)    
///     Note right of T: <= timeout
///     T->>-P: updates with ids [5]
///
///     C->>P: stop signal
///
///     P->>C: update(5)
///
///     C->>P: next
///
///     P->>T: *Acknolegment of update(5)*
///     T->>P: ok
///
///     P->>C: None
/// ```
///
/// [get_updates]: crate::requests::Requester::get_updates
pub fn polling<R>(
    bot: R,
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
        force_stop: bool,
    }

    fn stream<B>(st: &mut State<B>) -> impl Stream<Item = Result<Update, B::Err>> + Send + '_
    where
        B: Requester + Send,
        <B as Requester>::GetUpdates: Send,
    {
        stream::unfold(st, move |state| async move {
            let State { timeout, limit, allowed_updates, bot, offset, flag, force_stop, .. } =
                &mut *state;

            if *force_stop {
                return None;
            }

            if flag.is_stopped() {
                let mut req = bot.get_updates().offset(*offset).timeout(0).limit(1);
                req.payload_mut().allowed_updates = allowed_updates.take();

                return match req.send().await {
                    Ok(_) => None,
                    Err(err) => {
                        // Prevents infinite retries, see https://github.com/teloxide/teloxide/issues/496
                        *force_stop = true;

                        Some((Either::Left(stream::once(ready(Err(err)))), state))
                    }
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
        bot,
        timeout: timeout.map(|t| t.as_secs().try_into().expect("timeout is too big")),
        limit,
        allowed_updates,
        offset: 0,
        flag,
        token,
        force_stop: false,
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
    assert_send(&polling.stop_token());

    fn assert_send(_: &impl Send) {}
}
