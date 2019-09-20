#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde;

pub mod bot;
pub mod core;
pub mod errors;

pub use errors::DownloadError;
