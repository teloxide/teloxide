#![allow(clippy::large_enum_variant)]

use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Chat, Contact, Document, Game, InlineKeyboardMarkup,
    Invoice, Location, MessageEntity, PassportData, PhotoSize, Poll, Sticker,
    SuccessfulPayment, True, User, Venue, Video, VideoNote, Voice,
};

/// This object represents a message.
///
/// [The official docs](https://core.telegram.org/bots/api#message).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    /// Unique message identifier inside this chat.
    #[serde(rename = "message_id")]
    pub id: i32,

    /// Date the message was sent in Unix time.
    pub date: i32,

    /// Conversation the message belongs to.
    pub chat: Chat,

    #[serde(flatten)]
    pub kind: MessageKind,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MessageKind {
    Common {
        /// Sender, empty for messages sent to channels.
        #[serde(flatten)]
        from: Sender,

        #[serde(flatten)]
        forward_kind: ForwardKind,

        /// Date the message was last edited in Unix time.
        edit_date: Option<i32>,

        #[serde(flatten)]
        media_kind: MediaKind,

        /// Inline keyboard attached to the message. `login_url` buttons are
        /// represented as ordinary `url` buttons.
        reply_markup: Option<InlineKeyboardMarkup>,
    },
    NewChatMembers {
        /// New members that were added to the group or supergroup and
        /// information about them (the bot itself may be one of these
        /// members).
        new_chat_members: Vec<User>,
    },
    LeftChatMember {
        /// A member was removed from the group, information about them (this
        /// member may be the bot itself).
        left_chat_member: User,
    },
    NewChatTitle {
        /// A chat title was changed to this value.
        new_chat_title: String,
    },
    NewChatPhoto {
        /// A chat photo was change to this value.
        new_chat_photo: Vec<PhotoSize>,
    },
    DeleteChatPhoto {
        /// Service message: the chat photo was deleted.
        delete_chat_photo: True,
    },
    GroupChatCreated {
        /// Service message: the group has been created.
        group_chat_created: True,
    },
    SupergroupChatCreated {
        /// Service message: the supergroup has been created. This field can‘t
        /// be received in a message coming through updates, because bot can’t
        /// be a member of a supergroup when it is created. It can only be
        /// found in `reply_to_message` if someone replies to a very first
        /// message in a directly created supergroup.
        supergroup_chat_created: True,
    },
    ChannelChatCreated {
        /// Service message: the channel has been created. This field can‘t be
        /// received in a message coming through updates, because bot can’t be
        /// a member of a channel when it is created. It can only be found in
        /// `reply_to_message` if someone replies to a very first message in a
        /// channel.
        channel_chat_created: True,
    },
    Migrate {
        /// The group has been migrated to a supergroup with the specified
        /// identifier. This number may be greater than 32 bits and some
        /// programming languages may have difficulty/silent defects in
        /// interpreting it. But it is smaller than 52 bits, so a signed 64 bit
        /// integer or double-precision float type are safe for storing this
        /// identifier.
        migrate_to_chat_id: i64,

        /// The supergroup has been migrated from a group with the specified
        /// identifier. This number may be greater than 32 bits and some
        /// programming languages may have difficulty/silent defects in
        /// interpreting it. But it is smaller than 52 bits, so a signed 64 bit
        /// integer or double-precision float type are safe for storing this
        /// identifier.
        migrate_from_chat_id: i64,
    },
    Pinned {
        /// Specified message was pinned. Note that the Message object in this
        /// field will not contain further `reply_to_message` fields even if it
        /// is itself a reply.
        pinned: Box<Message>,
    },
    Invoice {
        /// Message is an invoice for a [payment], information about the
        /// invoice. [More about payments »].
        ///
        /// [payment]: https://core.telegram.org/bots/api#payments
        /// [More about payments »]: https://core.telegram.org/bots/api#payments
        invoice: Invoice,
    },
    SuccessfulPayment {
        /// Message is a service message about a successful payment,
        /// information about the payment. [More about payments »].
        ///
        /// [More about payments »]: https://core.telegram.org/bots/api#payments
        successful_payment: SuccessfulPayment,
    },
    ConnectedWebsite {
        /// The domain name of the website on which the user has logged in.
        /// [More about Telegram Login »].
        ///
        /// [More about Telegram Login »]: https://core.telegram.org/widgets/login
        connected_website: String,
    },
    PassportData {
        /// Telegram Passport data.
        passport_data: PassportData,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Sender {
    /// Sender of a message from chat.
    #[serde(rename = "from")]
    User(User),

    /// Signature of a sender of a message from a channel.
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MediaKind {
    Animation {
        /// Message is an animation, information about the animation. For
        /// backward compatibility, when this field is set, the document field
        /// will also be set.
        animation: Animation,

        #[doc(hidden)]
        /// "For backward compatibility" (c) Telegram Docs.
        #[serde(skip)]
        document: (),

        /// Caption for the animation, 0-1024 characters.
        caption: Option<String>,

        /// For messages with a caption, special entities like usernames, URLs,
        /// bot commands, etc. that appear in the caption.
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,
    },
    Audio {
        /// Message is an audio file, information about the file.
        audio: Audio,

        /// Caption for the audio, 0-1024 characters.
        caption: Option<String>,

        /// For messages with a caption, special entities like usernames, URLs,
        /// bot commands, etc. that appear in the caption.
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,
    },
    Contact {
        /// Message is a shared contact, information about the contact.
        contact: Contact,
    },
    Document {
        /// Message is a general file, information about the file.
        document: Document,

        /// Caption for the document, 0-1024 characters.
        caption: Option<String>,

        /// For messages with a caption, special entities like usernames, URLs,
        /// bot commands, etc. that appear in the caption.
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,
    },
    Game {
        /// Message is a game, information about the game. [More
        /// about games »].
        ///
        /// [More about games »]: https://core.telegram.org/bots/api#games
        game: Game,
    },
    Location {
        /// Message is a shared location, information about the location.
        location: Location,
    },
    Photo {
        /// Message is a photo, available sizes of the photo.
        photo: Vec<PhotoSize>,

        /// Caption for the photo, 0-1024 characters.
        caption: Option<String>,

        /// For messages with a caption, special entities like usernames, URLs,
        /// bot commands, etc. that appear in the caption.
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,

        /// The unique identifier of a media message group this message belongs
        /// to.
        media_group_id: Option<String>,
    },
    Poll {
        /// Message is a native poll, information about the poll.
        poll: Poll,
    },
    Sticker {
        /// Message is a sticker, information about the sticker.
        sticker: Sticker,
    },
    Text {
        /// For text messages, the actual UTF-8 text of the message, 0-4096
        /// characters.
        text: String,

        /// For text messages, special entities like usernames, URLs, bot
        /// commands, etc. that appear in the text.
        #[serde(default = "Vec::new")]
        entities: Vec<MessageEntity>,
    },
    Video {
        /// Message is a video, information about the video.
        video: Video,

        /// Caption for the video, 0-1024 characters.
        caption: Option<String>,

        /// For messages with a caption, special entities like usernames, URLs,
        /// bot commands, etc. that appear in the caption.
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,

        /// The unique identifier of a media message group this message belongs
        /// to.
        media_group_id: Option<String>,
    },
    VideoNote {
        /// Message is a [video note], information about the video message.
        ///
        /// [video note]: https://telegram.org/blog/video-messages-and-telescope
        video_note: VideoNote,
    },
    Voice {
        /// Message is a voice message, information about the file.
        voice: Voice,

        /// Caption for the voice, 0-1024 characters.
        caption: Option<String>,

        /// For messages with a caption, special entities like usernames, URLs,
        /// bot commands, etc. that appear in the caption.
        #[serde(default = "Vec::new")]
        caption_entities: Vec<MessageEntity>,
    },
    Venue {
        /// Message is a venue, information about the venue.
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
              "file_unique_id":"",
              "file_size": 10339,
              "width": 256,
              "height": 320
            },
            "file_id": "BAADAgADpgQAAgX9qEud6QcgPiOAlhYE",
            "file_unique_id":"",
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
              "file_unique_id":"",
              "file_size": 10339,
              "width": 256,
              "height": 320
            },
            "file_id": "BAADAgADpgQAAgX9qEud6QcgPiOAlhYE",
            "file_unique_id":"",
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
            "file_unique_id":"",
            "file_size": 4118,
            "width": 128,
            "height": 128
           },
           "file_id": "CAADAgADIwADsND4DGmmygHGlyggFgQ",
           "file_unique_id":"",
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
            "file_unique_id":"",
            "file_size": 18188,
            "width": 320,
            "height": 239
           },
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA3gAAyfcBAABFgQ",
            "file_unique_id":"",
            "file_size": 62123,
            "width": 800,
            "height": 598
           },
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA3kAAyTcBAABFgQ",
            "file_unique_id":"",
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
