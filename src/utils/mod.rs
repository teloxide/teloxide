//! Some useful utilities.

mod client_from_env;
pub mod command;
pub mod html;
pub mod markdown;
mod up_state;

pub use client_from_env::client_from_env;

#[cfg(feature = "frunk")]
// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "frunk")))]
pub use up_state::UpState;
