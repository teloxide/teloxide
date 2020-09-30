//! Core part of `teloxide` library.
// TODO: expand docs

// we pass "--cfg docsrs" when building docs to add `This is supported on feature="..." only.`
//
// To properly build docs of this crate run
// ```console
// $ RUSTDOCFLAGS="--cfg docsrs" cargo doc --open --all-features
// ```
#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg, doc_spotlight))]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]
#![forbid(unsafe_code)]
//#![deny(missing_docs)]

#[macro_use]
// The internal helper macros.
mod local_macros;

// FIXME(waffle): rethink modules, find a place for wrappers.
pub use self::{
    bot::{AutoSend, Bot, BotBuilder, CacheMe},
    errors::{ApiErrorKind, DownloadError, KnownApiErrorKind, RequestError},
};

pub mod payloads;
pub mod prelude;
pub mod requests;
pub mod types;

// FIXME(waffle): made `pub` to reexport bot wrappers, in future we may want to
//                reexport them from elsewhere
pub mod bot;

// reexported
mod errors;

// implementation details
mod net;
mod serde_multipart;

/// Constructs a client from the `TELOXIDE_PROXY` environmental variable.
///
/// This function passes the value of `TELOXIDE_PROXY` into
/// [`reqwest::Proxy::all`], if it exists, otherwise returns the default
/// client.
///
/// # Note
/// The created client will have safe settings, meaning that it will be able to
/// work in long time durations, see the [issue 223].
///
/// [`reqwest::Proxy::all`]: https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html#method.all
/// [issue 223]: https://github.com/teloxide/teloxide/issues/223
pub fn client_from_env() -> reqwest::Client {
    use crate::bot::{sound_bot, TELOXIDE_PROXY};
    use reqwest::Proxy;

    let builder = sound_bot();

    match std::env::var(TELOXIDE_PROXY).ok() {
        Some(proxy) => builder.proxy(Proxy::all(&proxy).expect("creating reqwest::Proxy")),
        None => builder,
    }
    .build()
    .expect("creating reqwest::Client")
}
