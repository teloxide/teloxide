use crate::core::types::{
    Animation, Audio, Chat, Contact, Document, Game, InlineKeyboardMarkup,
    Invoice, Location, MessageEntity, PassportData, PhotoSize, Poll, Sticker,
    SuccessfulPayment, User, Venue, Video, VideoNote, Voice,
};

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Message {
    #[serde(rename = "message_id")]
    pub id: i64,
    pub date: i32,
    pub chat: Chat,
    #[serde(flatten)]
    pub message_kind: MessageKind,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(untagged)]
pub enum MessageKind {
    IncomingMessage {
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

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
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

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum ForwardedFrom {
    #[serde(rename = "forward_from")]
    User(User),
    #[serde(rename = "forward_sender_name")]
    SenderName(String),
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(untagged)]
pub enum MediaKind {
    Animation {
        animation: Animation,
        caption: Option<String>,
    },
    Audio {
        audio: Audio,
        caption: Option<String>,
    },
    Contact {
        contact: Contact,
    },
    Document {
        document: Document,
        caption: Option<String>,
    },
    Game {
        game: Game,
    },
    Location {
        location: Location,
    },
    Photo {
        sizes: Vec<PhotoSize>,
        caption: Option<String>,
    },
    Poll {
        poll: Poll,
    },
    Sticker {
        sticker: Sticker,
    },
    Text {
        text: String,
        entities: Vec<MessageEntity>,
    },
    Video {
        video: Video,
        caption: Option<String>,
    },
    VideoNote {
        video_note: VideoNote,
    },
    Voice {
        voice: Voice,
        caption: Option<String>,
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
    fn origin_de() {
        let expected = Message {
            id: 0,
            date: 0,
            chat: Chat {
                id: 0,
                kind: ChatKind::Private {
                    type_: (),
                    username: None,
                    first_name: None,
                    last_name: None,
                },
                photo: None,
            },
            message_kind: MessageKind::IncomingMessage {
                forward_kind: ForwardKind::Origin {
                    reply_to_message: None,
                },
                edit_date: None,
                media_kind: MediaKind::Text {
                    text: "Hello".to_string(),
                    entities: vec![],
                },
                reply_markup: None,
            },
        };
        let actual = from_str::<Message>(r#"{"message_id":0,"date":0,"chat":{"chat_id":0,"type":"private"},"text":"Hello","entities":[]}"#).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn forward_de() {
        let expected = Message {
            id: 1,
            date: 1,
            chat: Chat {
                id: 1,
                kind: ChatKind::Private {
                    type_: (),
                    username: None,
                    first_name: None,
                    last_name: None,
                },
                photo: None,
            },
            message_kind: MessageKind::IncomingMessage {
                forward_kind: ForwardKind::NonChannelForward {
                    date: 1,
                    from: ForwardedFrom::User(User {
                        id: 123,
                        is_bot: false,
                        first_name: "Name".to_string(),
                        last_name: None,
                        username: None,
                        language_code: None,
                    }),
                },
                edit_date: None,
                media_kind: MediaKind::Text {
                    text: "Message".into(),
                    entities: vec![],
                },
                reply_markup: None,
            },
        };
        let actual = from_str::<Message>(
            r#"{"message_id":1,"date":1,"chat":{"chat_id":1,"type":"private"},"forward_date":1,"forward_from":{"id":123,"is_bot":false,"first_name":"Name"},"text":"Message","entities":[]}"#,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }
}
