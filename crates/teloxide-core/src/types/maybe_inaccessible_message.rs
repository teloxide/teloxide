use serde::{Deserialize, Serialize};

use crate::types::{Chat, InaccessibleMessage, Message, MessageId};

/// This object describes a message that can be inaccessible to the bot. It can
/// be one of:
/// - [Message]
/// - [InaccessibleMessage]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    Inaccessible(InaccessibleMessage),
    Regular(Box<Message>),
}

impl MaybeInaccessibleMessage {
    pub fn id(&self) -> MessageId {
        match self {
            Self::Inaccessible(i_message) => i_message.message_id,
            Self::Regular(message) => message.id,
        }
    }

    #[must_use]
    pub fn regular_message(&self) -> Option<&Message> {
        match self {
            Self::Regular(message) => Some(message),
            Self::Inaccessible(_) => None,
        }
    }

    #[must_use]
    pub fn chat(&self) -> &Chat {
        match self {
            Self::Regular(message) => &message.chat,
            Self::Inaccessible(i_message) => &i_message.chat,
        }
    }
}

impl<'de> Deserialize<'de> for MaybeInaccessibleMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let message: Message = Message::deserialize(deserializer)?;

        // Thank you, TBA 7.0 authors!
        if message.date.timestamp() == 0 {
            return Ok(MaybeInaccessibleMessage::Inaccessible(InaccessibleMessage {
                chat: message.chat,
                message_id: message.id,
            }));
        }
        Ok(MaybeInaccessibleMessage::Regular(Box::new(message)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inaccessible_message() {
        let json = r#"{
            "chat": {
                "id": 42,
                "first_name": "Вадим Игоревич",
                "last_name": "Сырцев",
                "username": "syrtcevvi",
                "type": "private"
            },
            "message_id": 4,
            "date": 0
        }"#;

        let inaccessible_message = serde_json::from_str::<MaybeInaccessibleMessage>(json);
        assert!(inaccessible_message.is_ok());
        assert!(matches!(inaccessible_message.unwrap(), MaybeInaccessibleMessage::Inaccessible(_)));
    }

    #[test]
    fn test_regular_message() {
        let json = r#"{
            "chat": {
                "id": 42,
                "first_name": "Вадим Игоревич",
                "last_name": "Сырцев",
                "username": "syrtcevvi",
                "type": "private"
            },
            "message_id": 4,
            "date": 1
        }"#;

        let regular_message = serde_json::from_str::<MaybeInaccessibleMessage>(json);
        assert!(regular_message.is_ok());
        assert!(matches!(regular_message.unwrap(), MaybeInaccessibleMessage::Regular(_)));
    }
}
