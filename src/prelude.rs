//! Commonly used items.

pub use crate::{
    dispatching::{
        dialogue::{
            exit, next, DialogueDispatcher, DialogueStage, DialogueWithCx, GetChatId, Transition,
            TransitionIn, TransitionOut,
        },
        Dispatcher, DispatcherHandlerRx, DispatcherHandlerRxExt, UpdateWithCx,
    },
    error_handlers::{LoggingErrorHandler, OnError},
    respond,
};

#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use crate::teloxide;

pub use teloxide_core::types::{
    CallbackQuery, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message, Poll, PollAnswer,
    PreCheckoutQuery, ShippingQuery,
};

#[cfg(feature = "auto-send")]
pub use crate::adaptors::AutoSend;

#[doc(inline)]
pub use teloxide_core::prelude::*;

#[cfg(feature = "frunk")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "frunk")))]
pub use crate::utils::UpState;

pub use tokio::sync::mpsc::UnboundedReceiver;

pub use futures::StreamExt;
