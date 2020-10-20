use crate::requests::Requester;

#[cfg(feature = "cache_me")]
use crate::adaptors::CacheMe;

#[cfg(feature = "auto_send")]
use crate::adaptors::AutoSend;

#[cfg(feature = "throttle")]
use crate::adaptors::throttle::{Limits, Throttle};

pub trait RequesterExt: Requester {
    /// Add `get_me` caching ability, see [`CacheMe`] for more.
    ///
    /// [`CacheMe`]:
    #[cfg(feature = "cache_me")]
    #[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "cache_me")))]
    fn cache_me(self) -> CacheMe<Self>
    where
        Self: Sized,
    {
        CacheMe::new(self)
    }

    /// Send requests automatically, see [`AutoSend`] for more.
    #[cfg(feature = "auto_send")]
    #[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "auto_send")))]
    fn auto_send(self) -> AutoSend<Self>
    where
        Self: Sized,
    {
        AutoSend::new(self)
    }

    /// Add throttling ability, see [`Throttle`] for more.
    ///
    /// Note: this spawns the worker, just as [`Throttle::new_spawn`].
    #[cfg(feature = "throttle")]
    #[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "throttle")))]
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
