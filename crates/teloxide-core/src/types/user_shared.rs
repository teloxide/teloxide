use serde::{Deserialize, Serialize};

use crate::types::UserId;

/// Information about the chat whose identifier was shared with the bot using a
/// [`KeyboardButtonRequestUser`] button.
///
/// [`KeyboardButtonRequestUser`]: crate::types::KeyboardButtonRequestUser
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UserShared {
    /// Identifier of the request.
    pub request_id: i32,
    /// Identifier of the shared user.
    pub user_id: UserId,
}
