//! Support for user dialogues.
//!
//! The main type is (surprise!) [`Dialogue`]. Under the hood, it is just a
//! wrapper over [`Storage`] and a chat ID. All it does is provides convenient
//! method for manipulating the dialogue state. [`Storage`] is where all
//! dialogue states are stored; it can be either [`InMemStorage`], which is a
//! simple hash map, or database wrappers such as [`SqliteStorage`]. In the
//! latter case, your dialogues are _persistent_, meaning that you can safely
//! restart your bot and all dialogues will remain in the database -- this is a
//! preferred method for production bots.
//!
//! [`examples/dialogue.rs`] clearly demonstrates the typical usage of
//! dialogues. Your dialogue state can be represented as an enumeration:
//!
//! ```ignore
//! #[derive(Clone)]
//! pub enum State {
//!     Start,
//!     ReceiveFullName,
//!     ReceiveAge { full_name: String },
//!     ReceiveLocation { full_name: String, age: u8 },
//! }
//! ```
//!
//! Each state is associated with its respective handler: e.g., when a dialogue
//! state is `ReceiveAge`, `receive_age` is invoked:
//!
//! ```ignore
//! async fn receive_age(
//!     bot: AutoSend<Bot>,
//!     msg: Message,
//!     dialogue: MyDialogue,
//!     (full_name,): (String,), // Available from `State::ReceiveAge`.
//! ) -> anyhow::Result<()> {
//!     match msg.text().map(|text| text.parse::<u8>()) {
//!         Some(Ok(age)) => {
//!             bot.send_message(msg.chat.id, "What's your location?").await?;
//!             dialogue.update(State::ReceiveLocation { full_name, age }).await?;
//!         }
//!         _ => {
//!             bot.send_message(msg.chat.id, "Send me a number.").await?;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! Variant's fields are passed to state handlers as tuples: `(full_name,):
//! (String,)`. Using [`Dialogue::update`], you can update the dialogue with a
//! new state, in our case -- `State::ReceiveLocation { full_name, age }`. To
//! exit the dialogue, just call [`Dialogue::exit`] and it will be removed from
//! the inner storage:
//!
//! ```ignore
//! async fn receive_location(
//!     bot: AutoSend<Bot>,
//!     msg: Message,
//!     dialogue: MyDialogue,
//!     (full_name, age): (String, u8), // Available from `State::ReceiveLocation`.
//! ) -> anyhow::Result<()> {
//!     match msg.text() {
//!         Some(location) => {
//!             let message =
//!                 format!("Full name: {}\nAge: {}\nLocation: {}", full_name, age, location);
//!             bot.send_message(msg.chat.id, message).await?;
//!             dialogue.exit().await?;
//!         }
//!         None => {
//!             bot.send_message(msg.chat.id, "Send me a text message.").await?;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [`examples/dialogue.rs`]: https://github.com/teloxide/teloxide/blob/master/examples/dialogue.rs

#[cfg(feature = "redis-storage")]
pub use crate::dispatching::dialogue::{RedisStorage, RedisStorageError};

#[cfg(feature = "sqlite-storage")]
pub use crate::dispatching::dialogue::{SqliteStorage, SqliteStorageError};

pub use get_chat_id::GetChatId;
pub use storage::*;
use teloxide_core::types::ChatId;

use std::{marker::PhantomData, sync::Arc};

mod get_chat_id;
mod storage;

/// A handle for controlling dialogue state.
#[derive(Debug)]
pub struct Dialogue<D, S>
where
    S: ?Sized,
{
    storage: Arc<S>,
    chat_id: ChatId,
    _phantom: PhantomData<D>,
}

// `#[derive]` requires generics to implement `Clone`, but `S` is wrapped around
// `Arc`, and `D` is wrapped around PhantomData.
impl<D, S> Clone for Dialogue<D, S>
where
    S: ?Sized,
{
    fn clone(&self) -> Self {
        Dialogue { storage: self.storage.clone(), chat_id: self.chat_id, _phantom: PhantomData }
    }
}

impl<D, S> Dialogue<D, S>
where
    D: Send + 'static,
    S: Storage<D> + ?Sized,
{
    /// Constructs a new dialogue with `storage` (where dialogues are stored)
    /// and `chat_id` of a current dialogue.
    #[must_use]
    pub fn new(storage: Arc<S>, chat_id: ChatId) -> Self {
        Self { storage, chat_id, _phantom: PhantomData }
    }

    /// Returns a chat ID associated with this dialogue.
    #[must_use]
    pub fn chat_id(&self) -> ChatId {
        self.chat_id
    }

    /// Retrieves the current state of the dialogue or `None` if there is no
    /// dialogue.
    pub async fn get(&self) -> Result<Option<D>, S::Error> {
        self.storage.clone().get_dialogue(self.chat_id).await
    }

    /// Like [`Dialogue::get`] but returns a default value if there is no
    /// dialogue.
    pub async fn get_or_default(&self) -> Result<D, S::Error>
    where
        D: Default,
    {
        match self.get().await? {
            Some(d) => Ok(d),
            None => {
                self.storage.clone().update_dialogue(self.chat_id, D::default()).await?;
                Ok(D::default())
            }
        }
    }

    /// Updates the dialogue state.
    ///
    /// The dialogue type `D` must implement `From<State>` to allow implicit
    /// conversion from `State` to `D`.
    pub async fn update<State>(&self, state: State) -> Result<(), S::Error>
    where
        D: From<State>,
    {
        let new_dialogue = state.into();
        self.storage.clone().update_dialogue(self.chat_id, new_dialogue).await?;
        Ok(())
    }

    /// Updates the dialogue with a default value.
    pub async fn reset(&self) -> Result<(), S::Error>
    where
        D: Default,
    {
        self.update(D::default()).await
    }

    /// Removes the dialogue from the storage provided to [`Dialogue::new`].
    pub async fn exit(&self) -> Result<(), S::Error> {
        self.storage.clone().remove_dialogue(self.chat_id).await
    }
}

/// Perform a dialogue FSM transition.
///
/// This macro expands to a [`dptree::Handler`] that filters your dialogue
/// state: if the state enumeration is of a certain variant, the execution
/// continues; otherwise, `dptree` will try the next branch.
///
/// Variants can take the following forms:
///
///  - `State::MyVariant` for empty variants;
///  - `State::MyVariant(param1, ..., paramN)` for function-like variants;
///  - `State::MyVariant { param1, ..., paramN }` for `struct`-like variants.
///
/// In the first case, this macro results in a simple [`dptree::filter`]; in the
/// second and third cases, this macro results in [`dptree::filter_map`] that
/// passes the payload of `MyVariant` to the next handler if the match occurs.
/// (This next handler can be an endpoint or a more complex one.) The payload
/// format depend on the form of `MyVariant`:
///
///  - For `State::MyVariant(param)` and `State::MyVariant { param }`, the
///    payload is `param`.
///  - For `State::MyVariant(param,)` and `State::MyVariant { param, }`, the
///    payload is `(param,)`.
///  - For `State::MyVariant(param1, ..., paramN)` and `State::MyVariant {
///    param1, ..., paramN }`, the payload is `(param1, ..., paramN)` (where
///    `N`>1).
///
/// ## Dependency requirements
///
///  - Your dialogue state enumeration `State`.
#[macro_export]
macro_rules! handler {
    ($($variant:ident)::+) => {
        $crate::dptree::filter(|state| matches!(state, $($variant)::+))
    };
    ($($variant:ident)::+ ($param:ident)) => {
        $crate::dptree::filter_map(|state| match state {
            $($variant)::+($param) => Some($param),
            _ => None,
        })
    };
    ($($variant:ident)::+ ($($param:ident),+ $(,)?)) => {
        $crate::dptree::filter_map(|state| match state {
            $($variant)::+($($param),+) => Some(($($param),+ ,)),
            _ => None,
        })
    };
    ($($variant:ident)::+ {$param:ident}) => {
        $crate::dptree::filter_map(|state| match state {
            $($variant)::+{$param} => Some($param),
            _ => None,
        })
    };
    ($($variant:ident)::+ {$($param:ident),+ $(,)?}) => {
        $crate::dptree::filter_map(|state| match state {
            $($variant)::+ { $($param),+ } => Some(($($param),+ ,)),
            _ => None,
        })
    };
}

#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum State {
        A,
        B(i32),
        C(i32, &'static str),
        D { foo: i32 },
        E { foo: i32, bar: &'static str },
        Other,
    }

    #[tokio::test]
    async fn handler_empty_variant() {
        let input = State::A;
        let h: dptree::Handler<_, _> = handler![State::A].endpoint(|| async move { 123 });

        assert_eq!(h.dispatch(dptree::deps![input]).await, ControlFlow::Break(123));
        assert!(matches!(h.dispatch(dptree::deps![State::Other]).await, ControlFlow::Continue(_)));
    }

    #[tokio::test]
    async fn handler_single_fn_variant() {
        let input = State::B(42);
        let h: dptree::Handler<_, _> = handler![State::B(x)].endpoint(|x: i32| async move {
            assert_eq!(x, 42);
            123
        });

        assert_eq!(h.dispatch(dptree::deps![input]).await, ControlFlow::Break(123));
        assert!(matches!(h.dispatch(dptree::deps![State::Other]).await, ControlFlow::Continue(_)));
    }

    #[tokio::test]
    async fn handler_single_fn_variant_trailing_comma() {
        let input = State::B(42);
        let h: dptree::Handler<_, _> = handler![State::B(x,)].endpoint(|(x,): (i32,)| async move {
            assert_eq!(x, 42);
            123
        });

        assert_eq!(h.dispatch(dptree::deps![input]).await, ControlFlow::Break(123));
        assert!(matches!(h.dispatch(dptree::deps![State::Other]).await, ControlFlow::Continue(_)));
    }

    #[tokio::test]
    async fn handler_fn_variant() {
        let input = State::C(42, "abc");
        let h: dptree::Handler<_, _> =
            handler![State::C(x, y)].endpoint(|(x, str): (i32, &'static str)| async move {
                assert_eq!(x, 42);
                assert_eq!(str, "abc");
                123
            });

        assert_eq!(h.dispatch(dptree::deps![input]).await, ControlFlow::Break(123));
        assert!(matches!(h.dispatch(dptree::deps![State::Other]).await, ControlFlow::Continue(_)));
    }

    #[tokio::test]
    async fn handler_single_struct_variant() {
        let input = State::D { foo: 42 };
        let h: dptree::Handler<_, _> = handler![State::D { foo }].endpoint(|x: i32| async move {
            assert_eq!(x, 42);
            123
        });

        assert_eq!(h.dispatch(dptree::deps![input]).await, ControlFlow::Break(123));
        assert!(matches!(h.dispatch(dptree::deps![State::Other]).await, ControlFlow::Continue(_)));
    }

    #[tokio::test]
    async fn handler_single_struct_variant_trailing_comma() {
        let input = State::D { foo: 42 };
        #[rustfmt::skip] // rustfmt removes the trailing comma from `State::D { foo, }`, but it plays a vital role in this test.
        let h: dptree::Handler<_, _> = handler![State::D { foo, }].endpoint(|(x,): (i32,)| async move {
            assert_eq!(x, 42);
            123
        });

        assert_eq!(h.dispatch(dptree::deps![input]).await, ControlFlow::Break(123));
        assert!(matches!(h.dispatch(dptree::deps![State::Other]).await, ControlFlow::Continue(_)));
    }

    #[tokio::test]
    async fn handler_struct_variant() {
        let input = State::E { foo: 42, bar: "abc" };
        let h: dptree::Handler<_, _> =
            handler![State::E { foo, bar }].endpoint(|(x, str): (i32, &'static str)| async move {
                assert_eq!(x, 42);
                assert_eq!(str, "abc");
                123
            });

        assert_eq!(h.dispatch(dptree::deps![input]).await, ControlFlow::Break(123));
        assert!(matches!(h.dispatch(dptree::deps![State::Other]).await, ControlFlow::Continue(_)));
    }
}
