use serde::{Deserialize, Serialize};

/// Type of the chat, from which the inline query was sent.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    /// Private chat with the inline query sender.
    Sender,
    Private,
    Group,
    Supergroup,
    Channel,
}
