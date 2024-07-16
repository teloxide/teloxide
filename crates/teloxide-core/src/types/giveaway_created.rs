use serde::{Deserialize, Serialize};

/// This object represents a service message about the creation of a scheduled
/// giveaway. Currently holds no information.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct GiveawayCreated {}
