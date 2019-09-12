use crate::core::types::{InlineKeyboardMarkup, InlineKeyboardButton};

pub struct InlineKeyboardMarkupBuilder {
    keyboard: InlineKeyboardMarkup,
}

/// Builder for [`InlineKeyboardMarkup`]
///
/// Example:
/// ```edition2018
/// use async_telegram_bot::keyboards;
///
/// fn main() {
///     let url_button = keyboards::InlineKeyboardButtonBuilder::url(
///         "text".to_string(),
///         "http://url.com".to_string()
///     );
///     let keyboard = keyboards::InlineKeyboardMarkupBuilder::new()
///         .row(vec![url_button])
///         .build();
/// }
/// ```
impl InlineKeyboardMarkupBuilder {
    pub fn new() -> Self {
        Self {
            keyboard: InlineKeyboardMarkup {
                inline_keyboard: vec![]
            }
        }
    }

    pub fn row(mut self, buttons: Vec<InlineKeyboardButton>) -> Self {
        self.keyboard.inline_keyboard.push(buttons);
        self
    }

    pub fn append_to_row(mut self, button: InlineKeyboardButton, index: usize)
        -> Self {
        match self.keyboard.inline_keyboard.get_mut(index) {
            Some(buttons) => buttons.push(button),
            None => self.keyboard.inline_keyboard.push(vec![button])
        };
        self
    }

    pub fn build(self) -> InlineKeyboardMarkup {
        self.keyboard
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboards::InlineKeyboardButtonBuilder;

    #[test]
    fn test_row() {
        let btn = InlineKeyboardButtonBuilder::url(
            "text".to_string(),
            "http://url".to_string(),
        );
        let kb = InlineKeyboardMarkupBuilder::new()
            .row(vec![btn.clone()])
            .build();
        let expected = InlineKeyboardMarkup {
            inline_keyboard: vec![vec![btn.clone()]],
        };
        assert_eq!(kb, expected);
    }
}
