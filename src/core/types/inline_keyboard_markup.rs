/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
