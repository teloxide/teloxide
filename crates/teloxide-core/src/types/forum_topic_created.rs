use serde::{Deserialize, Serialize};

/// This object represents a service message about a new forum topic created in
/// the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#forumtopiccreated).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicCreated {
    /// Name of the topic.
    pub name: String,

    /// Color of the topic icon in RGB format.
    // FIXME: use/add a specialized rgb color type?
    #[serde(with = "crate::types::serde_rgb")]
    pub icon_color: [u8; 3],

    /// Unique identifier of the custom emoji shown as the topic icon.
    // FIXME: CustomEmojiId
    pub icon_custom_emoji_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::types::ForumTopicCreated;

    #[test]
    fn deserialization() {
        let json =
            r#"{"icon_color":9367192,"icon_custom_emoji_id":"5312536423851630001","name":"???"}"#;

        let event = serde_json::from_str::<ForumTopicCreated>(json).unwrap();

        assert_eq!(event.name, "???");
        assert_eq!(event.icon_color, [0x8E, 0xEE, 0x98]);
        assert_eq!(event.icon_custom_emoji_id.as_deref(), Some("5312536423851630001"));
    }
}
