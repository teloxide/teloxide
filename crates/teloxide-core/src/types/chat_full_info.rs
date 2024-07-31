use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: in the TBA7.3 the Chat will be splitted into Chat and ChatInfo
// Currently it's just a container for the some fields of the Chat struct
#[serde_with::skip_serializing_none]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfo {
    /// Identifier of the accent color for the chat name and backgrounds of the
    /// chat photo, reply header, and link preview. See [accent colors] for more
    /// details.
    ///
    /// [accent colors]: https://core.telegram.org/bots/api#accent-colors
    pub accent_color_id: Option<u8>,
    /// Custom emoji identifier of the emoji chosen by the chat for the reply
    /// header and link preview background
    // FIXME: CustomEmojiId
    pub background_custom_emoji_id: Option<String>,
    /// Identifier of the accent color for the chat's profile background. See
    /// [profile accent colors] for more details.
    ///
    /// [profile accent colors]: https://core.telegram.org/bots/api#profile-accent-colors
    pub profile_accent_color_id: Option<u8>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile
    /// background
    // FIXME: CustomEmojiId
    pub profile_background_custom_emoji_id: Option<String>,
    /// Custom emoji identifier of emoji status of the other party in a private
    /// chat. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    // FIXME: CustomEmojiId
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date of the emoji status of the chat or the other party in a
    /// private chat, in Unix time, if any
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub emoji_status_expiration_date: Option<DateTime<Utc>>,
    /// True, if new chat members will have access to old messages; available
    /// only to chat administrators. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[serde(default)]
    pub has_visible_history: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_full_info_de() {
        assert_eq!(serde_json::from_str::<ChatFullInfo>("{}").unwrap(), ChatFullInfo::default());
        assert_eq!(
            serde_json::from_str::<ChatFullInfo>(
                r#"{
                    "emoji_status_expiration_date": 1720708004
                }"#
            )
            .unwrap(),
            ChatFullInfo {
                emoji_status_expiration_date: DateTime::from_timestamp(1720708004, 0),
                ..ChatFullInfo::default()
            }
        );
    }
}
