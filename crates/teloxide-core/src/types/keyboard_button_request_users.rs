use serde::{Deserialize, Serialize};

use crate::types::RequestId;

/// This object defines the criteria used to request a suitable users.
/// Information about the selected users will be shared with the bot when the
/// corresponding button is pressed. More about requesting users »
///
/// [More about requesting users »]: https://core.telegram.org/bots/features#chat-and-user-selection
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyboardButtonRequestUsers {
    /// Identifier of the request, which will be received back in the
    /// [`UsersShared`] object. Must be unique within the message.
    ///
    /// [`UsersShared`]: crate::types::UsersShared
    pub request_id: RequestId,

    /// Pass `true` to request a bot, pass `false` to request a regular user. If
    /// not specified, no additional restrictions are applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,

    /// Pass `true` to request a premium user, pass `false` to request a
    /// non-premium user. If not specified, no additional restrictions are
    /// applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,

    /// The maximum number of users to be selected; 1-10. Defaults to 1.
    #[serde(default = "one", skip_serializing_if = "is_one")]
    pub max_quantity: u8,

    /// Pass `true` to request the users' first and last names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,

    /// Pass `true` to request the users' username
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,

    /// Pass `true` to request the users' photos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

impl KeyboardButtonRequestUsers {
    /// Creates a new [`KeyboardButtonRequestUsers`].
    pub fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            user_is_bot: None,
            user_is_premium: None,
            max_quantity: 1,
            request_name: None,
            request_username: None,
            request_photo: None,
        }
    }

    /// Setter for `user_is_bot` field
    pub fn user_is_bot(mut self, value: bool) -> Self {
        self.user_is_bot = Some(value);
        self
    }

    /// Setter for `user_is_premium` field
    pub fn user_is_premium(mut self, value: bool) -> Self {
        self.user_is_premium = Some(value);
        self
    }

    /// Setter for `max_quantity` field, the value must be in the range 1..=10
    pub fn max_quantity(mut self, value: u8) -> Self {
        assert!((1..=10).contains(&value));

        self.max_quantity = value;
        self
    }

    /// Setter for `request_name` field
    pub fn request_name(mut self, value: bool) -> Self {
        self.request_name = Some(value);
        self
    }

    /// Setter for `request_username` field
    pub fn request_username(mut self, value: bool) -> Self {
        self.request_username = Some(value);
        self
    }

    /// Setter for `request_photo` field
    pub fn request_photo(mut self, value: bool) -> Self {
        self.request_photo = Some(value);
        self
    }
}

fn one() -> u8 {
    1
}

fn is_one(value: &u8) -> bool {
    *value == 1
}
