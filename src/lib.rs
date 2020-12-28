//! Core part of `teloxide` library.
// TODO: expand docs

// we pass "--cfg docsrs" when building docs to add `This is supported on feature="..." only.`
//
// To properly build docs of this crate run
// ```console
// $ RUSTDOCFLAGS="--cfg docsrs" cargo doc --open --all-features
// ```
#![forbid(unsafe_code)]
#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg, doc_spotlight))]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]
#![cfg_attr(feature = "full", deny(broken_intra_doc_links))]
//#![deny(missing_docs)]

// The internal helper macros.
#[macro_use]
mod local_macros;

pub use self::{
    bot::Bot,
    errors::{ApiError, DownloadError, RequestError},
};

pub mod adaptors;
pub mod net;
pub mod payloads;
pub mod prelude;
pub mod requests;
pub mod types;

// reexported
mod bot;
mod errors;

// implementation details
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
