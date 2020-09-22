use crate::{payloads::GetMe, requests::Request};

/// The trait implemented by all bots & bot wrappers.
/// Essentially a request builder factory (?).
///
/// _This trait is included in the crate's [`prelude`](crate::prelude)_.
#[cfg_attr(all(docsrs, feature = "nightly"), doc(spotlight))]
pub trait Requester {
    type GetMe: Request<Payload = GetMe>;

    /// For telegram documentation of the method see [`GetMe`].
    fn get_me(&self) -> Self::GetMe;

    // FIXME(waffle): add remaining 69 methods
}
