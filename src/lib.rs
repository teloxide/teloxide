#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/dev/logo.svg",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/dev/ICON.png"
)]
#![allow(clippy::match_bool)]

pub use bot::{Bot, BotBuilder};
pub use errors::{ApiErrorKind, DownloadError, RequestError};

mod errors;
mod net;

mod bot;
pub mod dispatching;
pub mod prelude;
pub mod requests;
pub mod types;
pub mod utils;

extern crate teloxide_macros;

/// Expands to a name of your crate.
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}
