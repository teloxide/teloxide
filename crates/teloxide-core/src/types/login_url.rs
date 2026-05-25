use serde::{Deserialize, Serialize};

/// This object represents a parameter of the inline keyboard button used to
/// automatically authorize a user.
///
/// Serves as a great replacement for the [Telegram Login Widget] when the user
/// is coming from Telegram. All the user needs to do is tap/click a button and
/// confirm that they want to log in:
///
/// <div align="center">
///     <img src="https://core.telegram.org/file/811140015/1734/8VZFkwWXalM.97872/6127fa62d8a0bf2b3c" width=300 />
/// </div>
///
/// [Telegram Login Widget]: https://core.telegram.org/widgets/login
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct LoginUrl {
    /// An HTTPS URL to be opened with user authorization data added to the
    /// query string when the button is pressed. If the user refuses to
    /// provide authorization data, the original URL without information
    /// about the user will be opened. The data added is the same as
    /// described in [Receiving authorization data].
    ///
    /// [Receiving authorization data]: https://core.telegram.org/widgets/login#receiving-authorization-data
    ///
    /// NOTE: You must always check the hash of the received data to verify the
    /// authentication and the integrity of the data as described in [Checking
    /// authorization].
    ///
    /// [Checking authorization]: https://core.telegram.org/widgets/login#checking-authorization
    pub url: reqwest::Url,
    /// New text of the button in forwarded messages.
    pub forward_text: Option<String>,
    /// Username of a bot, which will be used for user authorization. See
    /// [Setting up a bot] for more details. If not specified, the current bot's
    /// username will be assumed. The url's domain must be the same as the
    /// domain linked with the bot. See [Linking your domain to the bot] for
    /// more details.
    ///
    /// [Setting up a bot]: https://core.telegram.org/widgets/login#setting-up-a-bot
    /// [Linking your domain to the bot]: https://core.telegram.org/widgets/login#linking-your-domain-to-the-bot
    pub bot_username: Option<String>,
    /// Pass `true` to request the permission for your bot to send messages to
    /// the user.
    pub request_write_access: Option<bool>,
}

impl LoginUrl {
    #[must_use]
    pub fn url(mut self, val: reqwest::Url) -> Self {
        self.url = val;
        self
    }

    pub fn forward_text<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.forward_text = Some(val.into());
        self
    }

    pub fn bot_username<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.bot_username = Some(val.into());
        self
    }

    #[must_use]
    pub fn request_write_access(mut self, val: bool) -> Self {
        self.request_write_access = Some(val);
        self
    }
}
