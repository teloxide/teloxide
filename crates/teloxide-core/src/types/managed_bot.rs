use serde::{Deserialize, Serialize};

use crate::types::{User, UserId};

/// This object contains information about the bot that was created to be
/// managed by the current bot.
///
/// [The official docs](https://core.telegram.org/bots/api#managedbotcreated).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ManagedBotCreated {
    pub bot: User,
}

/// This object contains information about creation, token update, or owner
/// update of a bot managed by the current bot.
///
/// [The official docs](https://core.telegram.org/bots/api#managedbotupdated).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ManagedBotUpdated {
    pub user: User,
    pub bot: User,
}

/// This object describes the access settings of a bot.
///
/// [The official docs](https://core.telegram.org/bots/api#botaccesssettings).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotAccessSettings {
    pub is_access_restricted: bool,
    pub added_users: Option<Vec<User>>,
}

/// Parameters for the creation of a managed bot from a keyboard button.
///
/// [The official docs](https://core.telegram.org/bots/api#keyboardbuttonrequestmanagedbot).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct KeyboardButtonRequestManagedBot {
    pub request_id: i32,
    pub suggested_name: Option<String>,
    pub suggested_username: Option<String>,
}

/// Prepared keyboard button that can be used by a Mini App user.
///
/// [The official docs](https://core.telegram.org/bots/api#preparedkeyboardbutton).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PreparedKeyboardButton {
    pub id: String,
}

/// Describes an inline message sent by a guest bot.
///
/// [The official docs](https://core.telegram.org/bots/api#sentguestmessage).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct SentGuestMessage {
    pub inline_message_id: String,
}

/// Audios displayed on a user's profile.
///
/// [The official docs](https://core.telegram.org/bots/api#userprofileaudios).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserProfileAudios {
    pub total_count: u32,
    pub audios: Vec<crate::types::Audio>,
}

/// Rating of a user based on Telegram Star spending.
///
/// [The official docs](https://core.telegram.org/bots/api#userrating).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct UserRating {
    pub level: i32,
    pub rating: u32,
    pub current_level_rating: u32,
    pub next_level_rating: Option<u32>,
}

/// Service message about the chat owner leaving the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatownerleft).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatOwnerLeft {
    pub new_owner: Option<User>,
}

/// Service message about an ownership change in the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatownerchanged).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatOwnerChanged {
    pub new_owner: User,
}

pub type AddedUserId = UserId;
