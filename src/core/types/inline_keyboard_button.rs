/// This object represents one button of an inline keyboard.
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,

    #[serde(flatten)]
    pub kind: InlineKeyboardButtonKind,
}

#[derive(
    Debug, Clone, PartialEq, PartialOrd, Serialize, Eq, Hash, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum InlineKeyboardButtonKind {
    /// HTTP or tg:// url to be opened when button is pressed
    Url(String),
    /// Data to be sent in a callback query to the bot when button is pressed,
    /// 1-64 bytes
    CallbackData(String),
    /// If set, pressing the button will prompt the user to select one of their
    /// chats, open that chat and insert the bot‘s username and the specified
    /// inline query in the input field. Can be empty, in which case just the
    /// bot’s username will be inserted.
    ///
    /// Note: This offers an easy way for users to start using your bot in
    /// inline mode when they are currently in a private chat with it.
    /// Especially useful when combined with switch_pm… actions – in this case
    /// the user will be automatically returned to the chat they switched from,
    /// skipping the chat selection screen.
    SwitchInlineQuery(String),
    /// Optional. If set, pressing the button will insert the bot‘s username
    /// and the specified inline query in the current chat's input field.
    /// Can be empty, in which case only the bot’s username will be
    /// inserted.
    ///
    ///This offers a quick way for the user to open your bot in inline mode in
    /// the same chat – good for selecting something from multiple options.
    SwitchInlineQueryCurrentChat(String),
    /* CallbackGame(CallbackGame), TODO: разобраться, что с этим делать
     * TODO: add LoginUrl, pay */
}

/// Build buttons
///
/// Example:
/// ```edition2018
/// use async_telegram_bot::core::types::InlineKeyboardButton;
///
/// fn main() {
///     let url_button = InlineKeyboardButton::url(
///         "Text".to_string(),
///         "http://url.com".to_string(),
///     );
/// }
/// ```
impl InlineKeyboardButton {
    pub fn url(text: String, url: String) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::Url(url),
        }
    }

    pub fn callback(
        text: String,
        callback_data: String,
    ) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::CallbackData(callback_data),
        }
    }

    pub fn switch_inline_query(
        text: String,
        switch_inline_query: String,
    ) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::SwitchInlineQuery(
                switch_inline_query,
            ),
        }
    }

    pub fn switch_inline_query_current_chat(
        text: String,
        switch_inline_query_current_chat: String,
    ) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat(
                switch_inline_query_current_chat,
            ),
        }
    }
}
