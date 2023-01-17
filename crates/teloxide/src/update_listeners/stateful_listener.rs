use std::time::Duration;

use futures::Stream;

use crate::{
    stop::StopToken,
    types::{AllowedUpdate, Update},
    update_listeners::{AsUpdateStream, UpdateListener},
};

/// A listener created from functions.
///
/// This type allows to turn a stream of updates (+ some additional functions)
/// into an [`UpdateListener`].
///
/// For an example of usage, see [`polling`].
///
/// [`polling`]: crate::update_listeners::polling()
#[non_exhaustive]
pub struct StatefulListener<St, Assf, Sf, Hauf, Thf> {
    /// The state of the listener.
    pub state: St,

    /// The function used as [`AsUpdateStream::as_stream`].
    ///
    /// Must implement `for<'a> FnMut(&'a mut St) -> impl Stream + 'a`.
    pub stream: Assf,

    /// The function used as [`UpdateListener::stop_token`].
    ///
    /// Must implement `FnMut(&mut St) -> StopToken`.
    pub stop_token: Sf,

    /// The function used as [`UpdateListener::hint_allowed_updates`].
    ///
    /// Must implement `FnMut(&mut St, &mut dyn Iterator<Item =
    /// AllowedUpdate>)`.
    pub hint_allowed_updates: Option<Hauf>,

    /// The function used as [`UpdateListener::timeout_hint`].
    ///
    /// Must implement `Fn(&St) -> Option<Duration>`.
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

impl<'a, St, Assf, Sf, Hauf, Thf, Strm, E> AsUpdateStream<'a>
    for StatefulListener<St, Assf, Hauf, Sf, Thf>
where
    (St, Strm): 'a,
    Strm: Send,
    Assf: FnMut(&'a mut St) -> Strm,
    Strm: Stream<Item = Result<Update, E>>,
{
    type StreamErr = E;
    type Stream = Strm;

    fn as_stream(&'a mut self) -> Self::Stream {
        (self.stream)(&mut self.state)
    }
}

impl<St, Assf, Sf, Hauf, Thf, E> UpdateListener for StatefulListener<St, Assf, Sf, Hauf, Thf>
where
    Self: for<'a> AsUpdateStream<'a, StreamErr = E>,
    Sf: FnMut(&mut St) -> StopToken,
    Hauf: FnMut(&mut St, &mut dyn Iterator<Item = AllowedUpdate>),
    Thf: Fn(&St) -> Option<Duration>,
{
    type Err = E;

    fn stop_token(&mut self) -> StopToken {
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
