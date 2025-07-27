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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
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
/// use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardMarkup};
///
/// let url = url::Url::parse("https://example.com").unwrap();
/// let url_button = InlineKeyboardButton::url("text".to_string(), url);
/// let keyboard = InlineKeyboardMarkup::default().append_row(vec![url_button]);
/// ```
impl InlineKeyboardMarkup {
    pub fn new<I>(inline_keyboard: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoIterator<Item = InlineKeyboardButton>,
    {
        Self {
            inline_keyboard: inline_keyboard
                .into_iter()
                .map(<_>::into_iter)
                .map(<_>::collect)
                .collect(),
        }
    }

    pub fn inline_keyboard<I>(mut self, val: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoIterator<Item = InlineKeyboardButton>,
    {
        self.inline_keyboard = val.into_iter().map(<_>::into_iter).map(<_>::collect).collect();
        self
    }

    pub fn append_row<R>(mut self, buttons: R) -> Self
    where
        R: IntoIterator<Item = InlineKeyboardButton>,
    {
        self.inline_keyboard.push(buttons.into_iter().collect());
        self
    }

    #[must_use]
    pub fn append_to_row(mut self, index: usize, button: InlineKeyboardButton) -> Self {
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

    fn url(n: u32) -> reqwest::Url {
        reqwest::Url::parse(&format!("https://example.com/{n}")).unwrap()
    }

    #[test]
    fn append_row() {
        let button1 = InlineKeyboardButton::url("text 1".to_string(), url(1));
        let button2 = InlineKeyboardButton::url("text 2".to_string(), url(2));

        let markup =
            InlineKeyboardMarkup::default().append_row(vec![button1.clone(), button2.clone()]);

        let expected = InlineKeyboardMarkup { inline_keyboard: vec![vec![button1, button2]] };

        assert_eq!(markup, expected);
    }

    #[test]
    fn append_to_row_existent_row() {
        let button1 = InlineKeyboardButton::url("text 1".to_string(), url(1));
        let button2 = InlineKeyboardButton::url("text 2".to_string(), url(2));

        let markup = InlineKeyboardMarkup::default()
            .append_row(vec![button1.clone()])
            .append_to_row(0, button2.clone());

        let expected = InlineKeyboardMarkup { inline_keyboard: vec![vec![button1, button2]] };

        assert_eq!(markup, expected);
    }

    #[test]
    fn append_to_row_nonexistent_row() {
        let button1 = InlineKeyboardButton::url("text 1".to_string(), url(1));
        let button2 = InlineKeyboardButton::url("text 2".to_string(), url(2));

        let markup = InlineKeyboardMarkup::default()
            .append_row(vec![button1.clone()])
            .append_to_row(1, button2.clone());

        let expected = InlineKeyboardMarkup { inline_keyboard: vec![vec![button1], vec![button2]] };

        assert_eq!(markup, expected);
    }
}
