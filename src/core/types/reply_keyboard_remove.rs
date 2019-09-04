/// Upon receiving a message with this object, Telegram clients will remove
/// the current custom keyboard and display the default letter-keyboard.
/// By default, custom keyboards are displayed until a new keyboard is sent
/// by a bot. An exception is made for one-time keyboards that are hidden
/// immediately after the user presses a button (see ReplyKeyboardMarkup).
#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: True,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}