use serde::{Deserialize, Serialize};

/// Represents a community, a group of chats linked around a shared topic or
/// audience.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct Community {
    /// Unique identifier for this community.
    pub id: i64,
    /// Name of the community.
    pub name: String,
}

/// Describes a service message about a chat being added to a community.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatAdded {
    /// The community to which the chat was added.
    pub community: Community,
}

/// Describes a service message about a chat being removed from a community.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatRemoved {}

/// Describes a service message about a chat being joined by a user from a
/// community.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct CommunityChatJoined {
    pub community: Community,
}
