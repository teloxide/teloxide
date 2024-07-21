use serde::{Deserialize, Serialize};

/// This object represents a service message about a forum topic reopened in the
/// chat. Currently holds no information.
///
/// [The official docs](https://core.telegram.org/bots/api#forumtopicreopened).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicReopened {}
