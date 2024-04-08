use std::{
    future::Future,
    mem,
    pin::Pin,
    task::{
        self,
        Poll::{self, Ready},
    },
    time::Duration,
    vec,
};

use futures::{ready, stream::Stream};
use tokio::time::{sleep, Sleep};

use crate::{
    backoff::{exponential_backoff_strategy, BackoffStrategy},
    requests::{HasPayload, Request, Requester},
    stop::{mk_stop_token, StopFlag, StopToken},
    types::{AllowedUpdate, Update},
    update_listeners::{assert_update_listener, AsUpdateStream, UpdateListener},
};

/// Builder for polling update listener.
///
/// Can be created by [`Polling::builder`].
#[non_exhaustive]
#[must_use = "`PollingBuilder` is a builder and does nothing unless used"]
pub struct PollingBuilder<R> {
    pub bot: R,
    pub timeout: Option<Duration>,
    pub limit: Option<u8>,
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
    pub drop_pending_updates: bool,
    pub backoff_strategy: BackoffStrategy,
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
    /// If `limit` is 0 or greater than 100.
    #[track_caller]
    pub fn limit(self, limit: u8) -> Self {
        assert_ne!(limit, 0, "limit can't be 0");
        assert!(limit <= 100, "maximum limit is 100, can't set limit to `{limit}`");

        Self { limit: Some(limit), ..self }
    }

    /// A list of the types of updates you want to receive.
    ///
    /// ## Note
    ///
    /// `teloxide` normally (when using [`Dispatcher`] or [`repl`]s) sets this
    /// automatically via [`hint_allowed_updates`], so you rarely need to use
    /// `allowed_updates` explicitly.
    ///
    /// [`Dispatcher`]: crate::dispatching::Dispatcher
    /// [`repl`]: fn@crate::repl
    /// [`hint_allowed_updates`]: crate::update_listeners::UpdateListener::hint_allowed_updates
    pub fn allowed_updates(self, allowed_updates: Vec<AllowedUpdate>) -> Self {
        Self { allowed_updates: Some(allowed_updates), ..self }
    }

    /// Drops pending updates.
    pub fn drop_pending_updates(self) -> Self {
        Self { drop_pending_updates: true, ..self }
    }

    /// The backoff strategy that will be used for delay calculation between
    /// reconnections caused by network errors.
    ///
    /// By default, the [`exponential_backoff_strategy`] is used.
    pub fn backoff_strategy(
        self,
        backoff_strategy: impl 'static + Send + Fn(u32) -> Duration,
    ) -> Self {
        Self { backoff_strategy: Box::new(backoff_strategy), ..self }
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
        let Self { bot, timeout, limit, allowed_updates, drop_pending_updates, backoff_strategy } =
            self;
        let (token, flag) = mk_stop_token();
        let polling = Polling {
            bot,
            timeout,
            limit,
            allowed_updates,
            drop_pending_updates,
            flag: Some(flag),
            token,
            stop_token_cloned: false,
            backoff_strategy,
        };

        assert_update_listener(polling)
    }
}

/// Returns a long polling update listener with `timeout` of 10 seconds.
///
/// See also: [`Polling::builder`].
///
/// ## Notes
///
/// This function will automatically delete a webhook if it was set up.
pub async fn polling_default<R>(bot: R) -> Polling<R>
where
    R: Requester + Send + 'static,
    <R as Requester>::GetUpdates: Send,
{
    let polling =
        Polling::builder(bot).timeout(Duration::from_secs(10)).delete_webhook().await.build();

    assert_update_listener(polling)
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
///     P->>T: *Acknowledgement of update(5)*
///     T->>P: ok
///
///     P->>C: None
/// ```
///
/// [get_updates]: crate::requests::Requester::get_updates
/// [`Dispatcher`]: crate::dispatching::Dispatcher
#[must_use = "`Polling` is an update listener and does nothing unless used"]
pub struct Polling<B: Requester> {
    bot: B,
    timeout: Option<Duration>,
    limit: Option<u8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
    drop_pending_updates: bool,
    flag: Option<StopFlag>,
    token: StopToken,
    stop_token_cloned: bool,
    backoff_strategy: BackoffStrategy,
}

impl<R> Polling<R>
where
    R: Requester,
{
    /// Returns a builder for polling update listener.
    pub fn builder(bot: R) -> PollingBuilder<R>
    where
        R: Send + 'static,
        <R as Requester>::GetUpdates: Send,
    {
        PollingBuilder {
            bot,
            timeout: None,
            limit: None,
            allowed_updates: None,
            drop_pending_updates: false,
            backoff_strategy: Box::new(exponential_backoff_strategy),
        }
    }

    /// Returns true if re-initialization happened *and*
    /// the previous token was cloned.
    fn reinit_stop_flag_if_needed(&mut self) -> bool {
        if self.flag.is_some() {
            return false;
        }

        let (token, flag) = mk_stop_token();
        self.token = token;
        self.flag = Some(flag);
        mem::replace(&mut self.stop_token_cloned, false)
    }
}

#[pin_project::pin_project]
pub struct PollingStream<'a, B: Requester> {
    /// Parent structure
    polling: &'a mut Polling<B>,

    /// Whatever to drop pending updates or not.
    drop_pending_updates: bool,

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

    /// The flag that notifies polling to stop polling.
    #[pin]
    flag: StopFlag,

    /// How long it takes to make next reconnection attempt
    #[pin]
    eepy: Option<Sleep>,

    /// Counter for network errors occured during the current series of
    /// reconnections
    error_count: u32,
}

impl<B: Requester + Send + 'static> UpdateListener for Polling<B> {
    type Err = B::Err;

    fn stop_token(&mut self) -> StopToken {
        self.reinit_stop_flag_if_needed();
        self.stop_token_cloned = true;
        self.token.clone()
    }

    fn hint_allowed_updates(&mut self, hint: &mut dyn Iterator<Item = AllowedUpdate>) {
        // TODO: we should probably warn if there already were different allowed updates
        // before
        self.allowed_updates = Some(hint.collect());
    }
}

impl<'a, B: Requester + Send + 'a> AsUpdateStream<'a> for Polling<B> {
    type StreamErr = B::Err;
    type Stream = PollingStream<'a, B>;

    fn as_stream(&'a mut self) -> Self::Stream {
        let timeout = self.timeout.map(|t| t.as_secs().try_into().expect("timeout is too big"));
        let allowed_updates = self.allowed_updates.clone();
        let drop_pending_updates = self.drop_pending_updates;

        let token_used_and_updated = self.reinit_stop_flag_if_needed();

        // FIXME: document that `as_stream` is a destructive operation, actually,
        //        and you need to call `stop_token` *again* after it
        if token_used_and_updated {
            panic!(
                "detected calling `as_stream` a second time after calling `stop_token`. \
                 `as_stream` updates the stop token, thus you need to call it again after calling \
                 `as_stream`"
            )
        }

        // Unwrap: just called reinit
        let flag = self.flag.take().unwrap();
        PollingStream {
            polling: self,
            drop_pending_updates,
            timeout,
            allowed_updates,
            offset: 0,
            force_stop: false,
            stopping: false,
            buffer: Vec::new().into_iter(),
            in_flight: None,
            flag,
            eepy: None,
            error_count: 0,
        }
    }
}

impl<B: Requester> Stream for PollingStream<'_, B> {
    type Item = Result<Update, B::Err>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        log::trace!("polling polling stream");
        let mut this = self.as_mut().project();

        if *this.force_stop {
            return Ready(None);
        }

        // If there are any buffered updates, return one
        if let Some(upd) = this.buffer.next() {
            return Ready(Some(Ok(upd)));
        }

        // Check if we should stop and if so — drop in flight request,
        // we don't care about updates that happened *after* we started stopping
        //
        // N.B.: it's important to use `poll` and not `is_stopped` here,
        //       so that *this stream* is polled when the flag is set to stop
        if !*this.stopping && matches!(this.flag.poll(cx), Poll::Ready(())) {
            *this.stopping = true;

            log::trace!("dropping in-flight request");
            this.in_flight.set(None);
        }
        // Poll in-flight future until completion
        else if let Some(in_flight) = this.in_flight.as_mut().as_pin_mut() {
            let res = ready!(in_flight.poll(cx));
            log::trace!("in-flight request completed");
            this.in_flight.set(None);

            match res {
                Ok(_) if *this.stopping => return Ready(None),
                Err(err) if *this.stopping => {
                    // Prevents infinite retries, see https://github.com/teloxide/teloxide/issues/496
                    *this.force_stop = true;

                    return Ready(Some(Err(err)));
                }
                Ok(updates) => {
                    // Once we got the update hense the backoff reconnection strategy worked
                    *this.error_count = 0;

                    if let Some(upd) = updates.last() {
                        *this.offset = upd.id.as_offset();
                    }

                    match *this.drop_pending_updates {
                        false => *this.buffer = updates.into_iter(),
                        true => *this.drop_pending_updates = false,
                    }
                }
                Err(err) => {
                    // Prevents the CPU spike occuring at network connection lose: <https://github.com/teloxide/teloxide/issues/780>
                    let backoff_strategy = &this.polling.backoff_strategy;
                    this.eepy.set(Some(sleep(backoff_strategy(*this.error_count))));
                    log::trace!("set {:?} reconnection delay", backoff_strategy(*this.error_count));
                    return Ready(Some(Err(err)));
                }
            }
        }
        // Poll eepy future until completion, needed for backoff strategy
        else if let Some(eepy) = this.eepy.as_mut().as_pin_mut() {
            ready!(eepy.poll(cx));
            // As soon as delay is waited we increment the counter
            *this.error_count = this.error_count.saturating_add(1);
            log::trace!("current error count: {}", *this.error_count);
            log::trace!("backoff delay completed");
            this.eepy.as_mut().set(None);
        }

        let (offset, limit, timeout) = match (this.stopping, this.drop_pending_updates) {
            // Normal `get_updates()` call
            (false, false) => (*this.offset, this.polling.limit, *this.timeout),
            // Graceful shutdown `get_updates()` call (shutdown takes priority over dropping pending
            // updates)
            //
            // When stopping we set `timeout = 0` and `limit = 1` so that `get_updates()`
            // set last seen update (offset) and return immediately
            (true, _) => {
                log::trace!("graceful shutdown `get_updates` call");
                (*this.offset, Some(1), Some(0))
            }
            // Drop pending updates
            (_, true) => (-1, Some(1), Some(0)),
        };

        let req = this
            .polling
            .bot
            .get_updates()
            .with_payload_mut(|pay| {
                pay.offset = Some(offset);
                pay.timeout = timeout;
                pay.limit = limit;
                pay.allowed_updates = this.allowed_updates.take();
            })
            .send();
        this.in_flight.set(Some(req));

        // Immediately wake up to poll `self.in_flight`
        // (without this this stream becomes a zombie)
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

#[test]
fn polling_is_send() {
    let bot = crate::Bot::new("TOKEN");

    let mut polling = Polling::builder(bot).build();

    assert_send(&polling);
    assert_send(&polling.as_stream());
    assert_send(&polling.stop_token());

    fn assert_send(_: &impl Send) {}
}
