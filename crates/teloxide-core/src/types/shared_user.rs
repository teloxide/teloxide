use serde::{Deserialize, Serialize};

use crate::types::{PhotoSize, UserId};

/// This object contains information about a user that was shared with the bot
/// using a [`KeyboardButtonRequestUsers`] button.
///
/// [`KeyboardButtonRequestUsers`]: crate::types::KeyboardButtonRequestUsers
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct SharedUser {
    /// Identifier of the shared user
    pub user_id: UserId,

    /// First name of the user, if it was requested by the bot
    pub first_name: Option<String>,

    /// Last name of the user, if it was requested by the bot
    pub last_name: Option<String>,

    /// Username of the user, if it was requested by the bot
    pub username: Option<String>,

    /// Available sizes of the chat photo, if it was requested
    pub photo: Option<Vec<PhotoSize>>,
}
