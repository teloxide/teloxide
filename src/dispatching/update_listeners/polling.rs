use std::{
    convert::TryInto,
    future::Future,
    pin::Pin,
    task::{
        self,
        Poll::{self, Ready},
    },
    time::Duration,
    vec,
};

use futures::{ready, stream::Stream};

use crate::{
    dispatching::{
        stop_token::{AsyncStopFlag, AsyncStopToken},
        update_listeners::{AsUpdateStream, UpdateListener},
    },
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
    /// A timeout in seconds for polling.
    ///
    /// ## Note
    ///
    /// `timeout` should not be bigger than http client timeout, see
    /// [`default_reqwest_settings`] for default http client settings.
    ///
    /// [`default_reqwest_settings`]: crate::net::default_reqwest_settings
    pub fn timeout(self, timeout: Duration) -> Self {
        Self { timeout: Some(timeout), ..self }
    }

    /// Limit the number of updates to be retrieved at once. Values between
    /// 1—100 are accepted.
    ///
    /// ## Panics
    ///
    /// If `limit` is greater than 100.
    #[track_caller]
    pub fn limit(self, limit: u8) -> Self {
        assert!(limit <= 100, "Maximum limit is 100");

        Self { limit: Some(limit), ..self }
    }

    /// A list of the types of updates you want to receive.
    ///
    /// ## Note
    ///
    /// Teloxide normally (when using [`Dispatcher`] or [`repl`]s) sets this
    /// automatically via [`hint_allowed_updates`], so you rarely need to use
    /// `allowed_updates` explicitly.
    ///
    /// [`Dispatcher`]: crate::dispatching::Dispatcher
    /// [`repl`]: fn@crate::repl
    /// [`hint_allowed_updates`]: crate::dispatching::update_listeners::UpdateListener::hint_allowed_updates
    pub fn allowed_updates(self, allowed_updates: Vec<AllowedUpdate>) -> Self {
        Self { allowed_updates: Some(allowed_updates), ..self }
    }

    /// Deletes webhook if it was set up.
    pub async fn delete_webhook(self) -> Self {
        delete_webhook_if_setup(&self.bot).await;

        self
    }

    /// Returns a long polling update listener with configuration from the
    /// builder.
    ///
    /// See also: [`polling_default`], [`Polling`].
    pub fn build(self) -> Polling<R> {
        let Self { bot, timeout, limit, allowed_updates } = self;
        let (token, flag) = AsyncStopToken::new_pair();
        Polling { bot, timeout, limit, allowed_updates, flag, token }
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
pub async fn polling_default<R>(bot: R) -> Polling<R>
where
    R: Requester + Send + 'static,
    <R as Requester>::GetUpdates: Send,
{
    polling_builder(bot).timeout(Duration::from_secs(10)).delete_webhook().await.build()
}

/// Returns a long polling update listener with some additional options.
#[deprecated(since = "0.7.0", note = "use `polling_builder` instead")]
pub fn polling<R>(
    bot: R,
    timeout: Option<Duration>,
    limit: Option<u8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
) -> Polling<R>
where
    R: Requester + Send + 'static,
    <R as Requester>::GetUpdates: Send,
{
    let mut builder = polling_builder(bot);
    builder.timeout = timeout;
    builder.limit = limit;
    builder.allowed_updates = allowed_updates;
    builder.build()
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

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A polling update listener.
///
/// ## How it works
///
/// Long polling works by repeatedly calling
/// [`Bot::get_updates`][get_updates]. If telegram has any updates, it
/// returns them immediately, otherwise it waits until either it has any
/// updates or `timeout` expires.
///
/// Each [`get_updates`][get_updates] call includes an `offset` parameter
/// equal to the latest update id + one, that allows to only receive
/// updates that has not been received before.
///
/// When telegram receives a [`get_updates`][get_updates] request with
/// `offset = N` it forgets any updates with id < `N`. When `polling`
/// listener is stopped, it sends [`get_updates`][get_updates] with
/// `timeout = 0, limit = 1` and appropriate `offset`, so future bot
/// restarts won't see updates that were already seen.
///
/// Consumers of a [`Polling`] update listener then need to repeatedly call
/// [`futures::StreamExt::next`] to get the updates.
///
/// Here is an example diagram that shows these interactions between
/// consumers like [`Dispatcher`], [`Polling`] update listener and
/// telegram.
///
/// ```mermaid
/// sequenceDiagram    
///     participant C as Consumer
///     participant P as Polling
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
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub struct Polling<B: Requester> {
    bot: B,
    timeout: Option<Duration>,
    limit: Option<u8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
    flag: AsyncStopFlag,
    token: AsyncStopToken,
}

#[pin_project::pin_project]
pub struct PollingStream<'a, B: Requester> {
    /// Parent structure
    polling: &'a mut Polling<B>,

    /// Timeout parameter for normal `get_updates()` calls.
    timeout: Option<u32>,
    /// Allowed updates parameter for the first `get_updates()` call.
    allowed_updates: Option<Vec<AllowedUpdate>>,
    /// Offset parameter  for normal `get_updates()` calls.
    offset: i32,

    /// If this is set, return `None` from `poll_next` immediately.
    force_stop: bool,
    /// If true we've sent last `get_updates()` call for graceful shutdown.
    stopping: bool,

    /// Buffer of updates to be yielded.
    buffer: vec::IntoIter<Update>,

    /// In-flight `get_updates()` call.
    #[pin]
    in_flight: Option<<B::GetUpdates as Request>::Send>,
}

impl<B: Requester + Send + 'static> UpdateListener<B::Err> for Polling<B> {
    type StopToken = AsyncStopToken;

    fn stop_token(&mut self) -> Self::StopToken {
        self.token.clone()
    }

    fn hint_allowed_updates(&mut self, hint: &mut dyn Iterator<Item = AllowedUpdate>) {
        // TODO: we should probably warn if there already were different allowed updates
        // before
        self.allowed_updates = Some(hint.collect());
    }

    fn timeout_hint(&self) -> Option<Duration> {
        self.timeout
    }
}

impl<'a, B: Requester + Send + 'a> AsUpdateStream<'a, B::Err> for Polling<B> {
    type Stream = PollingStream<'a, B>;

    fn as_stream(&'a mut self) -> Self::Stream {
        let timeout = self.timeout.map(|t| t.as_secs().try_into().expect("timeout is too big"));
        let allowed_updates = self.allowed_updates.clone();
        PollingStream {
            polling: self,
            timeout,
            allowed_updates,
            offset: 0,
            force_stop: false,
            stopping: false,
            buffer: Vec::new().into_iter(),
            in_flight: None,
        }
    }
}

impl<B: Requester> Stream for PollingStream<'_, B> {
    type Item = Result<Update, B::Err>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.as_mut().project();

        if *this.force_stop {
            return Ready(None);
        }

        // Poll in-flight future until completion
        if let Some(in_flight) = this.in_flight.as_mut().as_pin_mut() {
            let res = ready!(in_flight.poll(cx));
            this.in_flight.set(None);

            match res {
                Ok(_) if *this.stopping => return Ready(None),
                Err(err) if *this.stopping => {
                    // Prevents infinite retries, see https://github.com/teloxide/teloxide/issues/496
                    *this.force_stop = true;

                    return Ready(Some(Err(err)));
                }
                Ok(updates) => {
                    if let Some(upd) = updates.last() {
                        *this.offset = upd.id + 1;
                    }

                    *this.buffer = updates.into_iter();
                }
                Err(err) => return Ready(Some(Err(err))),
            }
        }

        // If there are any buffered updates, return one
        if let Some(upd) = this.buffer.next() {
            return Ready(Some(Ok(upd)));
        }

        // When stopping we set `timeout = 0` and `limit = 1` so that `get_updates()`
        // set last seen update (offset) and return immediately
        let (timeout, limit) = if this.polling.flag.is_stopped() {
            *this.stopping = true;
            (Some(0), Some(1))
        } else {
            (*this.timeout, this.polling.limit)
        };

        let req = this
            .polling
            .bot
            .get_updates()
            .with_payload_mut(|pay| {
                pay.offset = Some(*this.offset);
                pay.timeout = timeout;
                pay.limit = limit;
                pay.allowed_updates = this.allowed_updates.take();
            })
            .send();
        this.in_flight.set(Some(req));

        // Recurse to poll `self.in_flight`
        self.poll_next(cx)
    }
}

#[test]
fn polling_is_send() {
    use crate::dispatching::update_listeners::AsUpdateStream;

    let bot = crate::Bot::new("TOKEN");
    #[allow(deprecated)]
    let mut polling = polling(bot, None, None, None);

    assert_send(&polling);
    assert_send(&polling.as_stream());
    assert_send(&polling.stop_token());

    fn assert_send(_: &impl Send) {}
}
