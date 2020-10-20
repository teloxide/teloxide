use crate::{
    adaptors::{CacheMe, throttle::Limits, Throttle, AutoSend},
    requests::Requester,
};

pub trait RequesterExt: Requester {
    /// Add `get_me` caching ability, see [`CacheMe`] for more.
    fn cache_me(self) -> CacheMe<Self>
    where
        Self: Sized,
    {
        CacheMe::new(self)
    }

    /// Send requests automatically, see [`AutoSend`] for more.
    fn auto_send(self) -> AutoSend<Self>
    where
        Self: Sized,
    {
        AutoSend::new(self)
    }

    /// Add throttling ability, see [`Throttle`] for more.
    ///
    /// Note: this spawns the worker, just as [`Throttle::new_spawn`].
    fn throttle(self, limits: Limits) -> Throttle<Self>
    where
        Self: Sized,
        // >:(
        // (waffle)
        Self: 'static,
    {
        Throttle::new_spawn(self, limits)
    }
}

impl<T> RequesterExt for T
where
    T: Requester,
{
    /* use default impls */
}
