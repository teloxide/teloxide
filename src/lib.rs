#![feature(type_alias_impl_trait)]
#![cfg_attr(feature = "never-type", feature(never_type))]
#![allow(clippy::unit_arg)] // TODO
#![allow(clippy::ptr_arg)] // TODO

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

pub use bot::Bot;
pub use errors::{DownloadError, RequestError};

mod errors;
mod network;

mod bot;
pub mod dispatching;
pub mod requests;
pub mod types;
