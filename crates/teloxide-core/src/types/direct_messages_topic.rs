use serde::{Deserialize, Serialize};

use crate::types::User;

/// Unique identifier of the topic.
#[derive(Clone, Copy)]
#[derive(Debug, derive_more::Display)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct TopicId(pub i32);

/// Describes a topic of a direct messages chat.
///
/// [The official docs](https://core.telegram.org/bots/api#directmessagestopic).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DirectMessagesTopic {
    /// Unique identifier of the topic
    pub topic_id: TopicId,

    /// Information about the user that created the topic. Currently, it is
    /// always present
    pub user: Option<User>,
}
