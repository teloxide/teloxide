use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Describes an inline message to be sent by a user of a Mini App.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedInlineMessage {
    /// Unique identifier of the prepared message.
    pub id: String,

    /// Expiration date of the prepared message, in Unix time.
    /// Expired prepared messages can no longer be used.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub expiration_date: DateTime<Utc>,
    
}
