use crate::{bot::CacheMe, requests::Requester};

pub trait RequesterExt: Requester {
    /// Add `get_me` caching ability, see [`CacheMe`] for more.
    fn cache_me(self) -> CacheMe<Self>
    where
        Self: Sized,
    {
        CacheMe::new(self)
    }
}

impl<T> RequesterExt for T
where
    T: Requester,
{
    /* use default impls */
}
