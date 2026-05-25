use serde::{Deserialize, Serialize};

use crate::types::{RequestId, SharedUser};

/// This object contains information about the users whose identifiers were
/// shared with the bot using a [KeyboardButtonRequestUsers] button.
///
/// [KeyboardButtonRequestUsers]: crate::types::KeyboardButtonRequestUsers
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UsersShared {
    /// Identifier of the request
    pub request_id: RequestId,
    /// Identifiers of the shared users
    pub users: Vec<SharedUser>,
}
