//! [REPL]s for dispatching updates.
//!
//! This module provides functions for easy update handling, that accept a
//! single "handler" function that processes all updates of a certain kind. Note
//! that REPLs are meant to be used as a prototyping tool and lack configuration
//! and some advanced features.
//!
//! [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop

mod commands_repl;
mod repl;

pub use commands_repl::{commands_repl, commands_repl_with_listener};
pub use repl::{repl, repl_with_listener};
