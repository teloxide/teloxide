use serde::{Deserialize, Serialize};

use crate::types::{Rgb, ThreadId};

/// This object represents a forum topic.
///
/// [The official docs](https://core.telegram.org/bots/api#forumtopiccreated).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    #[serde(rename = "message_thread_id")]
    pub thread_id: ThreadId,

    /// Name of the topic.
    pub name: String,

    /// Color of the topic icon in RGB format.
    pub icon_color: Rgb,

    /// Unique identifier of the custom emoji shown as the topic icon.
    // FIXME: CustomEmojiId
    pub icon_custom_emoji_id: Option<String>,
}
