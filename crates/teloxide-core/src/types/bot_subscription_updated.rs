use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::User;

/// Contains information about a change to a user's payment subscription.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BotSubscriptionUpdated {
    /// User whose subscription changed.
    pub user: User,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// The new subscription state.
    pub state: BotSubscriptionState,
}

/// A payment subscription state reported by Telegram.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub enum BotSubscriptionState {
    /// The user canceled the subscription.
    Canceled,
    /// The user enabled or re-enabled the subscription.
    Active,
    /// A subscription payment failed.
    Failed,
    /// A state introduced by Telegram after this teloxide release.
    Unknown(String),
}

impl Serialize for BotSubscriptionState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let state = match self {
            Self::Canceled => "canceled",
            Self::Active => "active",
            Self::Failed => "failed",
            Self::Unknown(state) => state,
        };
        serializer.serialize_str(state)
    }
}

impl<'de> Deserialize<'de> for BotSubscriptionState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "canceled" => Self::Canceled,
            "active" => Self::Active,
            "failed" => Self::Failed,
            state => Self::Unknown(state.to_owned()),
        })
    }
}
