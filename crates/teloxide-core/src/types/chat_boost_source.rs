use serde::{Deserialize, Serialize};

use crate::types::{MessageId, User};

/// This object describes the source of a chat boost.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[serde(tag = "source")]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveaway(ChatBoostSourceGiveaway),
}

/// The boost was obtained by subscribing to Telegram Premium or by gifting a
/// Telegram Premium subscription to another user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatBoostSourcePremium {
    /// User that boosted the chat.
    pub user: User,
}

/// The boost was obtained by the creation of Telegram Premium gift codes to
/// boost a chat. Each such code boosts the chat 4 times for the duration of the
/// corresponding Telegram Premium subscription.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatBoostSourceGiftCode {
    /// User for which the gift code was created.
    pub user: User,
}

/// The boost was obtained by the creation of a Telegram Premium giveaway. This
/// boosts the chat 4 times for the duration of the corresponding Telegram
/// Premium subscription.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatBoostSourceGiveaway {
    /// Identifier of a message in the chat with the giveaway; the message could
    /// have been deleted already. May be 0 if the message isn't sent yet.
    #[serde(flatten, with = "crate::types::prefix_giveaway_message_id")]
    #[cfg_attr(test, schemars(with = "i32"))]
    pub giveaway_message_id: MessageId,

    /// User that won the prize in the giveaway if any.
    pub user: Option<User>,

    /// The number of Telegram Stars to be split between giveaway winners
    pub prize_star_count: Option<u32>,

    /// `true`, if the giveaway was completed, but there was no user to win the
    /// prize.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_unclaimed: bool,
}

impl ChatBoostSource {
    #[must_use]
    pub fn user(&self) -> Option<&User> {
        Some(match &self {
            Self::Premium(premium) => &premium.user,
            Self::GiftCode(gift_code) => &gift_code.user,
            Self::Giveaway(giveaway) => return giveaway.user.as_ref(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_premium() {
        let data = r#"
        {
            "source": "premium",
            "user": {
                "id": 1459074222,
                "is_bot": false,
                "first_name": "shadowchain",
                "username": "shdwchn10",
                "language_code": "en",
                "is_premium": true
            }
        }
        "#;
        serde_json::from_str::<ChatBoostSource>(data).unwrap();
    }

    #[test]
    fn deserialize_gift_code() {
        let data = r#"
        {
            "source": "gift_code",
            "user": {
                "id": 1459074222,
                "is_bot": false,
                "first_name": "shadowchain",
                "username": "shdwchn10",
                "language_code": "en",
                "is_premium": false
            }
        }
        "#;
        serde_json::from_str::<ChatBoostSource>(data).unwrap();
    }

    #[test]
    fn deserialize_giveaway() {
        let data = r#"
        {
            "source": "giveaway",
            "giveaway_message_id": 420,
            "user": {
                "id": 1459074222,
                "is_bot": false,
                "first_name": "shadowchain",
                "username": "shdwchn10",
                "language_code": "en",
                "is_premium": false
            }
        }
        "#;
        serde_json::from_str::<ChatBoostSource>(data).unwrap();
    }
}
