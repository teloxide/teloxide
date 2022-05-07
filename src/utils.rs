//! Some useful utilities.

pub mod command;
pub mod html;
pub mod markdown;
pub mod message_entity;
pub(crate) mod shutdown_token;

pub use teloxide_core::net::client_from_env;
