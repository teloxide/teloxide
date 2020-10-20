//! Bot adaptors

#[cfg(feature = "auto_send")]
#[cfg_attr(docsrs, doc(cfg(feature = "auto_send")))]
pub mod auto_send;
#[cfg(feature = "cache_me")]
#[cfg_attr(docsrs, doc(cfg(feature = "cache_me")))]
pub mod cache_me;
#[cfg(feature = "throttle")]
#[cfg_attr(docsrs, doc(cfg(feature = "throttle")))]
pub mod throttle;

pub use {
    auto_send::AutoSend,
    cache_me::CacheMe,
    throttle::Throttle,
};

// FIXME: move default `parse_mode` to adaptor
