use serde::{Deserialize, Serialize};

use crate::types::User;

/// Describes the information about a paid media purchase.
///
/// [The official docs](https://core.telegram.org/bots/api#paidmediapurchased).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PaidMediaPurchased {
    /// User who purchased the media
    pub from: User,

    /// Bot-specified paid media payload
    pub paid_media_payload: String,
}
