#![allow(clippy::large_enum_variant)]

use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Chat, Contact, Dice, Document, Game, InlineKeyboardMarkup, Invoice, Location,
    MessageAutoDeleteTimerChanged, MessageEntity, PassportData, PhotoSize, Poll,
    ProximityAlertTriggered, Sticker, SuccessfulPayment, True, User, Venue, Video, VideoNote,
    Voice, VoiceChatEnded, VoiceChatParticipantsInvited, VoiceChatScheduled, VoiceChatStarted,
};

/// This object represents a message.
///
/// [The official docs](https://core.telegram.org/bots/api#message).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat.
    #[serde(rename = "message_id")]
    pub id: i32,

    /// Date the message was sent in Unix time.
    pub date: i32,

    /// Conversation the message belongs to.
    pub chat: Chat,

    /// Bot through which the message was sent.
    pub via_bot: Option<User>,

    #[serde(flatten)]
    pub kind: MessageKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageKind {
    Common(MessageCommon),
    NewChatMembers(MessageNewChatMembers),
    LeftChatMember(MessageLeftChatMember),
    NewChatTitle(MessageNewChatTitle),
    NewChatPhoto(MessageNewChatPhoto),
    DeleteChatPhoto(MessageDeleteChatPhoto),
    GroupChatCreated(MessageGroupChatCreated),
    SupergroupChatCreated(MessageSupergroupChatCreated),
    ChannelChatCreated(MessageChannelChatCreated),
    MessageAutoDeleteTimerChanged(MessageMessageAutoDeleteTimerChanged),
    Migrate(MessageMigrate),
    Pinned(MessagePinned),
    Invoice(MessageInvoice),
    SuccessfulPayment(MessageSuccessfulPayment),
    ConnectedWebsite(MessageConnectedWebsite),
    PassportData(MessagePassportData),
    Dice(MessageDice),
    ProximityAlertTriggered(MessageProximityAlertTriggered),
    VoiceChatScheduled(MessageVoiceChatScheduled),
    VoiceChatStarted(MessageVoiceChatStarted),
    VoiceChatEnded(MessageVoiceChatEnded),
    VoiceChatParticipantsInvited(MessageVoiceChatParticipantsInvited),
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageCommon {
    /// Sender, empty for messages sent to channels.
    pub from: Option<User>,

    /// Sender of the message, sent on behalf of a chat. The channel itself for
    /// channel messages. The supergroup itself for messages from anonymous
    /// group administrators. The linked channel for messages automatically
    /// forwarded to the discussion group
    pub sender_chat: Option<Chat>,

    /// Signature of the post author for messages in channels, or the custom
    /// title of an anonymous group administrator.
    pub author_signature: Option<String>,

    #[serde(flatten)]
    pub forward_kind: ForwardKind,

    /// Date the message was last edited in Unix time.
    pub edit_date: Option<i32>,

    #[serde(flatten)]
    pub media_kind: MediaKind,

    /// Inline keyboard attached to the message. `login_url` buttons are
    /// represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageNewChatMembers {
    /// New members that were added to the group or supergroup and
    /// information about them (the bot itself may be one of these
    /// members).
    pub new_chat_members: Vec<User>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageLeftChatMember {
    /// A member was removed from the group, information about them (this
    /// member may be the bot itself).
    pub left_chat_member: User,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageNewChatTitle {
    /// A chat title was changed to this value.
    pub new_chat_title: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageNewChatPhoto {
    /// A chat photo was change to this value.
    pub new_chat_photo: Vec<PhotoSize>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageDeleteChatPhoto {
    /// Service message: the chat photo was deleted.
    pub delete_chat_photo: True,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageGroupChatCreated {
    /// Service message: the group has been created.
    pub group_chat_created: True,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageSupergroupChatCreated {
    /// Service message: the supergroup has been created. This field can‘t
    /// be received in a message coming through updates, because bot can’t
    /// be a member of a supergroup when it is created. It can only be
    /// found in `reply_to_message` if someone replies to a very first
    /// message in a directly created supergroup.
    pub supergroup_chat_created: True,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageChannelChatCreated {
    /// Service message: the channel has been created. This field can‘t be
    /// received in a message coming through updates, because bot can’t be
    /// a member of a channel when it is created. It can only be found in
    /// `reply_to_message` if someone replies to a very first message in a
    /// channel.
    pub channel_chat_created: True,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageMessageAutoDeleteTimerChanged {
    /// Service message: auto-delete timer settings changed in the chat.
    pub message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageMigrate {
    /// The group has been migrated to a supergroup with the specified
    /// identifier. This number may be greater than 32 bits and some
    /// programming languages may have difficulty/silent defects in
    /// interpreting it. But it is smaller than 52 bits, so a signed 64 bit
    /// integer or double-precision float type are safe for storing this
    /// identifier.
    pub migrate_to_chat_id: i64,

    /// The supergroup has been migrated from a group with the specified
    /// identifier. This number may be greater than 32 bits and some
    /// programming languages may have difficulty/silent defects in
    /// interpreting it. But it is smaller than 52 bits, so a signed 64 bit
    /// integer or double-precision float type are safe for storing this
    /// identifier.
    pub migrate_from_chat_id: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagePinned {
    /// Specified message was pinned. Note that the Message object in this
    /// field will not contain further `reply_to_message` fields even if it
    /// is itself a reply.
    #[serde(rename = "pinned_message")]
    pub pinned: Box<Message>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageInvoice {
    /// Message is an invoice for a [payment], information about the
    /// invoice. [More about payments »].
    ///
    /// [payment]: https://core.telegram.org/bots/api#payments
    /// [More about payments »]: https://core.telegram.org/bots/api#payments
    pub invoice: Invoice,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageSuccessfulPayment {
    /// Message is a service message about a successful payment,
    /// information about the payment. [More about payments »].
    ///
    /// [More about payments »]: https://core.telegram.org/bots/api#payments
    pub successful_payment: SuccessfulPayment,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageConnectedWebsite {
    /// The domain name of the website on which the user has logged in.
    /// [More about Telegram Login »].
    ///
    /// [More about Telegram Login »]: https://core.telegram.org/widgets/login
    pub connected_website: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagePassportData {
    /// Telegram Passport data.
    pub passport_data: PassportData,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ForwardedFrom {
    #[serde(rename = "forward_from")]
    User(User),
    #[serde(rename = "forward_sender_name")]
    SenderName(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForwardKind {
    Channel(ForwardChannel),
    NonChannel(ForwardNonChannel),
    Origin(ForwardOrigin),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForwardChannel {
    #[serde(rename = "forward_date")]
    pub date: i32,

    #[serde(rename = "forward_from_chat")]
    pub chat: Chat,

    #[serde(rename = "forward_from_message_id")]
    pub message_id: i32,

    #[serde(rename = "forward_signature")]
    pub signature: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForwardNonChannel {
    #[serde(rename = "forward_date")]
    pub date: i32,

    #[serde(flatten)]
    pub from: ForwardedFrom,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ForwardOrigin {
    pub reply_to_message: Option<Box<Message>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaKind {
    Animation(MediaAnimation),
    Audio(MediaAudio),
    Contact(MediaContact),
    Document(MediaDocument),
    Game(MediaGame),
    Location(MediaLocation),
    Photo(MediaPhoto),
    Poll(MediaPoll),
    Sticker(MediaSticker),
    Text(MediaText),
    Video(MediaVideo),
    VideoNote(MediaVideoNote),
    Voice(MediaVoice),
    Venue(MediaVenue),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaAnimation {
    /// Message is an animation, information about the animation. For
    /// backward compatibility, when this field is set, the document field
    /// will also be set.
    pub animation: Animation,

    #[doc(hidden)]
    /// "For backward compatibility" (c) Telegram Docs.
    #[serde(skip)]
    pub document: (),

    /// Caption for the animation, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaAudio {
    /// Message is an audio file, information about the file.
    pub audio: Audio,

    /// Caption for the audio, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContact {
    /// Message is a shared contact, information about the contact.
    pub contact: Contact,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaDocument {
    /// Message is a general file, information about the file.
    pub document: Document,

    /// Caption for the document, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGame {
    /// Message is a game, information about the game. [More
    /// about games »].
    ///
    /// [More about games »]: https://core.telegram.org/bots/api#games
    pub game: Game,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaLocation {
    /// Message is a shared location, information about the location.
    pub location: Location,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaPhoto {
    /// Message is a photo, available sizes of the photo.
    pub photo: Vec<PhotoSize>,

    /// Caption for the photo, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,

    /// The unique identifier of a media message group this message belongs
    /// to.
    pub media_group_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaPoll {
    /// Message is a native poll, information about the poll.
    pub poll: Poll,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaSticker {
    /// Message is a sticker, information about the sticker.
    pub sticker: Sticker,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaText {
    /// For text messages, the actual UTF-8 text of the message, 0-4096
    /// characters.
    pub text: String,

    /// For text messages, special entities like usernames, URLs, bot
    /// commands, etc. that appear in the text.
    #[serde(default = "Vec::new")]
    pub entities: Vec<MessageEntity>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVideo {
    /// Message is a video, information about the video.
    pub video: Video,

    /// Caption for the video, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,

    /// The unique identifier of a media message group this message belongs
    /// to.
    pub media_group_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVideoNote {
    /// Message is a [video note], information about the video message.
    ///
    /// [video note]: https://telegram.org/blog/video-messages-and-telescope
    pub video_note: VideoNote,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVoice {
    /// Message is a voice message, information about the file.
    pub voice: Voice,

    /// Caption for the voice, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVenue {
    /// Message is a venue, information about the venue.
    pub venue: Venue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDice {
    /// Message is a dice with random value from 1 to 6.
    pub dice: Dice,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageProximityAlertTriggered {
    /// Service message. A user in the chat triggered another user's proximity
    /// alert while sharing Live Location.
    pub proximity_alert_triggered: ProximityAlertTriggered,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVoiceChatScheduled {
    /// Service message: voice chat scheduled
    pub voice_chat_scheduled: VoiceChatScheduled,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVoiceChatStarted {
    /// Service message: voice chat started.
    pub voice_chat_started: VoiceChatStarted,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVoiceChatEnded {
    /// Service message: voice chat ended.
    pub voice_chat_ended: VoiceChatEnded,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVoiceChatParticipantsInvited {
    /// Service message: new participants invited to a voice chat.
    pub voice_chat_participants_invited: VoiceChatParticipantsInvited,
}

mod getters {
    use std::ops::Deref;

    use crate::types::{
        self,
        message::{ForwardKind::NonChannel, MessageKind::*},
        Chat, ForwardChannel, ForwardKind, ForwardNonChannel, ForwardOrigin, ForwardedFrom,
        MediaAnimation, MediaAudio, MediaContact, MediaDocument, MediaGame, MediaKind,
        MediaLocation, MediaPhoto, MediaPoll, MediaSticker, MediaText, MediaVenue, MediaVideo,
        MediaVideoNote, MediaVoice, Message, MessageChannelChatCreated, MessageCommon,
        MessageConnectedWebsite, MessageDeleteChatPhoto, MessageDice, MessageEntity,
        MessageGroupChatCreated, MessageInvoice, MessageLeftChatMember, MessageMigrate,
        MessageNewChatMembers, MessageNewChatPhoto, MessageNewChatTitle, MessagePassportData,
        MessagePinned, MessageProximityAlertTriggered, MessageSuccessfulPayment,
        MessageSupergroupChatCreated, PhotoSize, True, User,
    };

    /// Getters for [Message] fields from [telegram docs].
    ///
    /// [Message]: crate::types::Message
    /// [telegram docs]: https://core.telegram.org/bots/api#message
    impl Message {
        pub fn from(&self) -> Option<&User> {
            match &self.kind {
                Common(MessageCommon { from, .. }) => from.as_ref(),
                _ => None,
            }
        }

        pub fn author_signature(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon {
                    author_signature, ..
                }) => author_signature.as_deref(),
                _ => None,
            }
        }

        pub fn sender_chat(&self) -> Option<&Chat> {
            match &self.kind {
                Common(MessageCommon { sender_chat, .. }) => sender_chat.as_ref(),
                _ => None,
            }
        }

        pub fn chat_id(&self) -> i64 {
            self.chat.id
        }

        /// NOTE: this is getter for both `forward_from` and
        /// `forward_sender_name`
        pub fn forward_from(&self) -> Option<&ForwardedFrom> {
            match &self.kind {
                Common(MessageCommon {
                    forward_kind: NonChannel(ForwardNonChannel { from, .. }),
                    ..
                }) => Some(from),
                _ => None,
            }
        }

        pub fn forward_from_chat(&self) -> Option<&Chat> {
            match &self.kind {
                Common(MessageCommon {
                    forward_kind: ForwardKind::Channel(ForwardChannel { chat, .. }),
                    ..
                }) => Some(chat),
                _ => None,
            }
        }

        pub fn forward_from_message_id(&self) -> Option<&i32> {
            match &self.kind {
                Common(MessageCommon {
                    forward_kind: ForwardKind::Channel(ForwardChannel { message_id, .. }),
                    ..
                }) => Some(message_id),
                _ => None,
            }
        }

        pub fn forward_signature(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon {
                    forward_kind: ForwardKind::Channel(ForwardChannel { signature, .. }),
                    ..
                }) => signature.as_ref().map(Deref::deref),
                _ => None,
            }
        }

        pub fn forward_date(&self) -> Option<&i32> {
            match &self.kind {
                Common(MessageCommon {
                    forward_kind: ForwardKind::Channel(ForwardChannel { date, .. }),
                    ..
                })
                | Common(MessageCommon {
                    forward_kind: ForwardKind::NonChannel(ForwardNonChannel { date, .. }),
                    ..
                }) => Some(date),
                _ => None,
            }
        }

        pub fn reply_to_message(&self) -> Option<&Message> {
            match &self.kind {
                Common(MessageCommon {
                    forward_kind:
                        ForwardKind::Origin(ForwardOrigin {
                            reply_to_message, ..
                        }),
                    ..
                }) => reply_to_message.as_ref().map(Deref::deref),
                _ => None,
            }
        }

        pub fn edit_date(&self) -> Option<&i32> {
            match &self.kind {
                Common(MessageCommon { edit_date, .. }) => edit_date.as_ref(),
                _ => None,
            }
        }

        pub fn media_group_id(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Video(MediaVideo { media_group_id, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Photo(MediaPhoto { media_group_id, .. }),
                    ..
                }) => media_group_id.as_ref().map(Deref::deref),
                _ => None,
            }
        }

        pub fn text(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Text(MediaText { text, .. }),
                    ..
                }) => Some(text),
                _ => None,
            }
        }

        pub fn entities(&self) -> Option<&[MessageEntity]> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Text(MediaText { entities, .. }),
                    ..
                }) => Some(entities),
                _ => None,
            }
        }

        pub fn caption_entities(&self) -> Option<&[MessageEntity]> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind:
                        MediaKind::Animation(MediaAnimation {
                            caption_entities, ..
                        }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind:
                        MediaKind::Audio(MediaAudio {
                            caption_entities, ..
                        }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind:
                        MediaKind::Document(MediaDocument {
                            caption_entities, ..
                        }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind:
                        MediaKind::Photo(MediaPhoto {
                            caption_entities, ..
                        }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind:
                        MediaKind::Video(MediaVideo {
                            caption_entities, ..
                        }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind:
                        MediaKind::Voice(MediaVoice {
                            caption_entities, ..
                        }),
                    ..
                }) => Some(caption_entities),
                _ => None,
            }
        }

        pub fn audio(&self) -> Option<&types::Audio> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Audio(MediaAudio { audio, .. }),
                    ..
                }) => Some(audio),
                _ => None,
            }
        }

        pub fn document(&self) -> Option<&types::Document> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Document(MediaDocument { document, .. }),
                    ..
                }) => Some(document),
                _ => None,
            }
        }

        pub fn animation(&self) -> Option<&types::Animation> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Animation(MediaAnimation { animation, .. }),
                    ..
                }) => Some(animation),
                _ => None,
            }
        }

        pub fn game(&self) -> Option<&types::Game> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Game(MediaGame { game, .. }),
                    ..
                }) => Some(game),
                _ => None,
            }
        }

        pub fn photo(&self) -> Option<&[PhotoSize]> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Photo(MediaPhoto { photo, .. }),
                    ..
                }) => Some(photo),
                _ => None,
            }
        }

        pub fn sticker(&self) -> Option<&types::Sticker> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Sticker(MediaSticker { sticker, .. }),
                    ..
                }) => Some(sticker),
                _ => None,
            }
        }

        pub fn video(&self) -> Option<&types::Video> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Video(MediaVideo { video, .. }),
                    ..
                }) => Some(video),
                _ => None,
            }
        }

        pub fn voice(&self) -> Option<&types::Voice> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Voice(MediaVoice { voice, .. }),
                    ..
                }) => Some(voice),
                _ => None,
            }
        }

        pub fn video_note(&self) -> Option<&types::VideoNote> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::VideoNote(MediaVideoNote { video_note, .. }),
                    ..
                }) => Some(video_note),
                _ => None,
            }
        }

        pub fn caption(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon { media_kind, .. }) => match media_kind {
                    MediaKind::Animation(MediaAnimation { caption, .. })
                    | MediaKind::Audio(MediaAudio { caption, .. })
                    | MediaKind::Document(MediaDocument { caption, .. })
                    | MediaKind::Photo(MediaPhoto { caption, .. })
                    | MediaKind::Video(MediaVideo { caption, .. })
                    | MediaKind::Voice(MediaVoice { caption, .. }) => {
                        caption.as_ref().map(Deref::deref)
                    }
                    _ => None,
                },
                _ => None,
            }
        }

        pub fn contact(&self) -> Option<&types::Contact> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Contact(MediaContact { contact, .. }),
                    ..
                }) => Some(contact),
                _ => None,
            }
        }

        pub fn location(&self) -> Option<&types::Location> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Location(MediaLocation { location, .. }),
                    ..
                }) => Some(location),
                _ => None,
            }
        }

        pub fn venue(&self) -> Option<&types::Venue> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Venue(MediaVenue { venue, .. }),
                    ..
                }) => Some(venue),
                _ => None,
            }
        }

        pub fn poll(&self) -> Option<&types::Poll> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Poll(MediaPoll { poll, .. }),
                    ..
                }) => Some(poll),
                _ => None,
            }
        }

        pub fn new_chat_members(&self) -> Option<&[User]> {
            match &self.kind {
                NewChatMembers(MessageNewChatMembers { new_chat_members }) => {
                    Some(new_chat_members.as_ref())
                }
                _ => None,
            }
        }

        pub fn left_chat_member(&self) -> Option<&User> {
            match &self.kind {
                LeftChatMember(MessageLeftChatMember { left_chat_member }) => {
                    Some(left_chat_member)
                }
                _ => None,
            }
        }

        pub fn new_chat_title(&self) -> Option<&str> {
            match &self.kind {
                NewChatTitle(MessageNewChatTitle { new_chat_title }) => Some(new_chat_title),
                _ => None,
            }
        }

        pub fn new_chat_photo(&self) -> Option<&[PhotoSize]> {
            match &self.kind {
                NewChatPhoto(MessageNewChatPhoto { new_chat_photo }) => Some(new_chat_photo),
                _ => None,
            }
        }

        // TODO: OK, `Option<True>` is weird, can we do something with it?
        //       mb smt like `is_delete_chat_photo(&self) -> bool`?
        pub fn delete_chat_photo(&self) -> Option<True> {
            match &self.kind {
                DeleteChatPhoto(MessageDeleteChatPhoto { delete_chat_photo }) => {
                    Some(*delete_chat_photo)
                }
                _ => None,
            }
        }

        pub fn group_chat_created(&self) -> Option<True> {
            match &self.kind {
                GroupChatCreated(MessageGroupChatCreated { group_chat_created }) => {
                    Some(*group_chat_created)
                }
                _ => None,
            }
        }

        pub fn super_group_chat_created(&self) -> Option<True> {
            match &self.kind {
                SupergroupChatCreated(MessageSupergroupChatCreated {
                    supergroup_chat_created,
                }) => Some(*supergroup_chat_created),
                _ => None,
            }
        }

        pub fn channel_chat_created(&self) -> Option<True> {
            match &self.kind {
                ChannelChatCreated(MessageChannelChatCreated {
                    channel_chat_created,
                }) => Some(*channel_chat_created),
                _ => None,
            }
        }

        pub fn migrate_to_chat_id(&self) -> Option<i64> {
            match &self.kind {
                Migrate(MessageMigrate {
                    migrate_to_chat_id, ..
                }) => Some(*migrate_to_chat_id),
                _ => None,
            }
        }

        pub fn migrate_from_chat_id(&self) -> Option<i64> {
            match &self.kind {
                Migrate(MessageMigrate {
                    migrate_from_chat_id,
                    ..
                }) => Some(*migrate_from_chat_id),
                _ => None,
            }
        }

        pub fn pinned_message(&self) -> Option<&Message> {
            match &self.kind {
                Pinned(MessagePinned { pinned }) => Some(pinned),
                _ => None,
            }
        }

        pub fn invoice(&self) -> Option<&types::Invoice> {
            match &self.kind {
                Invoice(MessageInvoice { invoice }) => Some(invoice),
                _ => None,
            }
        }

        pub fn successful_payment(&self) -> Option<&types::SuccessfulPayment> {
            match &self.kind {
                SuccessfulPayment(MessageSuccessfulPayment { successful_payment }) => {
                    Some(successful_payment)
                }
                _ => None,
            }
        }

        pub fn connected_website(&self) -> Option<&str> {
            match &self.kind {
                ConnectedWebsite(MessageConnectedWebsite { connected_website }) => {
                    Some(connected_website)
                }
                _ => None,
            }
        }

        pub fn passport_data(&self) -> Option<&types::PassportData> {
            match &self.kind {
                PassportData(MessagePassportData { passport_data }) => Some(passport_data),
                _ => None,
            }
        }

        pub fn dice(&self) -> Option<&types::Dice> {
            match &self.kind {
                Dice(MessageDice { dice }) => Some(dice),
                _ => None,
            }
        }

        pub fn proximity_alert_triggered(&self) -> Option<&types::ProximityAlertTriggered> {
            match &self.kind {
                ProximityAlertTriggered(MessageProximityAlertTriggered {
                    proximity_alert_triggered,
                }) => Some(proximity_alert_triggered),
                _ => None,
            }
        }

        pub fn reply_markup(&self) -> Option<&types::InlineKeyboardMarkup> {
            match &self.kind {
                Common(MessageCommon { reply_markup, .. }) => reply_markup.as_ref(),
                _ => None,
            }
        }
    }
}

impl Message {
    /// Produces a direct link to the message.
    ///
    /// Note that for private groups the link will only be accesible for group
    /// members.
    ///
    /// Returns `None` for private chats (i.e.: DMs).
    pub fn url(&self) -> Option<reqwest::Url> {
        if self.chat.is_private() {
            // For private chats (i.e.: DMs) we can't produce "normal" t.me link.
            //
            // There are "tg://openmessage?user_id={0}&message_id={1}" links, which are
            // supposed to open any chat, including private messages, but they
            // are only supported by some telegram clients (e.g. Plus Messenger,
            // Telegram for Androud 4.9+).
            return None;
        }

        let url = match self.chat.username() {
            // If it's public group (i.e. not DM, not private group), we can produce
            // "normal" t.me link (accesible to everyone).
            Some(username) => format!("https://t.me/{0}/{1}/", username, self.id),
            // For private groups we produce "private" t.me/c links. These are only
            // accesible to the group members.
            None => format!("https://t.me/c/{0}/{1}/", self.chat.id, self.id),
        };

        // UNWRAP:
        //
        // The `url` produced by formatting is correct since username is
        // /[a-zA-Z0-9_]{5,32}/ and chat/message ids are integers.
        Some(reqwest::Url::parse(&url).unwrap())
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
