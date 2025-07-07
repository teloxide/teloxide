use crate::types::{Gift, GiftId, MessageEntity};
use serde::{Deserialize, Serialize};

/// Describes a service message about a regular gift that was sent or received.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct GiftInfo {
    /// Information about the gift
    pub gift: Gift,

    /// Unique identifier of the received gift for the bot; only present for
    /// gifts received on behalf of business accounts
    pub owned_gift_id: Option<GiftId>,

    /// Number of Telegram Stars that can be claimed by the receiver by
    /// converting the gift; omitted if conversion to Telegram Stars is
    /// impossible
    pub convert_star_count: Option<u32>,

    /// Number of Telegram Stars that were prepaid by the sender for the
    /// ability to upgrade the gift
    pub prepaid_upgrade_star_count: Option<u32>,

    /// `true`, if the gift can be upgraded to a unique gift
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_be_upgraded: bool,

    /// Text of the message that was added to the gift
    pub text: Option<String>,

    /// Special entities that appear in the text
    pub entities: Option<Vec<MessageEntity>>,

    /// `true`, if the sender and gift text are shown only to the gift receiver;
    /// otherwise, everyone will be able to see them
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_private: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"{
            "gift": {
                "id": "1234",
                "sticker": {
                    "width": 512,
                    "height": 512,
                    "emoji": "😡",
                    "set_name": "AdvenTimeAnim",
                    "is_animated": true,
                    "is_video": false,
                    "type": "regular",
                    "thumbnail": {
                        "file_id": "AAMCAgADGQEAARIt0GMwiZ6n4nRbxdpM3pL8vPX6PVAhAAIjAAOw0PgMaabKAcaXKCABAAdtAAMpBA",
                        "file_unique_id": "AQADIwADsND4DHI",
                        "file_size": 4118,
                        "width": 128,
                        "height": 128
                    },
                    "file_id": "CAACAgIAAxkBAAESLdBjMImep-J0W8XaTN6S_Lz1-j1QIQACIwADsND4DGmmygHGlyggKQQ",
                    "file_unique_id": "AgADIwADsND4DA",
                    "file_size": 16639
                },
                "star_count": 10
            }
        }"#;

        let gift_info: GiftInfo = serde_json::from_str(data).unwrap();
        assert_eq!(gift_info.gift.star_count, 10);
        assert_eq!(gift_info.can_be_upgraded, false);
        assert_eq!(gift_info.is_private, false);
        assert_eq!(gift_info.gift.id, "1234".into());
    }
}
