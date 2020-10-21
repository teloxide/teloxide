//! ## Bot adaptors
//!
//! Bot adaptors are very similar to [`Iterator`] adaptors —
//! they are bots (implement [`Requester`]) which wrap other bots
//! adding new functionality.
//!
//! E.g. [`AutoSend`] allows `await`ing requests directly,
//! without need to use `.send()`.
//!
//! [`Requester`]: crate::requests::Requester

#[cfg(feature = "auto_send")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "auto_send")))]
pub mod auto_send;
#[cfg(feature = "cache_me")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "cache_me")))]
pub mod cache_me;
#[cfg(feature = "throttle")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "throttle")))]
pub mod throttle;

#[cfg(feature = "auto_send")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "auto_send")))]
pub use auto_send::AutoSend;
#[cfg(feature = "cache_me")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "cache_me")))]
pub use cache_me::CacheMe;
#[cfg(feature = "throttle")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "throttle")))]
pub use throttle::Throttle;

// FIXME: move default `parse_mode` to adaptor
