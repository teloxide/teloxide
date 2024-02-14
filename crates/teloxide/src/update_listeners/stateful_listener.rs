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
/// For an example of usage, see [`Polling`].
///
/// [`Polling`]: crate::update_listeners::Polling
#[non_exhaustive]
pub struct StatefulListener<St, Assf, Sf, Hauf> {
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
}

type Haufn<State> = for<'a, 'b> fn(&'a mut State, &'b mut dyn Iterator<Item = AllowedUpdate>);

impl<St, Assf, Sf> StatefulListener<St, Assf, Sf, Haufn<St>> {
    /// Creates a new stateful listener from its components.
    pub fn new(state: St, stream: Assf, stop_token: Sf) -> Self {
        Self::new_with_hints(state, stream, stop_token, None)
    }
}

impl<St, Assf, Sf, Hauf> StatefulListener<St, Assf, Sf, Hauf> {
    /// Creates a new stateful listener from its components.
    pub fn new_with_hints(
        state: St,
        stream: Assf,
        stop_token: Sf,
        hint_allowed_updates: Option<Hauf>,
    ) -> Self {
        Self { state, stream, stop_token, hint_allowed_updates }
    }
}

impl<'a, St, Assf, Sf, Hauf, Strm, E> AsUpdateStream<'a> for StatefulListener<St, Assf, Hauf, Sf>
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

impl<St, Assf, Sf, Hauf, E> UpdateListener for StatefulListener<St, Assf, Sf, Hauf>
where
    Self: for<'a> AsUpdateStream<'a, StreamErr = E>,
    Sf: FnMut(&mut St) -> StopToken,
    Hauf: FnMut(&mut St, &mut dyn Iterator<Item = AllowedUpdate>),
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
}
