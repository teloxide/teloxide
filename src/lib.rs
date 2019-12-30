#![allow(clippy::unit_arg)] // TODO
#![allow(clippy::ptr_arg)] // TODO
#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/dev/logo.svg",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/dev/ICON.png",
)]

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
