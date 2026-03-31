use serde::{Deserialize, Serialize};

/// Describes a service message about a change in the price of paid messages
/// within a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMessagePriceChanged {
    /// The new number of Telegram Stars that must be paid by non-administrator
    /// users of the supergroup chat for each sent message
    pub paid_message_star_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"{"paid_message_star_count": 1234}"#;
        assert_eq!(
            serde_json::from_str::<PaidMessagePriceChanged>(data).unwrap(),
            PaidMessagePriceChanged { paid_message_star_count: 1234 }
        );
    }
}
