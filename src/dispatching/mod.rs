//! Update dispatching.

/// If an update was handled by a dispatcher or not.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DispatchResult {
    Handled,
    Unhandled,
}

pub mod chat;
pub mod filters;
mod handler;
pub mod update_listeners;

pub use filters::Filter;
pub use handler::*;
