//! [REPL]s for dispatching updates.
//!
//! This module provides utilities for easy update handling. They accept a
//! single "handler" function that processes all updates of a certain kind. Note
//! that REPLs are meant to be used for simple scenarios, such as prototyping,
//! inasmuch they lack configuration and some [advanced features].
//!
//! [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
//! [advanced features]: crate::dispatching#dispatching-or-repls

mod commands_repl;
mod repl;

pub use commands_repl::CommandReplExt;
#[allow(deprecated)]
pub use commands_repl::{commands_repl, commands_repl_with_listener};
pub use repl::{repl, repl_with_listener, try_repl, try_repl_with_listener};
