// FIXME: rename module (s/reply_//)
use serde::{Deserialize, Serialize};

use crate::types::KeyboardButton;

/// This object represents a [custom keyboard] with reply options (see
/// [Introduction to bots] for details and examples).
///
/// [The official docs](https://core.telegram.org/bots/api#replykeyboardmarkup).
///
/// [custom keyboard]: https://core.telegram.org/bots#keyboards
/// [Introduction to bots]: https://core.telegram.org/bots#keyboards
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct KeyboardMarkup {
    /// Array of button rows, each represented by an Array of
    /// [`KeyboardButton`] objects
    ///
    /// [`KeyboardButton`]: crate::types::KeyboardButton
    pub keyboard: Vec<Vec<KeyboardButton>>,

    /// Requests clients to always show the keyboard when the regular keyboard
    /// is hidden. Defaults to `false`, in which case the custom keyboard
    /// can be hidden and opened with a keyboard icon.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_persistent: bool,

    /// Requests clients to resize the keyboard vertically for optimal fit
    /// (e.g., make the keyboard smaller if there are just two rows of
    /// buttons). Defaults to `false`, in which case the custom keyboard is
    /// always of the same height as the app's standard keyboard.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub resize_keyboard: bool,

    /// Requests clients to hide the keyboard as soon as it's been used. The
    /// keyboard will still be available, but clients will automatically
    /// display the usual letter-keyboard in the chat – the user can press a
    /// special button in the input field to see the custom keyboard again.
    /// Defaults to `false`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub one_time_keyboard: bool,

    /// The placeholder to be shown in the input field when the keyboard is
    /// active; 1-64 characters.
    #[serde(default, skip_serializing_if = "str::is_empty")]
    pub input_field_placeholder: String,

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
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub selective: bool,
}

impl KeyboardMarkup {
    // FIXME: Re-think the interface of building keyboard markups
    pub fn new<K>(keyboard: K) -> Self
    where
        K: IntoIterator,
        K::Item: IntoIterator<Item = KeyboardButton>,
    {
        Self {
            keyboard: keyboard.into_iter().map(<_>::into_iter).map(<_>::collect).collect(),
            is_persistent: false,
            resize_keyboard: false,
            one_time_keyboard: false,
            input_field_placeholder: String::new(),
            selective: false,
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
    pub fn persistent(self) -> Self {
        Self { is_persistent: true, ..self }
    }

    /// Sets [`resize_keyboard`] to `true`.
    ///
    /// [`resize_keyboard`]: KeyboardMarkup::resize_keyboard
    pub fn resize_keyboard(self) -> Self {
        Self { resize_keyboard: true, ..self }
    }

    /// Sets [`one_time_keyboard`] to `true`.
    ///
    /// [`one_time_keyboard`]: KeyboardMarkup::one_time_keyboard
    pub fn one_time_keyboard(self) -> Self {
        Self { one_time_keyboard: true, ..self }
    }

    // FIXME: document
    pub fn input_field_placeholder<T>(self, val: T) -> Self
    where
        T: Into<String>,
    {
        Self { input_field_placeholder: val.into(), ..self }
    }

    /// Sets [`selective`] to `true`.
    ///
    /// [`selective`]: KeyboardMarkup::selective
    pub fn selective(self) -> Self {
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
            "keyboard": [[{"text": "a"}, {"text": "b"}], [{"text": "c"}, {"text": "d"}]],
            "input_field_placeholder": "",
            "is_persistent": true,
            "one_time_keyboard": false,
            "resize_keyboard": true,
            "selective": false
        }
        "#;
        assert!(serde_json::from_str::<KeyboardMarkup>(data).is_ok())
    }
    #[test]
    fn serialize() {
        let keyboard = vec![vec![
            KeyboardButton::new("a"),
            KeyboardButton::new("b"),
            KeyboardButton::new("c"),
            KeyboardButton::new("d"),
        ]];
        let keyboard_markup = KeyboardMarkup::new(keyboard)
            .persistent()
            .resize_keyboard()
            .selective()
            .one_time_keyboard();
        let expected = r#"{"keyboard":[[{"text":"a"},{"text":"b"},{"text":"c"},{"text":"d"}]],"is_persistent":true,"resize_keyboard":true,"one_time_keyboard":true,"selective":true}"#;

        assert!(serde_json::ser::to_string(&keyboard_markup).is_ok_and(|s| s.eq(expected)));
    }
}
