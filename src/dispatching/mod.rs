//! Update dispatching.

/// If an update was handled by a dispatcher or not.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DispatchResult {
    Handled,
    Unhandled,
}

pub mod chat;
mod handler;
pub mod update_listeners;

pub use handler::*;
