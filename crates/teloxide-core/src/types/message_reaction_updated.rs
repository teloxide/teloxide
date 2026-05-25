use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::types::{Chat, MaybeAnonymousUser, MessageId, ReactionType, User};

/// This object represents a change of a reaction on a message performed by a
/// user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct MessageReactionUpdated {
    /// The chat containing the message the user reacted to
    pub chat: Chat,

    /// Unique identifier of the message inside the chat
    #[serde(flatten)]
    pub message_id: MessageId,

    /// The [`MaybeAnonymousUser::User`] that changed the reaction, if the user
    /// isn't anonymous or the [`MaybeAnonymousUser::Chat`] on behalf of
    /// which the reaction was changed, if the user is anonymous
    #[serde(deserialize_with = "deserialize_actor", flatten)]
    pub actor: MaybeAnonymousUser,

    /// Date of the change in Unix time
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub date: DateTime<Utc>,

    /// Previous list of reaction types that were set by the user
    pub old_reaction: Vec<ReactionType>,

    /// New list of reaction types that have been set by the user
    pub new_reaction: Vec<ReactionType>,
}

impl MessageReactionUpdated {
    #[must_use]
    pub fn chat(&self) -> Option<&Chat> {
        self.actor.chat()
    }

    #[must_use]
    pub fn user(&self) -> Option<&User> {
        self.actor.user()
    }
}

#[derive(Deserialize)]
struct ActorDe {
    /// The user that changed the reaction, if the user isn't anonymous
    user: Option<User>,
    /// The chat on behalf of which the reaction was changed, if the user is
    /// anonymous
    actor_chat: Option<Chat>,
}

fn deserialize_actor<'d, D: Deserializer<'d>>(d: D) -> Result<MaybeAnonymousUser, D::Error> {
    let ActorDe { user, actor_chat } = ActorDe::deserialize(d)?;

    Ok(actor_chat.map(MaybeAnonymousUser::Chat).or(user.map(MaybeAnonymousUser::User)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_user() {
        let data = r#"
        {
            "chat": {
                "id": -1002184233434,
                "title": "Test",
                "type": "supergroup"
            },
            "message_id": 35,
            "user": {
                "id": 1459074222,
                "is_bot": false,
                "first_name": "shadowchain",
                "username": "shdwchn10",
                "language_code": "en",
                "is_premium": true
            },
            "date": 1721306082,
            "old_reaction": [],
            "new_reaction": [
                {
                    "type": "emoji",
                    "emoji": "🌭"
                }
            ]
        }
        "#;
        let message_reaction_update = serde_json::from_str::<MessageReactionUpdated>(data).unwrap();

        assert!(message_reaction_update.actor.is_user());
    }

    #[test]
    fn deserialize_chat() {
        let data = r#"{
            "chat": {
                "id": -1002199793788,
                "title": "тест",
                "type": "supergroup"
            },
            "message_id": 2,
            "actor_chat": {
                "id": -1002199793788,
                "title": "тест",
                "type": "supergroup"
            },
            "date": 1723798597,
            "old_reaction": [
                {
                    "type": "emoji",
                    "emoji": "❤"
                }
            ],
            "new_reaction": []
        }"#;

        let message_reaction_update = serde_json::from_str::<MessageReactionUpdated>(data).unwrap();

        assert!(message_reaction_update.actor.is_chat())
    }
}
