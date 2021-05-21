use std::time::Duration;

use futures::Stream;
use teloxide_core::types::Update;

use crate::dispatching::{
    stop_token::{self, StopToken},
    update_listeners::{AsUpdateStream, UpdateListener},
};

/// A listener created from functions.
///
/// This type allows to turn a stream of updates (+some additional functions)
/// into an [`UpdateListener`].
///
/// For an example of usage see [`polling`]
///
/// [`polling`]: crate::dispatching::update_listeners::polling()
#[non_exhaustive]
pub struct StatefulListener<St, Assf, Sf, Thf> {
    /// The state of the listener.
    pub state: St,

    /// Function used as [`AsUpdateStream::as_stream`].
    ///
    /// Must be of type `for<'a> &'a mut St -> impl Stream + 'a` and callable by
    /// `&mut`.
    pub stream: Assf,

    /// Function used as [`UpdateListener::stop_token`].
    ///
    /// Must be of type `for<'a> &'a mut St -> impl StopToken`.
    pub stop_token: Sf,

    /// Function used as [`UpdateListener::timeout_hint`].
    ///
    /// Must be of type `for<'a> &'a St -> Option<Duration>` and callable by
    /// `&`.
    pub timeout_hint: Option<Thf>,
}

impl<St, Assf, Sf, Thf> StatefulListener<St, Assf, Sf, Thf> {
    /// Creates new stateful listener from it's components.
    pub fn new(state: St, stream: Assf, stop_token: Sf, timeout_hint: Option<Thf>) -> Self {
        Self { state, stream, stop_token, timeout_hint }
    }
}

impl<S, E>
    StatefulListener<
        S,
        for<'a> fn(&'a mut S) -> &'a mut S,
        for<'a> fn(&'a mut S) -> stop_token::Noop,
        for<'a> fn(&'a S) -> Option<Duration>,
    >
where
    S: Stream<Item = Result<Update, E>> + Unpin + 'static,
{
    /// Creates a new update listner from a stream of updates which ignore stop
    /// signals.
    ///
    /// It won't be possible to ever stop this listener with stop token.
    pub fn from_stream_without_graceful_shutdown(stream: S) -> Self {
        let this = Self {
            state: stream,
            stream: |s| s,
            stop_token: |_| stop_token::Noop,
            timeout_hint: Some(|_| {
                // FIXME: replace this by just Duration::MAX once 1.53 releases
                // be released
                const NANOS_PER_SEC: u32 = 1_000_000_000;
                let dmax = Duration::new(u64::MAX, NANOS_PER_SEC - 1);

                Some(dmax)
            }),
        };

        assert_update_listener(this)
    }
}

impl<'a, St, Assf, Sf, Thf, Strm, E> AsUpdateStream<'a, E> for StatefulListener<St, Assf, Sf, Thf>
where
    (St, Strm): 'a,
    Assf: FnMut(&'a mut St) -> Strm,
    Strm: Stream<Item = Result<Update, E>>,
{
    type Stream = Strm;

    fn as_stream(&'a mut self) -> Self::Stream {
        (self.stream)(&mut self.state)
    }
}

impl<St, Assf, Sf, Stt, Thf, E> UpdateListener<E> for StatefulListener<St, Assf, Sf, Thf>
where
    Self: for<'a> AsUpdateStream<'a, E>,
    Sf: FnMut(&mut St) -> Stt,
    Stt: StopToken,
    Thf: Fn(&St) -> Option<Duration>,
{
    type StopToken = Stt;

    fn stop_token(&mut self) -> Stt {
        (self.stop_token)(&mut self.state)
    }

    fn timeout_hint(&self) -> Option<Duration> {
        self.timeout_hint.as_ref().and_then(|f| f(&self.state))
    }
}

fn assert_update_listener<L, E>(l: L) -> L
where
    L: UpdateListener<E>,
{
    l
}
