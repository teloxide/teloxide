//! Dispatching updates from 1-to-1 chats.
//!
//! It's fairly simple: you have a session type and a handler that accepts
//! (session, update) and turns a session into the next state. When a new
//! user sends a message to your bot, the dispatcher creates a default session
//! and supplies it to your handler, but when an old user sends a message, your
//! handler gets the saved session with him.

// TODO: examples

mod dispatcher;
mod storage;

pub use dispatcher::*;
pub use storage::*;
