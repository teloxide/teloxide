use serde::{Deserialize, Serialize};

use crate::types::True;

/// Upon receiving a message with this object, Telegram clients will display a
/// reply interface to the user (act as if the user has selected the bot‘s
/// message and tapped ’Reply').
///
/// This can be extremely useful if you want to create user-friendly
/// step-by-step interfaces without having to sacrifice [privacy mode].
///
/// [The official docs](https://core.telegram.org/bots/api#forcereply).
///
/// [privacy mode]: https://core.telegram.org/bots#privacy-mode
#[serde_with::skip_serializing_none]
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the
    /// bot‘s message and tapped ’Reply'.
    pub force_reply: True,

    /// The placeholder to be shown in the input field when the reply is active;
    /// 1-64 characters.
    pub input_field_placeholder: Option<String>,

    /// Use this parameter if you want to force reply from specific users only.
    /// Targets: 1) users that are `@mentioned` in the text of the
    /// [`Message`] object; 2) if the bot's message is a reply
    /// (has reply_to_message_id), sender of the original message.
    ///
    /// [`Message`]: crate::types::Message
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub selective: bool,
}

impl ForceReply {
    #[must_use]
    pub const fn new() -> Self {
        Self { force_reply: True, input_field_placeholder: None, selective: false }
    }

    pub fn input_field_placeholder<T>(self, val: T) -> Self
    where
        T: Into<Option<String>>,
    {
        Self { input_field_placeholder: val.into(), ..self }
    }

    /// Sets [`selective`] to `true`.
    ///
    /// [`selective`]: ForceReply::selective
    #[must_use]
    pub fn selective(self) -> Self {
        Self { selective: true, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "force_reply": true,
            "input_field_placeholder": "placeholder",
            "selective": false
        }
        "#;
        serde_json::from_str::<ForceReply>(data).unwrap();
    }
}
