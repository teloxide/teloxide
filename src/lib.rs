#![feature(termination_trait_lib)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde;

mod network;
mod errors;

pub mod bot;
pub mod requests;
pub mod types;

pub use errors::{DownloadError, RequestError};
