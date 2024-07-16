use serde::{Deserialize, Serialize};

use crate::types::UserId;

/// This object contains information about the users whose identifiers were
/// shared with the bot using a [KeyboardButtonRequestUsers] button.
///
/// [KeyboardButtonRequestUsers]: crate::types::KeyboardButtonRequestUsers
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UsersShared {
    /// Identifier of the request
    pub request_id: i32,
    /// Identifiers of the shared users
    pub user_ids: Vec<UserId>,
}
