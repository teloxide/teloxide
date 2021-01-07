//! Some useful utilities.

mod client_from_env;
pub mod command;
pub mod html;
pub mod markdown;
mod up_state;

pub use client_from_env::client_from_env;

#[cfg(feature = "frunk")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "frunk")))]
pub use up_state::UpState;
