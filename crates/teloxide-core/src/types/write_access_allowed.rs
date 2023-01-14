use serde::{Deserialize, Serialize};

/// This object represents a service message about a user allowing a bot added
/// to the attachment menu to write messages. Currently holds no information.
///
/// [The official docs](https://core.telegram.org/bots/api#writeaccessallowed).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WriteAccessAllowed;
