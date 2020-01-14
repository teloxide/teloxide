//! Dispatching updates from chats.
//!
//! There are four main components:
//!
//!  1. Your session type `Session`, which designates a dialogue state at the
//! current moment.
//!  2. [`Storage`] that encapsulates all the sessions.
//!  3. Your handler of type `H: async Fn(Session, Update) ->
//! HandleResult<Session>` that receives an update and turns your session into
//! the next state.
//!  4. [`Dispatcher`], which encapsulates your handler and [`Storage`], and has
//! the [`dispatch(Update) -> DispatchResult`] function.
//!
//! Every time you call [`.dispatch(update)`] on your dispatcher, the following
//! steps are executed:
//!
//!  1. If a supplied update is not from a chat, return
//! [`DispatchResult::Unhandled`].
//!  2. If a storage doesn't contain a session from this chat, supply
//! `Session::default()` into you handler, otherwise, supply the previous
//! session.
//!  3. If a handler has returned [`SessionState::Terminate`], remove the
//! session from a storage, otherwise force the storage to update the session.
//!
//! [`Storage`]: crate::dispatching::private::Storage
//! [`Dispatcher`]: crate::dispatching::private::Dispatcher
//! [`dispatch(Update) -> DispatchResult`]:
//! crate::dispatching::private::Dispatcher::dispatch
//! [`.dispatch(update)`]: crate::dispatching::private::Dispatcher::dispatch
//! [`DispatchResult::Unhandled`]: crate::dispatching::DispatchResult::Unhandled
//! [`SessionState::Terminate`]: crate::dispatching::SessionState::Terminate

// TODO: examples

mod chat_update;
mod dispatcher;
mod storage;

pub use chat_update::*;
pub use dispatcher::*;
pub use storage::*;
