//! Commonly used items.

#![deprecated(note = "Use dispatching2 instead")]
#![allow(deprecated)]

pub use crate::{
    error_handlers::{LoggingErrorHandler, OnError},
    respond,
};

pub use crate::dispatching::{
    dialogue::{
        exit, next, DialogueDispatcher, DialogueStage, DialogueWithCx, GetChatId, Transition,
        TransitionIn, TransitionOut,
    },
    Dispatcher, DispatcherHandlerRx, DispatcherHandlerRxExt, UpdateWithCx,
};

#[cfg(feature = "macros")]
pub use crate::teloxide;

pub use teloxide_core::types::{
    CallbackQuery, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message, Poll, PollAnswer,
    PreCheckoutQuery, ShippingQuery,
};

#[cfg(feature = "auto-send")]
pub use crate::adaptors::AutoSend;

#[doc(no_inline)]
pub use teloxide_core::prelude::*;

#[cfg(feature = "frunk")]
pub use crate::utils::UpState;

pub use tokio::sync::mpsc::UnboundedReceiver;

pub use futures::StreamExt;
