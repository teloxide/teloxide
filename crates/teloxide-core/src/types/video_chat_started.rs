use serde::{Deserialize, Serialize};

/// This object represents a service message about a video chat started in the
/// chat. Currently holds no information.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct VideoChatStarted {}
