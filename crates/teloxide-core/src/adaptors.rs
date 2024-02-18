//! Wrappers altering functionality of a bot.
//!
//! Bot adaptors are very similar to the [`Iterator`] adaptors: they are bots
//! wrapping other bots to alter existing or add new functionality.
//!
//! [`Requester`]: crate::requests::Requester

/// [`CacheMe`] bot adaptor which caches [`GetMe`] requests.
///
/// [`CacheMe`]: cache_me::CacheMe
/// [`GetMe`]: crate::payloads::GetMe
#[cfg(feature = "cache_me")]
pub mod cache_me;

/// [`Trace`] bot adaptor which traces requests.
///
/// [`Trace`]: trace::Trace
#[cfg(feature = "trace_adaptor")]
pub mod trace;

/// [`ErasedRequester`] bot adaptor which allows to erase type of
/// [`Requester`].
///
/// [`ErasedRequester`]: erased::ErasedRequester
/// [`Requester`]: crate::requests::Requester
#[cfg(feature = "erased")]
pub mod erased;

/// [`Throttle`] bot adaptor which allows automatically throttle when hitting
/// API limits.
///
/// [`Throttle`]: throttle::Throttle
#[cfg(feature = "throttle")]
pub mod throttle;

mod parse_mode;

#[cfg(feature = "cache_me")]
pub use cache_me::CacheMe;
#[cfg(feature = "erased")]
pub use erased::ErasedRequester;
#[cfg(feature = "throttle")]
pub use throttle::Throttle;
#[cfg(feature = "trace_adaptor")]
pub use trace::Trace;

pub use parse_mode::DefaultParseMode;
