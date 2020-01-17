//! Filters of messages.

pub use main::*;

pub use command::*;
pub use message_caption::*;
pub use message_text::*;
pub use message_text_caption::*;
#[cfg(feature = "regex_filter")]
pub use regex_filter::*;

mod main;

mod command;
mod message_caption;
mod message_text;
mod message_text_caption;
#[cfg(feature = "regex_filter")]
mod regex_filter;
