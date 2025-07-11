use serde::{Deserialize, Serialize};

/// Describes a service message about a change in the price of direct messages
/// sent to a channel chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct DirectMessagePriceChanged {
    /// _true_ if direct messages are enabled for the channel chat; false
    /// otherwise
    pub are_direct_messages_enabled: bool,

    /// The new number of Telegram Stars that must be paid by users for each
    /// direct message sent to the channel. Does not apply to users who have
    /// been exempted by administrators. Defaults to 0.
    pub direct_message_star_count: Option<u32>,
}
