use serde::{Deserialize, Serialize};

use crate::types::KeyboardButton;

/// This object represents a [custom keyboard] with reply options (see
/// [Introduction to bots] for details and examples).
///
/// [The official docs](https://core.telegram.org/bots/api#replykeyboardmarkup).
///
/// [custom keyboard]: https://core.telegram.org/bots#keyboards
/// [Introduction to bots]: https://core.telegram.org/bots#keyboards
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of
    /// [`KeyboardButton`] objects
    ///
    /// [`KeyboardButton`]: crate::types::KeyboardButton
    pub keyboard: Vec<Vec<KeyboardButton>>,

    /// Requests clients to resize the keyboard vertically for optimal fit
    /// (e.g., make the keyboard smaller if there are just two rows of
    /// buttons). Defaults to `false`, in which case the custom keyboard is
    /// always of the same height as the app's standard keyboard.
    pub resize_keyboard: Option<bool>,

    /// Requests clients to hide the keyboard as soon as it's been used. The
    /// keyboard will still be available, but clients will automatically
    /// display the usual letter-keyboard in the chat – the user can press a
    /// special button in the input field to see the custom keyboard again.
    /// Defaults to `false`.
    pub one_time_keyboard: Option<bool>,

    /// Use this parameter if you want to show the keyboard to specific users
    /// only. Targets: 1) users that are `@mentioned` in the `text` of the
    /// [`Message`] object; 2) if the bot's message is a reply (has
    /// `reply_to_message_id`), sender of the original message.
    ///
    /// Example: A user requests to change the bot‘s language, bot replies to
    /// the request with a keyboard to select the new language. Other users
    /// in the group don’t see the keyboard.
    ///
    /// [`Message`]: crate::types::Message
    pub selective: Option<bool>,
}
