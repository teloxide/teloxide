//! Wrappers altering functionality of a bot.
//!
//! Bot adaptors are very similar to the [`Iterator`] adaptors: they are bots
//! wrapping other bots to alter existing or add new functionality.
//!
//! E.g. [`AutoSend`] allows `await`ing requests directly, no need to use
//! `.send()`.
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

mod parse_mode;

#[cfg(feature = "auto_send")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "auto_send")))]
pub use auto_send::AutoSend;
#[cfg(feature = "cache_me")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "cache_me")))]
pub use cache_me::CacheMe;
#[cfg(feature = "throttle")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "throttle")))]
pub use throttle::Throttle;

pub use parse_mode::DefaultParseMode;

// FIXME: move default `parse_mode` to adaptor
