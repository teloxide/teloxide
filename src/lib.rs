#![feature(termination_trait_lib)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde;

pub use errors::{DownloadError, RequestError};

mod errors;
mod network;

pub mod bot;
pub mod requests;
pub mod types;
