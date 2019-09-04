use crate::core::types::{
    Animation, Audio, Chat, Contact, Document, Game, InlineKeyboardMarkup,
    Invoice, Location, MessageEntity, PassportData, PhotoSize, Poll, Sticker,
    SuccessfulPayment, User, Venue, Video, VideoNote, Voice,
};

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Clone)]
pub struct Message {
    #[serde(rename = "message_id")]
    pub id: i32,
    pub date: i32,
    pub chat: Chat,
    #[serde(flatten)]
    pub type_: MessageType,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum MessageType {}
