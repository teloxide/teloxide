use crate::core::types::{InlineKeyboardButton, InlineKeyboardButtonKind};

pub struct InlineKeyboardButtonBuilder;

/// Build buttons
///
/// Example:
/// ```edition2018
/// use async_telegram_bot::keyboards::InlineKeyboardButtonBuilder;
///
/// fn main() {
///     let url_button = InlineKeyboardButtonBuilder::url(
///         "Text".to_string(),
///         "http://url.com".to_string(),
///     );
/// }
/// ```
impl InlineKeyboardButtonBuilder {
    pub fn url(text: String, url: String) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::Url(url),
        }
    }

    pub fn callback(text: String, callback_data: String)
        -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::CallbackData(callback_data),
        }
    }

    pub fn switch_inline_query(text: String, switch_inline_query: String)
        -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::SwitchInlineQuery(switch_inline_query)
        }
    }

    pub fn switch_inline_query_current_chat(
        text: String,
        switch_inline_query_current_chat: String
    ) -> InlineKeyboardButton {

        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat(
                switch_inline_query_current_chat
            )
        }
    }
}

