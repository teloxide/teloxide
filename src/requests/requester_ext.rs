use crate::{requests::Requester, AutoSend, CacheMe};

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
}

impl<T> RequesterExt for T
where
    T: Requester,
{
    /* use default impls */
}
