use serde::{Deserialize, Serialize};

use crate::types::True;

/// Upon receiving a message with this object, Telegram clients will remove the
/// current custom keyboard and display the default letter-keyboard.
///
/// By default, custom keyboards are displayed until a new keyboard is sent by a
/// bot. An exception is made for one-time keyboards that are hidden immediately
/// after the user presses a button (see [`KeyboardMarkup`]).
///
/// [The official docs](https://core.telegram.org/bots/api#replykeyboardremove).
///
/// [`KeyboardMarkup`]: crate::types::KeyboardMarkup
#[serde_with::skip_serializing_none]
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[derive(Eq, Hash, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct KeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able
    /// to summon this keyboard; if you want to hide the keyboard from sight
    /// but keep it accessible, use one_time_keyboard in
    /// [`KeyboardMarkup`]).
    ///
    /// [`KeyboardMarkup`]: crate::types::KeyboardMarkup
    pub remove_keyboard: True,

    /// Use this parameter if you want to remove the keyboard for specific
    /// users only. Targets: 1) users that are `@mentioned` in the `text` of
    /// the [`Message`] object; 2) if the bot's message is a reply (has
    /// `reply_to_message_id`), sender of the original message.
    ///
    /// Example: A user votes in a poll, bot returns confirmation message in
    /// reply to the vote and removes the keyboard for that user, while still
    /// showing the keyboard with poll options to users who haven't voted yet.
    ///
    /// [`Message`]: crate::types::Message
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub selective: bool,
}

impl KeyboardRemove {
    #[must_use]
    pub const fn new() -> Self {
        Self { remove_keyboard: True, selective: false }
    }

    /// Sets [`selective`] to `true`.
    ///
    /// [`selective`]: KeyboardRemove::selective
    #[must_use]
    pub const fn selective(self) -> Self {
        Self { selective: true, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "remove_keyboard": true,
            "selective": false
        }
        "#;
        serde_json::from_str::<KeyboardRemove>(data).unwrap();
    }
}
