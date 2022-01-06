use std::time::Duration;

use futures::Stream;

use crate::{
    dispatching::{
        stop_token::{self, StopToken},
        update_listeners::{AsUpdateStream, UpdateListener},
    },
    types::{AllowedUpdate, Update},
};

/// A listener created from functions.
///
/// This type allows to turn a stream of updates (+ some additional functions)
/// into an [`UpdateListener`].
///
/// For an example of usage, see [`polling`].
///
/// [`polling`]: crate::dispatching::update_listeners::polling()
#[non_exhaustive]
pub struct StatefulListener<St, Assf, Sf, Hauf, Thf> {
    /// The state of the listener.
    pub state: St,

    /// The function used as [`AsUpdateStream::as_stream`].
    ///
    /// Must be of type `for<'a> &'a mut St -> impl Stream + 'a` and callable by
    /// `&mut`.
    pub stream: Assf,

    /// The function used as [`UpdateListener::stop_token`].
    ///
    /// Must be of type `for<'a> &'a mut St -> impl StopToken`.
    pub stop_token: Sf,

    /// The function used as [`UpdateListener::hint_allowed_updates`].
    ///
    /// Must be of type `for<'a, 'b> &'a mut St, &'b mut dyn Iterator<Item =
    /// AllowedUpdate>  -> ()`.
    pub hint_allowed_updates: Option<Hauf>,

    /// The function used as [`UpdateListener::timeout_hint`].
    ///
    /// Must be of type `for<'a> &'a St -> Option<Duration>` and callable by
    /// `&`.
    pub timeout_hint: Option<Thf>,
}

type Haufn<State> = for<'a, 'b> fn(&'a mut State, &'b mut dyn Iterator<Item = AllowedUpdate>);
type Thfn<State> = for<'a> fn(&'a State) -> Option<Duration>;

impl<St, Assf, Sf> StatefulListener<St, Assf, Sf, Haufn<St>, Thfn<St>> {
    /// Creates a new stateful listener from its components.
    pub fn new(state: St, stream: Assf, stop_token: Sf) -> Self {
        Self::new_with_hints(state, stream, stop_token, None, None)
    }
}

impl<St, Assf, Sf, Hauf, Thf> StatefulListener<St, Assf, Sf, Hauf, Thf> {
    /// Creates a new stateful listener from its components.
    pub fn new_with_hints(
        state: St,
        stream: Assf,
        stop_token: Sf,
        hint_allowed_updates: Option<Hauf>,
        timeout_hint: Option<Thf>,
    ) -> Self {
        Self { state, stream, stop_token, hint_allowed_updates, timeout_hint }
    }
}

impl<S, E>
    StatefulListener<
        S,
        for<'a> fn(&'a mut S) -> &'a mut S,
        for<'a> fn(&'a mut S) -> stop_token::Noop,
        Haufn<S>,
        Thfn<S>,
    >
where
    S: Stream<Item = Result<Update, E>> + Unpin + Send + 'static,
{
    /// Creates a new update listener from a stream of updates which ignores
    /// stop signals.
    ///
    /// It won't be possible to ever stop this listener with a stop token.
    pub fn from_stream_without_graceful_shutdown(stream: S) -> Self {
        let this = Self::new_with_hints(
            stream,
            |s| s,
            |_| stop_token::Noop,
            None,
            Some(|_| {
                // FIXME: replace this by just Duration::MAX once 1.53 releases
                // be released
                const NANOS_PER_SEC: u32 = 1_000_000_000;
                let dmax = Duration::new(u64::MAX, NANOS_PER_SEC - 1);

                Some(dmax)
            }),
        );

        assert_update_listener(this)
    }
}

impl<'a, St, Assf, Sf, Hauf, Thf, Strm, E> AsUpdateStream<'a, E>
    for StatefulListener<St, Assf, Hauf, Sf, Thf>
where
    (St, Strm): 'a,
    Strm: Send,
    Assf: FnMut(&'a mut St) -> Strm,
    Strm: Stream<Item = Result<Update, E>>,
{
    type Stream = Strm;

    fn as_stream(&'a mut self) -> Self::Stream {
        (self.stream)(&mut self.state)
    }
}

impl<St, Assf, Sf, Hauf, Stt, Thf, E> UpdateListener<E>
    for StatefulListener<St, Assf, Sf, Hauf, Thf>
where
    Self: for<'a> AsUpdateStream<'a, E>,
    Sf: FnMut(&mut St) -> Stt,
    Stt: StopToken,
    Hauf: FnMut(&mut St, &mut dyn Iterator<Item = AllowedUpdate>),
    Thf: Fn(&St) -> Option<Duration>,
{
    type StopToken = Stt;

    fn stop_token(&mut self) -> Stt {
        (self.stop_token)(&mut self.state)
    }

    fn hint_allowed_updates(&mut self, hint: &mut dyn Iterator<Item = AllowedUpdate>) {
        if let Some(f) = &mut self.hint_allowed_updates {
            f(&mut self.state, hint);
        }
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
