use serde::{Deserialize, Serialize};

/// This object represents a service message about General forum topic unhidden
/// in the chat. Currently holds no information.
///
/// [The official docs](https://core.telegram.org/bots/api#generalforumtopicunhidden).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct GeneralForumTopicUnhidden {}
