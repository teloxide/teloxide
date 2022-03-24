//! REPLs for dispatching updates.

mod commands_repl;
mod repl;

pub use commands_repl::{commands_repl, commands_repl_with_listener};
//pub use dialogues_repl::{dialogues_repl, dialogues_repl_with_listener};
pub use repl::{repl, repl_with_listener};
