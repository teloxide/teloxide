use std::time::Duration;

use futures::Stream;
use teloxide_core::types::Update;

use crate::dispatching::{
    stop_token::StopToken,
    update_listeners::{AsUpdateStream, UpdateListener},
};

/// A listener created from `state` and `stream`/`stop` functions.
pub(crate) struct StatefulListener<St, Assf, Sf, Thf> {
    /// The state of the listener.
    pub(crate) state: St,

    /// Function used as `AsUpdateStream::as_stream`.
    ///
    /// Must be of type `for<'a> &'a mut St -> impl Stream + 'a` and callable by
    /// `&mut`.
    pub(crate) stream: Assf,

    /// Function used as `UpdateListener::stop`.
    ///
    /// Must be of type `for<'a> &'a mut St -> impl StopToken`.
    pub(crate) stop: Sf,

    /// Function used as `UpdateListener::timeout_hint`.
    ///
    /// Must be of type `for<'a> &'a St -> Option<Duration>` and callable by
    /// `&`.
    pub(crate) timeout_hint: Option<Thf>,
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
        (self.stop)(&mut self.state)
    }

    fn timeout_hint(&self) -> Option<Duration> {
        self.timeout_hint.as_ref().and_then(|f| f(&self.state))
    }
}
