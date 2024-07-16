use serde::{Deserialize, Serialize};

use crate::types::Message;

/// This object represents a service message about the completion of a giveaway
/// without public winners.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway
    pub winner_count: u32,

    /// Number of undistributed prizes
    pub unclaimed_prize_count: Option<u32>,

    /// Message with the giveaway that was completed, if it wasn't deleted
    pub giveaway_message: Option<Box<Message>>,
}
