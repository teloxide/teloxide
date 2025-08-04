use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Describes an inline message to be sent by a user of a Mini App.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PreparedInlineMessage {
    /// Unique identifier of the prepared message
    pub id: String,

    /// Expiration date of the prepared message, in Unix time. Expired prepared
    /// messages can no longer be used
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub expiration_date: DateTime<Utc>,
}
