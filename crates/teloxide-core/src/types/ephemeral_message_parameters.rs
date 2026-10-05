use crate::types::UserId;
use serde::{Deserialize, Serialize};

/// Parameters for sending an ephemeral message to a user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct EphemeralMessageParameters {
    /// Identifier of the user who will receive the message.
    pub receiver_user_id: UserId,
    /// Identifier of the callback query which triggered the message, if any.
    pub callback_query_id: Option<String>,
    /// Shows the ephemeral message in place of the original callback message.
    pub replace_callback_query_message: Option<bool>,
}

impl EphemeralMessageParameters {
    pub const fn new(receiver_user_id: UserId) -> Self {
        Self { receiver_user_id, callback_query_id: None, replace_callback_query_message: None }
    }

    pub fn callback_query_id(mut self, callback_query_id: impl Into<String>) -> Self {
        self.callback_query_id = Some(callback_query_id.into());
        self
    }

    pub const fn replace_callback_query_message(mut self, replace: bool) -> Self {
        self.replace_callback_query_message = Some(replace);
        self
    }
}
