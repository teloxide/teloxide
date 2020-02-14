use serde::{Deserialize, Serialize};

use crate::types::InlineKeyboardButton;

/// This object represents an [inline keyboard] that appears right next to the
/// message it belongs to.
///
/// *Note*: This will only work in Telegram versions released after 9 April,
/// 2016. Older clients will display unsupported message.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinekeyboardmarkup).
///
/// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an array of
    /// [`InlineKeyboardButton`] objects.
    ///
    /// [`InlineKeyboardButton`]: crate::types::InlineKeyboardButton
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// Build `InlineKeyboardMarkup`.
///
/// # Examples
/// ```
/// use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
///
/// let url_button = InlineKeyboardButton::url(
///     "text".to_string(),
///     "http://url.com".to_string(),
/// );
/// let keyboard = InlineKeyboardMarkup::default().append_row(vec![url_button]);
/// ```
impl InlineKeyboardMarkup {
    pub fn append_row(mut self, buttons: Vec<InlineKeyboardButton>) -> Self {
        self.inline_keyboard.push(buttons);
        self
    }

    pub fn append_to_row(
        mut self,
        button: InlineKeyboardButton,
        index: usize,
    ) -> Self {
        match self.inline_keyboard.get_mut(index) {
            Some(buttons) => buttons.push(button),
            None => self.inline_keyboard.push(vec![button]),
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_row() {
        let button1 = InlineKeyboardButton::url(
            "text 1".to_string(),
            "url 1".to_string(),
        );
        let button2 = InlineKeyboardButton::url(
            "text 2".to_string(),
            "url 2".to_string(),
        );

        let markup = InlineKeyboardMarkup::default()
            .append_row(vec![button1.clone(), button2.clone()]);

        let expected = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![button1, button2]],
        };

        assert_eq!(markup, expected);
    }

    #[test]
    fn append_to_row_existent_row() {
        let button1 = InlineKeyboardButton::url(
            "text 1".to_string(),
            "url 1".to_string(),
        );
        let button2 = InlineKeyboardButton::url(
            "text 2".to_string(),
            "url 2".to_string(),
        );

        let markup = InlineKeyboardMarkup::default()
            .append_row(vec![button1.clone()])
            .append_to_row(button2.clone(), 0);

        let expected = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![button1, button2]],
        };

        assert_eq!(markup, expected);
    }

    #[test]
    fn append_to_row_nonexistent_row() {
        let button1 = InlineKeyboardButton::url(
            "text 1".to_string(),
            "url 1".to_string(),
        );
        let button2 = InlineKeyboardButton::url(
            "text 2".to_string(),
            "url 2".to_string(),
        );

        let markup = InlineKeyboardMarkup::default()
            .append_row(vec![button1.clone()])
            .append_to_row(button2.clone(), 1);

        let expected = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![button1], vec![button2]],
        };

        assert_eq!(markup, expected);
    }
}
