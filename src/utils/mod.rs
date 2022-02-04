//! Some useful utilities.

pub mod command;
pub mod html;
pub mod markdown;
pub(crate) mod shutdown_token;
mod up_state;

pub use teloxide_core::net::client_from_env;

#[cfg(feature = "frunk")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "frunk")))]
pub use up_state::UpState;
