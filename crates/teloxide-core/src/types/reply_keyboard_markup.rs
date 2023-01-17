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
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
// FIXME: unoption bools?
pub struct KeyboardMarkup {
    /// Array of button rows, each represented by an Array of
    /// [`KeyboardButton`] objects
    ///
    /// [`KeyboardButton`]: crate::types::KeyboardButton
    pub keyboard: Vec<Vec<KeyboardButton>>,

    /// Requests clients to always show the keyboard when the regular keyboard
    /// is hidden. Defaults to `false`, in which case the custom keyboard
    /// can be hidden and opened with a keyboard icon.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_persistent: bool,

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

    /// The placeholder to be shown in the input field when the keyboard is
    /// active; 1-64 characters.
    pub input_field_placeholder: Option<String>,

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

impl KeyboardMarkup {
    pub fn new<K>(keyboard: K) -> Self
    where
        K: IntoIterator,
        K::Item: IntoIterator<Item = KeyboardButton>,
    {
        Self {
            keyboard: keyboard.into_iter().map(<_>::into_iter).map(<_>::collect).collect(),
            is_persistent: false,
            resize_keyboard: None,
            one_time_keyboard: None,
            input_field_placeholder: None,
            selective: None,
        }
    }

    pub fn append_row<R>(mut self, buttons: R) -> Self
    where
        R: IntoIterator<Item = KeyboardButton>,
    {
        self.keyboard.push(buttons.into_iter().collect());
        self
    }

    #[must_use]
    pub fn append_to_row(mut self, index: usize, button: KeyboardButton) -> Self {
        match self.keyboard.get_mut(index) {
            Some(buttons) => buttons.push(button),
            None => self.keyboard.push(vec![button]),
        };
        self
    }

    /// Sets [`is_persistent`] to `true`.
    ///
    /// [`is_persistent`]: KeyboardMarkup::is_persistent
    pub fn persistent(mut self) -> Self {
        self.is_persistent = true;
        self
    }

    pub fn resize_keyboard<T>(mut self, val: T) -> Self
    where
        T: Into<Option<bool>>,
    {
        self.resize_keyboard = val.into();
        self
    }

    pub fn one_time_keyboard<T>(mut self, val: T) -> Self
    where
        T: Into<Option<bool>>,
    {
        self.one_time_keyboard = val.into();
        self
    }

    pub fn input_field_placeholder<T>(mut self, val: T) -> Self
    where
        T: Into<Option<String>>,
    {
        self.input_field_placeholder = val.into();
        self
    }

    pub fn selective<T>(mut self, val: T) -> Self
    where
        T: Into<Option<bool>>,
    {
        self.selective = val.into();
        self
    }
}
