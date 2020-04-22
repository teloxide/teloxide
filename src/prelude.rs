//! Commonly used items.

pub use crate::{
    dispatching::{
        dialogue::{
            exit, next, DialogueDispatcher, DialogueDispatcherHandlerCx,
            DialogueStage, GetChatId,
        },
        update_listeners::polling_default,
        StreamExt as _, UpdateWithCx,
    },
    error_handlers::{LoggingErrorHandler, OnError},
    requests::{Request, ResponseResult},
    types::{Message, Update},
    Bot, RequestError,
};

pub use tokio::sync::mpsc::UnboundedReceiver;

pub use demux_stream::*;
pub use enum_extract::extract;
pub use frunk::{Coprod, Hlist};
pub use futures::StreamExt as _;
