use crate::core::types::{
    Animation, Audio, Chat, Contact, Document, Game, InlineKeyboardMarkup,
    Invoice, Location, MessageEntity, PassportData, PhotoSize, Poll, Sticker,
    SuccessfulPayment, User, Venue, Video, VideoNote, Voice,
};

#[derive(Debug, Deserialize, PartialEq, Clone, Serialize)]
pub struct Message {
    #[serde(rename = "message_id")]
    pub id: i64,
    pub date: i32,
    pub chat: Chat,
    #[serde(flatten)]
    pub message_kind: MessageKind,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageKind {
    IncomingMessage {
        #[serde(flatten)]
        from: Sender,
        #[serde(flatten)]
        forward_kind: ForwardKind,
        edit_date: Option<i32>,
        #[serde(flatten)]
        media_kind: MediaKind,
        reply_markup: Option<InlineKeyboardMarkup>,
    },
    NewChatMembers {
        new_chat_members: Vec<User>,
    },
    LeftChatMember {
        left_chat_member: User,
    },
    NewChatTitle {
        new_chat_title: String,
    },
    NewChatPhoto {
        new_chat_photo: Vec<PhotoSize>,
    },
    DeleteChatPhoto {
        delete_chat_photo: bool,
    },
    GroupChatCreated {
        group_chat_created: bool,
    },
    SupergroupChatCreated {
        supergroup_chat_created: bool,
    },
    ChannelChatCreated {
        channel_chat_created: bool,
    },
    Migrate {
        migrate_to_chat_id: i64,
        migrate_from_chat_id: i64,
    },
    PinnedMessage {
        pinned_message: Box<Message>,
    },
    Invoice {
        invoice: Invoice,
    },
    SuccessfulPayment {
        successful_payment: SuccessfulPayment,
    },
    ConnectedWebsite {
        connected_website: String,
    },
    PassportData {
        passport_data: PassportData,
    },
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
pub enum Sender {
    /// If message is sent from Chat
    #[serde(rename = "from")]
    User(User),
    /// If message is sent from Channel
    #[serde(rename = "author_signature")]
    Signature(String),
}

#[derive(Debug, Deserialize, PartialEq, Clone, Serialize)]
#[serde(untagged)]
pub enum ForwardKind {
    ChannelForward {
        #[serde(rename = "forward_date")]
        date: i32,
        #[serde(rename = "forward_from_chat")]
        chat: Chat,
        #[serde(rename = "forward_from_message_id")]
        message_id: i64,
        #[serde(rename = "forward_signature")]
        signature: Option<String>,
    },
    NonChannelForward {
        #[serde(rename = "forward_date")]
        date: i32,
        #[serde(flatten)]
        from: ForwardedFrom,
    },
    Origin {
        reply_to_message: Option<Box<Message>>,
    },
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
pub enum ForwardedFrom {
    #[serde(rename = "forward_from")]
    User(User),
    #[serde(rename = "forward_sender_name")]
    SenderName(String),
}

#[derive(Debug, Deserialize, PartialEq, Clone, Serialize)]
#[serde(untagged)]
pub enum MediaKind {
    Animation {
        animation: Animation,
        #[doc(hidden)]
        /// "For backward compatibility" (c) Telegram Docs
        #[serde(skip)]
        document: (),
        caption: Option<String>,
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>
    },
    Audio {
        audio: Audio,
        caption: Option<String>,
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>
    },
    Contact {
        contact: Contact,
    },
    Document {
        document: Document,
        caption: Option<String>,
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>
    },
    Game {
        game: Game,
    },
    Location {
        location: Location,
    },
    Photo {
        photo: Vec<PhotoSize>,
        caption: Option<String>,
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,
        media_group_id: Option<String>,
    },
    Poll {
        poll: Poll,
    },
    Sticker {
        sticker: Sticker,
    },
    Text {
        text: String,
        #[serde(default = "Vec::new")]
        entities: Vec<MessageEntity>,
    },
    Video {
        video: Video,
        caption: Option<String>,
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,
        media_group_id: Option<String>,
    },
    VideoNote {
        video_note: VideoNote,
    },
    Voice {
        voice: Voice,
        caption: Option<String>,
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,
    },
    Venue {
        venue: Venue,
    },
}

#[cfg(test)]
mod tests {
    use crate::core::types::*;
    use serde_json::from_str;

    #[test]
    fn sent_message_de() {
        let expected = Message {
            id: 6534,
            date: 1567898953,
            chat: Chat {
                id: 218485655,
                photo: None,
                kind: ChatKind::Private {
                    type_: (),
                    first_name: Some("W".to_string()),
                    last_name: None,
                    username: Some("WaffleLapkin".to_string()),
                },
            },
            message_kind: MessageKind::IncomingMessage {
                from: Sender::User(User {
                    id: 457569668,
                    is_bot: true,
                    first_name: "BT".to_string(),
                    last_name: None,
                    username: Some("BloodyTestBot".to_string()),
                    language_code: None,
                }),
                forward_kind: ForwardKind::Origin {
                    reply_to_message: None,
                },
                edit_date: None,
                media_kind: MediaKind::Text {
                    text: "text".to_string(),
                    entities: vec![],
                },
                reply_markup: None,
            },
        };
        // actual message from telegram
        let json = r#"{
  "message_id": 6534,
  "from": {
    "id": 457569668,
    "is_bot": true,
    "first_name": "BT",
    "username": "BloodyTestBot"
  },
  "chat": {
    "id": 218485655,
    "first_name": "W",
    "username": "WaffleLapkin",
    "type": "private"
  },
  "date": 1567898953,
  "text": "text"
}"#;
        let actual = from_str::<Message>(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn media_message_de() {
        let json = r#"{
  "message_id": 198283,
  "from": {
    "id": 250918540,
    "is_bot": false,
    "first_name": "Андрей",
    "last_name": "Власов",
    "username": "aka_dude",
    "language_code": "en"
  },
  "chat": {
    "id": 250918540,
    "first_name": "Андрей",
    "last_name": "Власов",
    "username": "aka_dude",
    "type": "private"
  },
  "date": 1567927221,
  "video": {
    "duration": 13,
    "width": 512,
    "height": 640,
    "mime_type": "video/mp4",
    "thumb": {
      "file_id": "AAQCAAOmBAACBf2oS53pByA-I4CWWCObDwAEAQAHbQADMWcAAhYE",
      "file_size": 10339,
      "width": 256,
      "height": 320
    },
    "file_id": "BAADAgADpgQAAgX9qEud6QcgPiOAlhYE",
    "file_size": 1381334
  }
}"#;
        let actual = from_str::<Message>(json).unwrap();
        let expected = Message {
            id: 198283,
            date: 1567927221,
            chat: Chat {
                id: 250918540,
                photo: None,
                kind: ChatKind::Private {
                    first_name: Some("Андрей".to_string()),
                    last_name: Some("Власов".to_string()),
                    username: Some("aka_dude".to_string()),
                    type_: ()
                }
            },
            message_kind: MessageKind::IncomingMessage {
                from: Sender::User(User {
                    id: 250918540,
                    is_bot: false,
                    first_name: "Андрей".to_string(),
                    last_name: Some("Власов".to_string()),
                    username: Some("aka_dude".to_string()),
                    language_code: Some("en".to_string())
                }),
                forward_kind: ForwardKind::Origin { reply_to_message: None },
                edit_date: None,
                media_kind: MediaKind::Video {
                    video: Video {
                        duration: 13,
                        width: 512,
                        height: 640,
                        mime_type: Some("video/mp4".to_string()),
                        thumb: Some(PhotoSize {
                            file_id: "AAQCAAOmBAACBf2oS53pByA-I4CWWCObDwAEAQAHbQADMWcAAhYE".to_string(),
                            file_size: Some(10339),
                            width: 256,
                            height: 320
                        }),
                        file_id: "BAADAgADpgQAAgX9qEud6QcgPiOAlhYE".to_string(),
                        file_size: Some(1381334)
                    },
                    caption: None,
                    caption_entities: vec![],
                    media_group_id: None
                },
                reply_markup: None
            },
            
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn media_group_message_de() {
        let json = r#"{
  "message_id": 198283,
  "from": {
    "id": 250918540,
    "is_bot": false,
    "first_name": "Андрей",
    "last_name": "Власов",
    "username": "aka_dude",
    "language_code": "en"
  },
  "chat": {
    "id": 250918540,
    "first_name": "Андрей",
    "last_name": "Власов",
    "username": "aka_dude",
    "type": "private"
  },
  "date": 1567927221,
  "media_group_id": "12543417770506682",
  "video": {
    "duration": 13,
    "width": 512,
    "height": 640,
    "mime_type": "video/mp4",
    "thumb": {
      "file_id": "AAQCAAOmBAACBf2oS53pByA-I4CWWCObDwAEAQAHbQADMWcAAhYE",
      "file_size": 10339,
      "width": 256,
      "height": 320
    },
    "file_id": "BAADAgADpgQAAgX9qEud6QcgPiOAlhYE",
    "file_size": 1381334
  }
}"#;
        let actual = from_str::<Message>(json).unwrap();
        let expected = Message {
            id: 198283,
            date: 1567927221,
            chat: Chat {
                id: 250918540,
                photo: None,
                kind: ChatKind::Private {
                    first_name: Some("Андрей".to_string()),
                    last_name: Some("Власов".to_string()),
                    username: Some("aka_dude".to_string()),
                    type_: ()
                }
            },
            message_kind: MessageKind::IncomingMessage {
                from: Sender::User(User {
                    id: 250918540,
                    is_bot: false,
                    first_name: "Андрей".to_string(),
                    last_name: Some("Власов".to_string()),
                    username: Some("aka_dude".to_string()),
                    language_code: Some("en".to_string())
                }),
                forward_kind: ForwardKind::Origin { reply_to_message: None },
                edit_date: None,
                media_kind: MediaKind::Video {
                    video: Video {
                        duration: 13,
                        width: 512,
                        height: 640,
                        mime_type: Some("video/mp4".to_string()),
                        thumb: Some(PhotoSize {
                            file_id: "AAQCAAOmBAACBf2oS53pByA-I4CWWCObDwAEAQAHbQADMWcAAhYE".to_string(),
                            file_size: Some(10339),
                            width: 256,
                            height: 320
                        }),
                        file_id: "BAADAgADpgQAAgX9qEud6QcgPiOAlhYE".to_string(),
                        file_size: Some(1381334)
                    },
                    caption: None,
                    caption_entities: vec![],
                    media_group_id: Some("12543417770506682".to_string())
                },
                reply_markup: None
            },
            
        };
        assert_eq!(actual, expected);
    }
}
