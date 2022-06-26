use crate::types::{CallbackGame, LoginUrl, True, WebAppInfo};
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InlineKeyboardButtonKind {
    /// HTTP or `tg://` url to be opened when button is pressed.
    ///
    /// Links in the form of `tg://user?id=<user_id>` can be used to mention a
    /// user by their ID without using a username, if this is allowed by
    /// their privacy settings. This will only work in Telegram versions
    /// released after December 7, 2021. Older clients will display _unsupported
    /// message_.
    Url(reqwest::Url),

    /// An HTTP URL used to automatically authorize the user. Can be used as a
    /// replacement for the [Telegram Login Widget].
    ///
    /// [Telegram Login Widget]: https://core.telegram.org/widgets/login
    LoginUrl(LoginUrl),

    /// Data to be sent in a [`CallbackQuery`] to the bot when button is
    /// pressed, 1-64 bytes.
    ///
    /// [`CallbackQuery`]: crate::types::CallbackQuery
    CallbackData(String),

    /// Description of the [Web App] that will be launched when the user presses
    /// the button. The Web App will be able to send an arbitrary message on
    /// behalf of the user using the method [`AnswerWebAppQuery`]. Available
    /// only in private chats between a user and the bot.
    ///
    /// [Web App]: https://core.telegram.org/bots/webapps
    /// [`AnswerWebAppQuery`]: crate::payloads::AnswerWebAppQuery
    WebApp(WebAppInfo),

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
    Pay(True),
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

    pub fn url<T>(text: T, url: reqwest::Url) -> InlineKeyboardButton
    where
        T: Into<String>,
    {
        InlineKeyboardButton {
            text: text.into(),
            kind: InlineKeyboardButtonKind::Url(url),
        }
    }

    pub fn login<T>(text: T, url: LoginUrl) -> InlineKeyboardButton
    where
        T: Into<String>,
    {
        InlineKeyboardButton {
            text: text.into(),
            kind: InlineKeyboardButtonKind::LoginUrl(url),
        }
    }

    pub fn callback<T, C>(text: T, callback_data: C) -> InlineKeyboardButton
    where
        T: Into<String>,
        C: Into<String>,
    {
        InlineKeyboardButton {
            text: text.into(),
            kind: InlineKeyboardButtonKind::CallbackData(callback_data.into()),
        }
    }

    pub fn web_app<T>(text: T, info: WebAppInfo) -> InlineKeyboardButton
    where
        T: Into<String>,
    {
        InlineKeyboardButton {
            text: text.into(),
            kind: InlineKeyboardButtonKind::WebApp(info),
        }
    }

    pub fn switch_inline_query<T, Q>(text: T, switch_inline_query: Q) -> InlineKeyboardButton
    where
        T: Into<String>,
        Q: Into<String>,
    {
        InlineKeyboardButton {
            text: text.into(),
            kind: InlineKeyboardButtonKind::SwitchInlineQuery(switch_inline_query.into()),
        }
    }

    pub fn switch_inline_query_current_chat<T, Q>(
        text: T,
        switch_inline_query_current_chat: Q,
    ) -> InlineKeyboardButton
    where
        T: Into<String>,
        Q: Into<String>,
    {
        InlineKeyboardButton {
            text: text.into(),
            kind: InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat(
                switch_inline_query_current_chat.into(),
            ),
        }
    }

    pub fn callback_game<T>(text: T, game: CallbackGame) -> InlineKeyboardButton
    where
        T: Into<String>,
    {
        InlineKeyboardButton {
            text: text.into(),
            kind: InlineKeyboardButtonKind::CallbackGame(game),
        }
    }

    pub fn pay<T>(text: T) -> InlineKeyboardButton
    where
        T: Into<String>,
    {
        InlineKeyboardButton {
            text: text.into(),
            kind: InlineKeyboardButtonKind::Pay(True),
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
