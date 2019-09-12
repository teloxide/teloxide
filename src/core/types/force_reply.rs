/// Upon receiving a message with this object, Telegram clients will
/// display a reply interface to the user (act as if the user has
/// selected the bot‘s message and tapped ’Reply'). This can be
/// extremely useful if you want to create user-friendly step-by-step
/// interfaces without having to sacrifice privacy mod
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the
    /// bot‘s message and tapped ’Reply'
    pub force_reply: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional. Use this parameter if you want to force reply from specific
    /// users only. Targets: 1) users that are @mentioned in the text of the
    /// [`Message`] object; 2) if the bot's message is a reply
    /// (has reply_to_message_id), sender of the original message.
    pub selective: Option<bool>,
}
