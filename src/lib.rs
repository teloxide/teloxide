//! Core part of `teloxide` library.
// TODO: expand docs

// we pass "--cfg docsrs" when building docs to add `This is supported on feature="..." only.`
//
// To properly build docs of this crate run
// ```console
// $ RUSTDOCFLAGS="--cfg docsrs -Znormalize-docs" cargo doc --open --all-features
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
