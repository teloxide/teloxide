//! Dispatching updates from 1-to-1 chats.
//!
//! There are four main components:
//!
//!  1. Your session type `Session`, which designates a dialogue state at the
//! current moment.
//!  2. A storage that encapsulates all the sessions.
//!  3. Your handler of type `H: async Fn(Session, Update) ->
//! HandleResult<Session>` that receives an update and turns your session into
//! the next state.
//!  4. The dispatcher, which encapsulates your handler and [`Storage`], and has
//! the `dispatch(Update) -> DispatchResult` function.
//!
//! Every time you call `.dispatch(update)` on your updater, the following steps
//! are executed:
//!
//!  1. If a supplied update is not from a 1-to-1 chat, return
//! `DispatchResult::Unhandled`.
//!  2. If a storage doesn't contain a session from this chat, supply
//! `Session::default()` into you handler, otherwise, supply the previous
//! session.
//!  3. If a handler has returned `HandleResult::Terminate`, remove the sesion
//! from a storage, otherwise force the storage to update the session.

// TODO: examples

mod dispatcher;
mod storage;

pub use dispatcher::*;
pub use storage::*;
