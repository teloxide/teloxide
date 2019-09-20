#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde;

mod network;

pub mod bot;
pub mod errors;
pub mod requests;
pub mod types;

pub use errors::{DownloadError, RequestError};
