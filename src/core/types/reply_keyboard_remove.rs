/// Upon receiving a message with this object, Telegram clients will remove
/// the current custom keyboard and display the default letter-keyboard.
/// By default, custom keyboards are displayed until a new keyboard is sent
/// by a bot. An exception is made for one-time keyboards that are hidden
/// immediately after the user presses a button (see [`ReplyKeyboardMarkup`]).
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct ReplyKeyboardRemove {
    /// equests clients to remove the custom keyboard (user will not be able to
    /// summon this keyboard; if you want to hide the keyboard from sight but
    /// keep it accessible, use one_time_keyboard in ReplyKeyboardMarkup)
    pub remove_keyboard: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional. Use this parameter if you want to show the keyboard to
    /// specific users only. Targets: 1) users that are @mentioned in the text
    /// of the [`Message`] object; 2) if the bot's message is a reply
    /// (has reply_to_message_id), sender of the original message.
    ///
    /// Example: A user requests to change the bot‘s language, bot replies to
    /// the request with a keyboard to select the new language. Other users in
    /// the group don’t see the keyboard.
    pub selective: Option<bool>,
}
