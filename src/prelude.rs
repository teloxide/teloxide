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

// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use crate::teloxide;

pub use teloxide_core::types::{
    CallbackQuery, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message, Poll, PollAnswer,
    PreCheckoutQuery, ShippingQuery,
};

#[cfg(feature = "teloxide-core/auto_send")]
pub use crate::adaptors::AutoSend;

#[doc(inline)]
pub use teloxide_core::prelude::*;

#[cfg(feature = "frunk")]
// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "frunk")))]
pub use crate::utils::UpState;

pub use tokio::sync::mpsc::UnboundedReceiver;

pub use futures::StreamExt;
