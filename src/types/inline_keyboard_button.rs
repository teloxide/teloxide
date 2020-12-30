use crate::types::{CallbackGame, LoginUrl};
use serde::{Deserialize, Serialize};

/// This object represents one button of an inline keyboard.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinekeyboardbutton).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button.
    pub text: String,

    #[serde(flatten)]
    pub kind: InlineKeyboardButtonKind,
}

impl InlineKeyboardButton {
    pub fn new<S>(text: S, kind: InlineKeyboardButtonKind) -> Self
    where
        S: Into<String>,
    {
        Self {
            text: text.into(),
            kind,
        }
    }

    pub fn text<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.text = val.into();
        self
    }

    pub fn kind(mut self, val: InlineKeyboardButtonKind) -> Self {
        self.kind = val;
        self
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InlineKeyboardButtonKind {
    /// HTTP or tg:// url to be opened when button is pressed.
    Url(String),

    /// An HTTP URL used to automatically authorize the user. Can be used as a
    /// replacement for the [Telegram Login Widget]().
    ///
    /// [Telegram Login Widget]: https://core.telegram.org/widgets/login
    LoginUrl(LoginUrl),

    /// Data to be sent in a [`CallbackQuery`] to the bot when button is
    /// pressed, 1-64 bytes.
    ///
    /// [`CallbackQuery`]: crate::types::CallbackQuery
    CallbackData(String),

    /// If set, pressing the button will prompt the user to select one of their
    /// chats, open that chat and insert the bot‘s username and the specified
    /// inline query in the input field. Can be empty, in which case just the
    /// bot’s username will be inserted.
    ///
    /// Note: This offers an easy way for users to start using your bot in
    /// [inline mode] when they are currently in a private chat with it.
    /// Especially useful when combined with [switch_pm…] actions – in this
    /// case the user will be automatically returned to the chat they
    /// switched from, skipping the chat selection screen.
    ///
    /// [inline mode]: https://core.telegram.org/bots/inline
    /// [switch_pm…]: https://core.telegram.org/bots/api#answerinlinequery
    SwitchInlineQuery(String),

    /// If set, pressing the button will insert the bot‘s username and the
    /// specified inline query in the current chat's input field.
    /// Can be empty, in which case only the bot’s username will be
    /// inserted.
    ///
    ///This offers a quick way for the user to open your bot in inline mode in
    /// the same chat – good for selecting something from multiple options.
    SwitchInlineQueryCurrentChat(String),

    /// Description of the game that will be launched when the user presses the
    /// button.
    ///
    /// ## Note
    /// This type of button **must** always be the first button in the first
    /// row.
    CallbackGame(CallbackGame),

    /// Specify True, to send a [Pay button].
    ///
    /// ## Note
    /// This type of button **must** always be the first button in the first
    /// row.
    ///
    /// [Pay button]: https://core.telegram.org/bots/api#payments
    Pay(bool),
}

/// Build buttons.
///
/// # Examples
/// ```
/// use teloxide_core::types::InlineKeyboardButton;
///
/// let url_button = InlineKeyboardButton::url("Text".to_string(), "http://url.com".to_string());
/// ```
impl InlineKeyboardButton {
    pub fn url(text: String, url: String) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::Url(url),
        }
    }

    pub fn callback(text: String, callback_data: String) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::CallbackData(callback_data),
        }
    }

    pub fn switch_inline_query(text: String, switch_inline_query: String) -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            kind: InlineKeyboardButtonKind::SwitchInlineQuery(switch_inline_query),
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
