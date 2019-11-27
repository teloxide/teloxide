use crate::types::{
    Animation, Audio, Chat, Contact, Document, Game, InlineKeyboardMarkup,
    Invoice, Location, MessageEntity, PassportData, PhotoSize, Poll, Sticker,
    SuccessfulPayment, True, User, Venue, Video, VideoNote, Voice,
};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Message {
    #[serde(rename = "message_id")]
    pub id: i32,
    pub date: i32,
    pub chat: Chat,
    #[serde(flatten)]
    pub kind: MessageKind,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum MessageKind {
    Common {
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
        delete_chat_photo: True,
    },
    GroupChatCreated {
        group_chat_created: True,
    },
    SupergroupChatCreated {
        supergroup_chat_created: True,
    },
    ChannelChatCreated {
        channel_chat_created: True,
    },
    Migrate {
        migrate_to_chat_id: i64,
        migrate_from_chat_id: i64,
    },
    Pinned {
        pinned: Box<Message>,
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Sender {
    /// If message is sent from Chat
    #[serde(rename = "from")]
    User(User),
    /// If message is sent from Channel
    #[serde(rename = "author_signature")]
    Signature(String),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ForwardedFrom {
    #[serde(rename = "forward_from")]
    User(User),
    #[serde(rename = "forward_sender_name")]
    SenderName(String),
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum ForwardKind {
    ChannelForward {
        #[serde(rename = "forward_date")]
        date: i32,
        #[serde(rename = "forward_from_chat")]
        chat: Chat,
        #[serde(rename = "forward_from_message_id")]
        message_id: i32,
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

#[derive(Debug, Deserialize, PartialEq, Clone)]
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
        caption_entities: Vec<MessageEntity>,
    },
    Audio {
        audio: Audio,
        caption: Option<String>,
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,
    },
    Contact {
        contact: Contact,
    },
    Document {
        document: Document,
        caption: Option<String>,
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,
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

mod getters {
    use std::ops::Deref;

    use crate::types::{
        self,
        message::{
            ForwardKind::{ChannelForward, NonChannelForward, Origin},
            MediaKind::{
                Animation, Audio, Contact, Document, Game, Location, Photo,
                Poll, Sticker, Text, Venue, Video, VideoNote, Voice,
            },
            MessageKind::{
                ChannelChatCreated, Common, ConnectedWebsite, DeleteChatPhoto,
                GroupChatCreated, Invoice, LeftChatMember, Migrate,
                NewChatMembers, NewChatPhoto, NewChatTitle, PassportData,
                Pinned, SuccessfulPayment, SupergroupChatCreated,
            },
        },
        Chat, ForwardedFrom, Message, MessageEntity, PhotoSize, Sender, True,
        User,
    };

    /// Getters for [Message] fields from [telegram docs].
    ///
    /// [Message]: crate::types::Message
    /// [telegram docs]: https://core.telegram.org/bots/api#message
    impl Message {
        /// NOTE: this is getter for both `from` and `author_signature`
        pub fn from(&self) -> Option<&Sender> {
            match &self.kind {
                Common { from, .. } => Some(from),
                _ => None,
            }
        }

        /// NOTE: this is getter for both `forward_from` and
        /// `forward_sender_name`
        pub fn forward_from(&self) -> Option<&ForwardedFrom> {
            match &self.kind {
                Common {
                    forward_kind: NonChannelForward { from, .. },
                    ..
                } => Some(from),
                _ => None,
            }
        }

        pub fn forward_from_chat(&self) -> Option<&Chat> {
            match &self.kind {
                Common {
                    forward_kind: ChannelForward { chat, .. },
                    ..
                } => Some(chat),
                _ => None,
            }
        }

        pub fn forward_from_message_id(&self) -> Option<&i32> {
            match &self.kind {
                Common {
                    forward_kind: ChannelForward { message_id, .. },
                    ..
                } => Some(message_id),
                _ => None,
            }
        }

        pub fn forward_signature(&self) -> Option<&str> {
            match &self.kind {
                Common {
                    forward_kind: ChannelForward { signature, .. },
                    ..
                } => signature.as_ref().map(Deref::deref),
                _ => None,
            }
        }

        pub fn forward_date(&self) -> Option<&i32> {
            match &self.kind {
                Common {
                    forward_kind: ChannelForward { date, .. },
                    ..
                }
                | Common {
                    forward_kind: NonChannelForward { date, .. },
                    ..
                } => Some(date),
                _ => None,
            }
        }

        pub fn reply_to_message(&self) -> Option<&Message> {
            match &self.kind {
                Common {
                    forward_kind:
                        Origin {
                            reply_to_message, ..
                        },
                    ..
                } => reply_to_message.as_ref().map(Deref::deref),
                _ => None,
            }
        }

        pub fn edit_date(&self) -> Option<&i32> {
            match &self.kind {
                Common { edit_date, .. } => edit_date.as_ref(),
                _ => None,
            }
        }

        pub fn media_group_id(&self) -> Option<&str> {
            match &self.kind {
                Common {
                    media_kind: Video { media_group_id, .. },
                    ..
                }
                | Common {
                    media_kind: Photo { media_group_id, .. },
                    ..
                } => media_group_id.as_ref().map(Deref::deref),
                _ => None,
            }
        }

        pub fn text(&self) -> Option<&str> {
            match &self.kind {
                Common {
                    media_kind: Text { text, .. },
                    ..
                } => Some(text),
                _ => None,
            }
        }

        pub fn entities(&self) -> Option<&[MessageEntity]> {
            match &self.kind {
                Common {
                    media_kind: Text { entities, .. },
                    ..
                } => Some(entities),
                _ => None,
            }
        }

        pub fn caption_entities(&self) -> Option<&[MessageEntity]> {
            match &self.kind {
                Common {
                    media_kind:
                        Animation {
                            caption_entities, ..
                        },
                    ..
                }
                | Common {
                    media_kind:
                        Audio {
                            caption_entities, ..
                        },
                    ..
                }
                | Common {
                    media_kind:
                        Document {
                            caption_entities, ..
                        },
                    ..
                }
                | Common {
                    media_kind:
                        Photo {
                            caption_entities, ..
                        },
                    ..
                }
                | Common {
                    media_kind:
                        Video {
                            caption_entities, ..
                        },
                    ..
                }
                | Common {
                    media_kind:
                        Voice {
                            caption_entities, ..
                        },
                    ..
                } => Some(caption_entities),
                _ => None,
            }
        }

        pub fn audio(&self) -> Option<&types::Audio> {
            match &self.kind {
                Common {
                    media_kind: Audio { audio, .. },
                    ..
                } => Some(audio),
                _ => None,
            }
        }

        pub fn document(&self) -> Option<&types::Document> {
            match &self.kind {
                Common {
                    media_kind: Document { document, .. },
                    ..
                } => Some(document),
                _ => None,
            }
        }

        pub fn animation(&self) -> Option<&types::Animation> {
            match &self.kind {
                Common {
                    media_kind: Animation { animation, .. },
                    ..
                } => Some(animation),
                _ => None,
            }
        }

        pub fn game(&self) -> Option<&types::Game> {
            match &self.kind {
                Common {
                    media_kind: Game { game, .. },
                    ..
                } => Some(game),
                _ => None,
            }
        }

        pub fn photo(&self) -> Option<&[PhotoSize]> {
            match &self.kind {
                Common {
                    media_kind: Photo { photo, .. },
                    ..
                } => Some(photo),
                _ => None,
            }
        }

        pub fn sticker(&self) -> Option<&types::Sticker> {
            match &self.kind {
                Common {
                    media_kind: Sticker { sticker, .. },
                    ..
                } => Some(sticker),
                _ => None,
            }
        }

        pub fn video(&self) -> Option<&types::Video> {
            match &self.kind {
                Common {
                    media_kind: Video { video, .. },
                    ..
                } => Some(video),
                _ => None,
            }
        }

        pub fn voice(&self) -> Option<&types::Voice> {
            match &self.kind {
                Common {
                    media_kind: Voice { voice, .. },
                    ..
                } => Some(voice),
                _ => None,
            }
        }

        pub fn video_note(&self) -> Option<&types::VideoNote> {
            match &self.kind {
                Common {
                    media_kind: VideoNote { video_note, .. },
                    ..
                } => Some(video_note),
                _ => None,
            }
        }

        pub fn caption(&self) -> Option<&str> {
            match &self.kind {
                Common { media_kind, .. } => match media_kind {
                    Animation { caption, .. }
                    | Audio { caption, .. }
                    | Document { caption, .. }
                    | Photo { caption, .. }
                    | Video { caption, .. }
                    | Voice { caption, .. } => {
                        caption.as_ref().map(Deref::deref)
                    }
                    _ => None,
                },
                _ => None,
            }
        }

        pub fn contact(&self) -> Option<&types::Contact> {
            match &self.kind {
                Common {
                    media_kind: Contact { contact },
                    ..
                } => Some(contact),
                _ => None,
            }
        }

        pub fn location(&self) -> Option<&types::Location> {
            match &self.kind {
                Common {
                    media_kind: Location { location, .. },
                    ..
                } => Some(location),
                _ => None,
            }
        }

        pub fn venue(&self) -> Option<&types::Venue> {
            match &self.kind {
                Common {
                    media_kind: Venue { venue, .. },
                    ..
                } => Some(venue),
                _ => None,
            }
        }

        pub fn poll(&self) -> Option<&types::Poll> {
            match &self.kind {
                Common {
                    media_kind: Poll { poll, .. },
                    ..
                } => Some(poll),
                _ => None,
            }
        }

        pub fn new_chat_members(&self) -> Option<&[User]> {
            match &self.kind {
                NewChatMembers { new_chat_members } => Some(new_chat_members),
                _ => None,
            }
        }

        pub fn left_chat_member(&self) -> Option<&User> {
            match &self.kind {
                LeftChatMember { left_chat_member } => Some(left_chat_member),
                _ => None,
            }
        }

        pub fn new_chat_title(&self) -> Option<&str> {
            match &self.kind {
                NewChatTitle { new_chat_title } => Some(new_chat_title),
                _ => None,
            }
        }

        pub fn new_chat_photo(&self) -> Option<&[PhotoSize]> {
            match &self.kind {
                NewChatPhoto { new_chat_photo } => Some(new_chat_photo),
                _ => None,
            }
        }

        // TODO: OK, `Option<True>` is weird, can we do something with it?
        //       mb smt like `is_delete_chat_photo(&self) -> bool`?
        pub fn delete_chat_photo(&self) -> Option<True> {
            match &self.kind {
                DeleteChatPhoto { delete_chat_photo } => {
                    Some(*delete_chat_photo)
                }
                _ => None,
            }
        }

        pub fn group_chat_created(&self) -> Option<True> {
            match &self.kind {
                GroupChatCreated { group_chat_created } => {
                    Some(*group_chat_created)
                }
                _ => None,
            }
        }

        pub fn super_group_chat_created(&self) -> Option<True> {
            match &self.kind {
                SupergroupChatCreated {
                    supergroup_chat_created,
                } => Some(*supergroup_chat_created),
                _ => None,
            }
        }

        pub fn channel_chat_created(&self) -> Option<True> {
            match &self.kind {
                ChannelChatCreated {
                    channel_chat_created,
                } => Some(*channel_chat_created),
                _ => None,
            }
        }

        pub fn migrate_to_chat_id(&self) -> Option<&i64> {
            match &self.kind {
                Migrate {
                    migrate_to_chat_id, ..
                } => Some(migrate_to_chat_id),
                _ => None,
            }
        }

        pub fn migrate_from_chat_id(&self) -> Option<&i64> {
            match &self.kind {
                Migrate {
                    migrate_from_chat_id,
                    ..
                } => Some(migrate_from_chat_id),
                _ => None,
            }
        }

        pub fn pinned_message(&self) -> Option<&Message> {
            match &self.kind {
                Pinned { pinned } => Some(pinned),
                _ => None,
            }
        }

        pub fn invoice(&self) -> Option<&types::Invoice> {
            match &self.kind {
                Invoice { invoice } => Some(invoice),
                _ => None,
            }
        }

        pub fn successful_payment(&self) -> Option<&types::SuccessfulPayment> {
            match &self.kind {
                SuccessfulPayment { successful_payment } => {
                    Some(successful_payment)
                }
                _ => None,
            }
        }

        pub fn connected_website(&self) -> Option<&str> {
            match &self.kind {
                ConnectedWebsite { connected_website } => {
                    Some(connected_website)
                }
                _ => None,
            }
        }

        pub fn passport_data(&self) -> Option<&types::PassportData> {
            match &self.kind {
                PassportData { passport_data } => Some(passport_data),
                _ => None,
            }
        }

        pub fn reply_markup(&self) -> Option<&types::InlineKeyboardMarkup> {
            match &self.kind {
                Common { reply_markup, .. } => reply_markup.as_ref(),
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use crate::types::*;

    #[test]
    fn de_media_forwarded() {
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
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    #[test]
    fn de_media_group_forwarded() {
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
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    #[test]
    fn de_text() {
        let json = r#"{
          "message_id": 199785,
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
          "date": 1568289890,
          "text": "Лол кек 😂"
         }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    #[test]
    fn de_sticker() {
        let json = r#"{
          "message_id": 199787,
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
          "date": 1568290188,
          "sticker": {
           "width": 512,
           "height": 512,
           "emoji": "😡",
           "set_name": "AdvenTimeAnim",
           "is_animated": true,
           "thumb": {
            "file_id": "AAQCAAMjAAOw0PgMaabKAcaXKCBLubkPAAQBAAdtAAPGKwACFgQ",
            "file_size": 4118,
            "width": 128,
            "height": 128
           },
           "file_id": "CAADAgADIwADsND4DGmmygHGlyggFgQ",
           "file_size": 16639
          }
         }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    #[test]
    fn de_image() {
        let json = r#"{
          "message_id": 199791,
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
          "date": 1568290622,
          "photo": [
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA20AAybcBAABFgQ",
            "file_size": 18188,
            "width": 320,
            "height": 239
           },
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA3gAAyfcBAABFgQ",
            "file_size": 62123,
            "width": 800,
            "height": 598
           },
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA3kAAyTcBAABFgQ",
            "file_size": 75245,
            "width": 962,
            "height": 719
           }
          ]
         }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }
}
