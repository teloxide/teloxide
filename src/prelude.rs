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

pub use demux_stream::*;
pub use futures::StreamExt as _;
