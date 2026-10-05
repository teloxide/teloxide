use serde::{Deserialize, Serialize};

use crate::types::{MaybeInaccessibleMessage, MessageEntity};

/// Service message about an option added to a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloptionadded).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOptionAdded {
    pub poll_message: Option<MaybeInaccessibleMessage>,
    pub option_persistent_id: String,
    pub option_text: String,
    pub option_text_entities: Option<Vec<MessageEntity>>,
}

/// Service message about an option deleted from a poll.
///
/// [The official docs](https://core.telegram.org/bots/api#polloptiondeleted).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOptionDeleted {
    pub poll_message: Option<MaybeInaccessibleMessage>,
    pub option_persistent_id: String,
    pub option_text: String,
    pub option_text_entities: Option<Vec<MessageEntity>>,
}
