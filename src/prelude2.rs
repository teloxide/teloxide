//! Commonly used items (dispatching2 version).

pub use crate::{
    error_handlers::{LoggingErrorHandler, OnError},
    respond,
};

pub use crate::dispatching2::{
    dialogue::{Dialogue, DialogueHandlerExt as _},
    Dispatcher, HandlerExt as _, MessageFilterExt as _, UpdateFilterExt as _,
};

#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use crate::teloxide;

pub use teloxide_core::types::{
    CallbackQuery, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message, Poll, PollAnswer,
    PreCheckoutQuery, ShippingQuery, Update,
};

#[cfg(feature = "auto-send")]
pub use crate::adaptors::AutoSend;

#[doc(no_inline)]
pub use teloxide_core::prelude::*;

pub use dptree::{self, prelude::*};
