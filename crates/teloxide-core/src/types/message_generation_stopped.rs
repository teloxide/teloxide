use crate::types::{Chat, ThreadId};
use serde::{Deserialize, Deserializer, Serialize};

/// An update about a user stopping message generation.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct MessageGenerationStopped {
    pub chat: Chat,
    pub message_thread_id: Option<ThreadId>,
    #[serde(deserialize_with = "deserialize_draft_id")]
    pub draft_id: i32,
}

fn deserialize_draft_id<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IntegerOrString {
        Integer(i32),
        String(String),
    }

    match IntegerOrString::deserialize(deserializer)? {
        IntegerOrString::Integer(value) => Ok(value),
        IntegerOrString::String(value) => value.parse().map_err(serde::de::Error::custom),
    }
}
