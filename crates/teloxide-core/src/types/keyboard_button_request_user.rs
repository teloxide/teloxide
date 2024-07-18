use serde::{Deserialize, Serialize};

/// This object defines the criteria used to request a suitable users. The
/// identifiers of the selected users will be shared with the bot when the
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
    pub request_id: i32,

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
    #[serde(default = "de_max_quantity_default")]
    pub max_quantity: u8,
}

impl KeyboardButtonRequestUsers {
    /// Creates a new [`KeyboardButtonRequestUsers`].
    pub fn new(request_id: i32) -> Self {
        Self { request_id, user_is_bot: None, user_is_premium: None, max_quantity: 1 }
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
}

fn de_max_quantity_default() -> u8 {
    1
}
