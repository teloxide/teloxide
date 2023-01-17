//! Commonly used items.

pub use crate::error_handlers::{LoggingErrorHandler, OnError};

#[allow(deprecated)]
pub use crate::respond;

pub use crate::dispatching::{
    dialogue::Dialogue, Dispatcher, HandlerExt as _, MessageFilterExt as _, UpdateFilterExt as _,
};

#[cfg(feature = "ctrlc_handler")]
pub use crate::repls::CommandReplExt as _;

pub use teloxide_core::{
    requests::ResponseResult,
    types::{
        CallbackQuery, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message, Poll,
        PollAnswer, PreCheckoutQuery, ShippingQuery, Update,
    },
};

#[cfg(feature = "auto-send")]
#[allow(deprecated)]
pub use crate::adaptors::AutoSend;

#[doc(no_inline)]
pub use teloxide_core::prelude::*;

pub use dptree::{self, prelude::*};
