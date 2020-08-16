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
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}

impl LoginUrl {
    pub fn url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.url = val.into();
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

    pub fn request_write_access<S>(mut self, val: bool) -> Self {
        self.request_write_access = Some(val);
        self
    }
}
