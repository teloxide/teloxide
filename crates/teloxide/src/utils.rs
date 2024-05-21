//! Some useful utilities.

pub mod command;
pub mod html;
pub mod markdown;
pub(crate) mod shutdown_token;
pub mod unparser;

pub use teloxide_core::net::client_from_env;
