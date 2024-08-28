use serde::{Deserialize, Serialize};

use crate::types::{Chat, User};

/// Represents either [`User`] or anonymous user ([`Chat`]) that acts on behalf
/// of the chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeAnonymousUser {
    User(User),
    Chat(Chat),
}

impl MaybeAnonymousUser {
    pub fn is_user(&self) -> bool {
        self.user().is_some()
    }

    pub fn is_chat(&self) -> bool {
        self.chat().is_some()
    }

    #[must_use]
    pub fn chat(&self) -> Option<&Chat> {
        match self {
            Self::Chat(chat) => Some(chat),
            _ => None,
        }
    }

    #[must_use]
    pub fn user(&self) -> Option<&User> {
        match self {
            Self::User(user) => Some(user),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_de() {
        let json = r#"{
            "id": 42,
            "is_bot": false,
            "first_name": "blah"
        }"#;

        let user: MaybeAnonymousUser = serde_json::from_str(json).unwrap();

        assert!(user.user().is_some());
    }

    #[test]
    fn chat_de() {
        let json = r#"{
            "id": -1001160242915,
            "title": "a",
            "type": "group"
        }"#;

        let chat: MaybeAnonymousUser = serde_json::from_str(json).unwrap();

        assert!(chat.chat().is_some());
    }
}
