#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

pub use errors::{DownloadError, RequestError};

mod errors;
mod network;

pub mod bot;
pub mod requests;
pub mod types;
