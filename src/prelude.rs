//! Commonly used items.

pub use crate::{
    dispatching::UpdateWithCx,
    requests::{respond, Request, ResponseResult},
    types::{Message, Update},
    Bot, RequestError,
};

#[cfg(feature = "frunk")]
// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "frunk")))]
pub use crate::utils::UpState;

pub use tokio::sync::mpsc::UnboundedReceiver;

pub use futures::StreamExt;
