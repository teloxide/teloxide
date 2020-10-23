//! Payloads - data types sended to relegram

pub mod setters;

mod get_me;
mod send_message;

pub use get_me::{GetMe, GetMeSetters};
pub use send_message::{SendMessage, SendMessageSetters};
