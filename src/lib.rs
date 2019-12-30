#![allow(clippy::unit_arg)] // TODO
#![allow(clippy::ptr_arg)] // TODO
#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/dev/ICON.png",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/dev/ICON.png"
)]

pub use bot::Bot;
pub use errors::{DownloadError, RequestError};

mod errors;
mod network;

mod bot;
pub mod dispatching;
pub mod requests;
pub mod types;
