use crate::types::{CallbackGame, LoginUrl, SwitchInlineQueryChosenChat, True, WebAppInfo};
use serde::{Deserialize, Serialize};

/// This object represents one button of an inline keyboard.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinekeyboardbutton).
#[serde_with::skip_serializing_none]
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
    /// their privacy settings.
    Url(reqwest::Url),

    /// An HTTPS URL used to automatically authorize the user. Can be used as a
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
    /// only in private chats between a user and the bot. Not supported for
    /// messages sent on behalf of a Telegram Business account.
    ///
    /// [Web App]: https://core.telegram.org/bots/webapps
    /// [`AnswerWebAppQuery`]: crate::payloads::AnswerWebAppQuery
    WebApp(WebAppInfo),

    /// If set, pressing the button will prompt the user to select one of their
    /// chats, open that chat and insert the bot‘s username and the specified
    /// inline query in the input field. Can be empty, in which case just the
    /// bot’s username will be inserted. Not supported for messages sent on
    /// behalf of a Telegram Business account.
    ///
    /// Note: This offers an easy way for users to start using your bot in
    /// [inline mode] when they are currently in a private chat with it.
    /// Especially useful when combined with [switch_pm…] actions – in this
    /// case the user will be automatically returned to the chat they
    /// switched from, skipping the chat selection screen.
    ///
    /// [inline mode]: https://core.telegram.org/bots/inline
    /// [switch_pm…]: crate::payloads::AnswerInlineQuery
    SwitchInlineQuery(String),

    /// If set, pressing the button will insert the bot‘s username and the
    /// specified inline query in the current chat's input field.
    /// Can be empty, in which case only the bot’s username will be
    /// inserted.
    ///
    /// This offers a quick way for the user to open your bot in inline mode in
    /// the same chat – good for selecting something from multiple options. Not
    /// supported in channels and for messages sent on behalf of a Telegram
    /// Business account.
    SwitchInlineQueryCurrentChat(String),

    /// If set, pressing the button will prompt the user to select one of their
    /// chats of the specified type, open that chat and insert the bot's
    /// username and the specified inline query in the input field. Not
    /// supported for messages sent on behalf of a Telegram Business account.
    SwitchInlineQueryChosenChat(SwitchInlineQueryChosenChat),

    /// Description of the game that will be launched when the user presses the
    /// button.
    ///
    /// ## Note
    ///
    /// This type of button **must** always be the first button in the first
    /// row.
    CallbackGame(CallbackGame),

    /// Specify True, to send a [Pay button].
    ///
    /// ## Note
    ///
    /// This type of button **must** always be the first button in the first
    /// row.
    ///
    /// [Pay button]: https://core.telegram.org/bots/api#payments
    Pay(True),
}

impl InlineKeyboardButton {
    /// Creates a new `InlineKeyboardButton`.
    pub fn new<S>(text: S, kind: InlineKeyboardButtonKind) -> Self
    where
        S: Into<String>,
    {
        Self { text: text.into(), kind }
    }

    /// Constructor for `InlineKeyboardButton` with [`Url`] kind.
    ///
    /// [`Url`]: InlineKeyboardButtonKind::Url
    pub fn url<T>(text: T, url: reqwest::Url) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::Url(url))
    }

    /// Constructor for `InlineKeyboardButton` with [`LoginUrl`] kind.
    ///
    /// [`LoginUrl`]: InlineKeyboardButtonKind::LoginUrl
    pub fn login<T>(text: T, url: LoginUrl) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::LoginUrl(url))
    }

    /// Constructor for `InlineKeyboardButton` with [`CallbackData`] kind.
    ///
    /// [`CallbackData`]: InlineKeyboardButtonKind::CallbackData
    pub fn callback<T, C>(text: T, callback_data: C) -> Self
    where
        T: Into<String>,
        C: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::CallbackData(callback_data.into()))
    }

    /// Constructor for `InlineKeyboardButton` with [`WebApp`] kind.
    ///
    /// [`WebApp`]: InlineKeyboardButtonKind::WebApp
    pub fn web_app<T>(text: T, info: WebAppInfo) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::WebApp(info))
    }

    /// Constructor for `InlineKeyboardButton` with [`SwitchInlineQuery`] kind.
    ///
    /// [`SwitchInlineQuery`]: InlineKeyboardButtonKind::SwitchInlineQuery
    pub fn switch_inline_query<T, Q>(text: T, switch_inline_query: Q) -> Self
    where
        T: Into<String>,
        Q: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::SwitchInlineQuery(switch_inline_query.into()))
    }

    /// Constructor for `InlineKeyboardButton` with
    /// [`SwitchInlineQueryCurrentChat`] kind.
    ///
    /// [`SwitchInlineQueryCurrentChat`]: InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat
    pub fn switch_inline_query_current_chat<T, Q>(
        text: T,
        switch_inline_query_current_chat: Q,
    ) -> Self
    where
        T: Into<String>,
        Q: Into<String>,
    {
        Self::new(
            text,
            InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat(
                switch_inline_query_current_chat.into(),
            ),
        )
    }

    /// Constructor for `InlineKeyboardButton` with [`CallbackGame`] kind.
    ///
    /// [`CallbackGame`]: InlineKeyboardButtonKind::CallbackGame
    pub fn callback_game<T>(text: T, game: CallbackGame) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::CallbackGame(game))
    }

    /// Constructor for `InlineKeyboardButton` with [`Pay`] kind.
    ///
    /// [`Pay`]: InlineKeyboardButtonKind::Pay
    pub fn pay<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonKind::Pay(True))
    }
}
