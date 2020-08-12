//! Core part of `teloxide` library.
// TODO: expand docs

// we pass "--cfg docsrs" when building docs to add `This is supported on feature="..." only.`
//
// To properly build docs of this crate run
// ```console
// $ RUSTDOCFLAGS="--cfg docsrs" cargo doc --open --all-features
// ```
#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg))]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
