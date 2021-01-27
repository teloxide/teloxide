pub(crate) mod core;
pub mod dialogue;
mod dispatcher;
mod dispatcher_context;
pub mod error_handlers;
mod handlers;
pub(crate) mod repls;
#[cfg(test)]
mod tests;
mod update_listeners;
pub mod update_with_cx;

pub use dispatcher::{Dispatcher, DispatcherBuilder};
pub use handlers::updates;
pub use update_with_cx::UpdateWithCx;

pub mod tel {
    pub use super::handlers::commands::Command;
}
