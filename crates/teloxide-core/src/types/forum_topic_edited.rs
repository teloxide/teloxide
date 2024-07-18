use serde::{Deserialize, Serialize};

/// This object represents a service message about an edited forum topic.
///
/// [The official docs](https://core.telegram.org/bots/api#forumtopicedited).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicEdited {
    /// New name of the topic, if it was edited
    pub name: Option<String>,

    /// New identifier of the custom emoji shown as the topic icon, if it was
    /// edited; an empty string if the icon was removed
    pub icon_custom_emoji_id: Option<String>,
}
