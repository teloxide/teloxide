#![allow(clippy::large_enum_variant)]

use serde::{Deserialize, Serialize};

use crate::types::{
    chat::{ChatKind, PublicChatKind},
    Animation, Audio, Chat, ChatPublic, Contact, Document, Game, InlineKeyboardMarkup, Invoice,
    Location, MessageEntity, PassportData, PhotoSize, Poll, PublicChatChannel,
    PublicChatSupergroup, Sticker, SuccessfulPayment, True, User, Venue, Video, VideoNote, Voice,
};

/// This object represents a message.
///
/// [The official docs](https://core.telegram.org/bots/api#message).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl Message {
    pub fn new(id: i32, date: i32, chat: Chat, kind: MessageKind) -> Self {
        Self { id, date, chat, kind }
    }

    pub fn id(mut self, val: i32) -> Self {
        self.id = val;
        self
    }

    pub fn date(mut self, val: i32) -> Self {
        self.date = val;
        self
    }

    pub fn chat(mut self, val: Chat) -> Self {
        self.chat = val;
        self
    }

    pub fn kind(mut self, val: MessageKind) -> Self {
        self.kind = val;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
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
    Migrate(MessageMigrate),
    Pinned(MessagePinned),
    Invoice(MessageInvoice),
    SuccessfulPayment(MessageSuccessfulPayment),
    ConnectedWebsite(MessageConnectedWebsite),
    PassportData(MessagePassportData),
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageCommon {
    /// Sender, empty for messages sent to channels.
    pub from: Option<User>,

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

impl MessageCommon {
    pub fn new(forward_kind: ForwardKind, media_kind: MediaKind) -> Self {
        Self { from: None, forward_kind, edit_date: None, media_kind, reply_markup: None }
    }

    pub fn from(mut self, val: User) -> Self {
        self.from = Some(val);
        self
    }

    pub fn forward_kind(mut self, val: ForwardKind) -> Self {
        self.forward_kind = val;
        self
    }

    pub fn edit_date(mut self, val: i32) -> Self {
        self.edit_date = Some(val);
        self
    }

    pub fn media_kind(mut self, val: MediaKind) -> Self {
        self.media_kind = val;
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageNewChatMembers {
    /// New members that were added to the group or supergroup and
    /// information about them (the bot itself may be one of these
    /// members).
    pub new_chat_members: Vec<User>,
}

impl MessageNewChatMembers {
    pub fn new<N>(new_chat_members: N) -> Self
    where
        N: Into<Vec<User>>,
    {
        Self { new_chat_members: new_chat_members.into() }
    }

    pub fn new_chat_members<N>(mut self, val: N) -> Self
    where
        N: Into<Vec<User>>,
    {
        self.new_chat_members = val.into();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageLeftChatMember {
    /// A member was removed from the group, information about them (this
    /// member may be the bot itself).
    pub left_chat_member: User,
}

impl MessageLeftChatMember {
    pub fn new<N>(left_chat_member: N) -> Self
    where
        N: Into<User>,
    {
        Self { left_chat_member: left_chat_member.into() }
    }

    pub fn left_chat_member<N>(mut self, val: N) -> Self
    where
        N: Into<User>,
    {
        self.left_chat_member = val.into();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageNewChatTitle {
    /// A chat title was changed to this value.
    pub new_chat_title: String,
}

impl MessageNewChatTitle {
    pub fn new<N>(new_chat_title: N) -> Self
    where
        N: Into<String>,
    {
        Self { new_chat_title: new_chat_title.into() }
    }

    pub fn new_chat_title<N>(mut self, val: N) -> Self
    where
        N: Into<String>,
    {
        self.new_chat_title = val.into();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageNewChatPhoto {
    /// A chat photo was change to this value.
    pub new_chat_photo: Vec<PhotoSize>,
}

impl MessageNewChatPhoto {
    pub fn new<N>(new_chat_photo: N) -> Self
    where
        N: Into<Vec<PhotoSize>>,
    {
        Self { new_chat_photo: new_chat_photo.into() }
    }

    pub fn new_chat_photo<N>(mut self, val: N) -> Self
    where
        N: Into<Vec<PhotoSize>>,
    {
        self.new_chat_photo = val.into();
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageDeleteChatPhoto {
    /// Service message: the chat photo was deleted.
    pub delete_chat_photo: True,
}

impl MessageDeleteChatPhoto {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageGroupChatCreated {
    /// Service message: the group has been created.
    pub group_chat_created: True,
}

impl MessageGroupChatCreated {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageSupergroupChatCreated {
    /// Service message: the supergroup has been created. This field can‘t
    /// be received in a message coming through updates, because bot can’t
    /// be a member of a supergroup when it is created. It can only be
    /// found in `reply_to_message` if someone replies to a very first
    /// message in a directly created supergroup.
    pub supergroup_chat_created: True,
}

impl MessageSupergroupChatCreated {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageChannelChatCreated {
    /// Service message: the channel has been created. This field can‘t be
    /// received in a message coming through updates, because bot can’t be
    /// a member of a channel when it is created. It can only be found in
    /// `reply_to_message` if someone replies to a very first message in a
    /// channel.
    pub channel_chat_created: True,
}

impl MessageChannelChatCreated {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl MessageMigrate {
    pub fn new(migrate_to_chat_id: i64, migrate_from_chat_id: i64) -> Self {
        Self { migrate_to_chat_id, migrate_from_chat_id }
    }

    pub fn migrate_to_chat_id(mut self, val: i64) -> Self {
        self.migrate_to_chat_id = val;
        self
    }

    pub fn migrate_from_chat_id(mut self, val: i64) -> Self {
        self.migrate_from_chat_id = val;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessagePinned {
    /// Specified message was pinned. Note that the Message object in this
    /// field will not contain further `reply_to_message` fields even if it
    /// is itself a reply.
    #[serde(rename = "pinned_message")]
    pub pinned: Box<Message>,
}

impl MessagePinned {
    pub fn new(pinned: Message) -> Self {
        Self { pinned: Box::new(pinned) }
    }

    pub fn pinned(mut self, val: Message) -> Self {
        self.pinned = Box::new(val);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageInvoice {
    /// Message is an invoice for a [payment], information about the
    /// invoice. [More about payments »].
    ///
    /// [payment]: https://core.telegram.org/bots/api#payments
    /// [More about payments »]: https://core.telegram.org/bots/api#payments
    pub invoice: Invoice,
}

impl MessageInvoice {
    pub fn new(invoice: Invoice) -> Self {
        Self { invoice }
    }

    pub fn invoice(mut self, val: Invoice) -> Self {
        self.invoice = val;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageSuccessfulPayment {
    /// Message is a service message about a successful payment,
    /// information about the payment. [More about payments »].
    ///
    /// [More about payments »]: https://core.telegram.org/bots/api#payments
    pub successful_payment: SuccessfulPayment,
}

impl MessageSuccessfulPayment {
    pub fn new(successful_payment: SuccessfulPayment) -> Self {
        Self { successful_payment }
    }

    pub fn successful_payment(mut self, val: SuccessfulPayment) -> Self {
        self.successful_payment = val;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageConnectedWebsite {
    /// The domain name of the website on which the user has logged in.
    /// [More about Telegram Login »].
    ///
    /// [More about Telegram Login »]: https://core.telegram.org/widgets/login
    pub connected_website: String,
}

impl MessageConnectedWebsite {
    pub fn new<S>(connected_website: S) -> Self
    where
        S: Into<String>,
    {
        Self { connected_website: connected_website.into() }
    }

    pub fn connected_website<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.connected_website = val.into();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessagePassportData {
    /// Telegram Passport data.
    pub passport_data: PassportData,
}

impl MessagePassportData {
    pub fn new(passport_data: PassportData) -> Self {
        Self { passport_data }
    }

    pub fn passport_data(mut self, val: PassportData) -> Self {
        self.passport_data = val;
        self
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ForwardedFrom {
    #[serde(rename = "forward_from")]
    User(User),
    #[serde(rename = "forward_sender_name")]
    SenderName(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ForwardKind {
    Channel(ForwardChannel),
    NonChannel(ForwardNonChannel),
    Origin(ForwardOrigin),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl ForwardChannel {
    pub fn new(date: i32, chat: Chat, message_id: i32) -> Self {
        Self { date, chat, message_id, signature: None }
    }

    pub fn date(mut self, val: i32) -> Self {
        self.date = val;
        self
    }

    pub fn chat(mut self, val: Chat) -> Self {
        self.chat = val;
        self
    }

    pub fn message_id(mut self, val: i32) -> Self {
        self.message_id = val;
        self
    }

    pub fn signature<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.signature = Some(val.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForwardNonChannel {
    #[serde(rename = "forward_date")]
    pub date: i32,

    #[serde(flatten)]
    pub from: ForwardedFrom,
}

impl ForwardNonChannel {
    pub fn new(date: i32, from: ForwardedFrom) -> Self {
        Self { date, from }
    }

    pub fn date(mut self, val: i32) -> Self {
        self.date = val;
        self
    }

    pub fn from(mut self, val: ForwardedFrom) -> Self {
        self.from = val;
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForwardOrigin {
    pub reply_to_message: Option<Box<Message>>,
}

impl ForwardOrigin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reply_to_message(mut self, val: Message) -> Self {
        self.reply_to_message = Some(Box::new(val));
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
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
#[non_exhaustive]
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

impl MediaAnimation {
    pub fn new<CE>(animation: Animation, caption_entities: CE) -> Self
    where
        CE: Into<Vec<MessageEntity>>,
    {
        Self { animation, document: (), caption: None, caption_entities: caption_entities.into() }
    }

    pub fn animation(mut self, val: Animation) -> Self {
        self.animation = val;
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn caption_entities<CE>(mut self, val: CE) -> Self
    where
        CE: Into<Vec<MessageEntity>>,
    {
        self.caption_entities = val.into();
        self
    }
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl MediaAudio {
    pub fn new<CE>(audio: Audio, caption_entities: CE) -> Self
    where
        CE: Into<Vec<MessageEntity>>,
    {
        Self { audio, caption: None, caption_entities: caption_entities.into() }
    }

    pub fn audio(mut self, val: Audio) -> Self {
        self.audio = val;
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn caption_entities<CE>(mut self, val: CE) -> Self
    where
        CE: Into<Vec<MessageEntity>>,
    {
        self.caption_entities = val.into();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MediaContact {
    /// Message is a shared contact, information about the contact.
    contact: Contact,
}

impl MediaContact {
    pub fn new(contact: Contact) -> Self {
        Self { contact }
    }

    pub fn contact(mut self, val: Contact) -> Self {
        self.contact = val;
        self
    }
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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

impl MediaDocument {
    pub fn new<CE>(document: Document, caption_entities: CE) -> Self
    where
        CE: Into<Vec<MessageEntity>>,
    {
        Self { document, caption: None, caption_entities: caption_entities.into() }
    }

    pub fn document(mut self, val: Document) -> Self {
        self.document = val;
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn caption_entities<CE>(mut self, val: CE) -> Self
    where
        CE: Into<Vec<MessageEntity>>,
    {
        self.caption_entities = val.into();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MediaGame {
    /// Message is a game, information about the game. [More
    /// about games »].
    ///
    /// [More about games »]: https://core.telegram.org/bots/api#games
    pub game: Game,
}

impl MediaGame {
    pub fn new(game: Game) -> Self {
        Self { game }
    }

    pub fn game(mut self, val: Game) -> Self {
        self.game = val;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MediaLocation {
    /// Message is a shared location, information about the location.
    pub location: Location,
}

impl MediaLocation {
    pub fn new(location: Location) -> Self {
        Self { location }
    }

    pub fn location(mut self, val: Location) -> Self {
        self.location = val;
        self
    }
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct MediaPoll {
    /// Message is a native poll, information about the poll.
    pub poll: Poll,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MediaSticker {
    /// Message is a sticker, information about the sticker.
    pub sticker: Sticker,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
pub struct MediaVideoNote {
    /// Message is a [video note], information about the video message.
    ///
    /// [video note]: https://telegram.org/blog/video-messages-and-telescope
    pub video_note: VideoNote,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct MediaVenue {
    /// Message is a venue, information about the venue.
    pub venue: Venue,
}

mod getters {
    use std::ops::Deref;

    use crate::types::{
        self,
        message::{
            ForwardKind::NonChannel,
            MessageKind::{
                ChannelChatCreated, Common, ConnectedWebsite, DeleteChatPhoto, GroupChatCreated,
                Invoice, LeftChatMember, Migrate, NewChatMembers, NewChatPhoto, NewChatTitle,
                PassportData, Pinned, SuccessfulPayment, SupergroupChatCreated,
            },
        },
        Chat, ForwardChannel, ForwardKind, ForwardNonChannel, ForwardOrigin, ForwardedFrom,
        MediaAnimation, MediaAudio, MediaContact, MediaDocument, MediaGame, MediaKind,
        MediaLocation, MediaPhoto, MediaPoll, MediaSticker, MediaText, MediaVenue, MediaVideo,
        MediaVideoNote, MediaVoice, Message, MessageChannelChatCreated, MessageCommon,
        MessageConnectedWebsite, MessageDeleteChatPhoto, MessageEntity, MessageGroupChatCreated,
        MessageInvoice, MessageLeftChatMember, MessageMigrate, MessageNewChatMembers,
        MessageNewChatPhoto, MessageNewChatTitle, MessagePassportData, MessagePinned,
        MessageSuccessfulPayment, MessageSupergroupChatCreated, PhotoSize, True, User,
    };

    /// Getters for [Message] fields from [telegram docs].
    ///
    /// [Message]: crate::types::Message
    /// [telegram docs]: https://core.telegram.org/bots/api#message
    impl Message {
        /// NOTE: this is getter for both `from` and `author_signature`
        pub fn from(&self) -> Option<&User> {
            match &self.kind {
                Common(MessageCommon { from, .. }) => from.as_ref(),
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
                    forward_kind: ForwardKind::Origin(ForwardOrigin { reply_to_message, .. }),
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

        pub fn text_owned(&self) -> Option<String> {
            self.text().map(ToOwned::to_owned)
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
                    media_kind: MediaKind::Animation(MediaAnimation { caption_entities, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Audio(MediaAudio { caption_entities, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Document(MediaDocument { caption_entities, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Photo(MediaPhoto { caption_entities, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Video(MediaVideo { caption_entities, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Voice(MediaVoice { caption_entities, .. }),
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
                SupergroupChatCreated(MessageSupergroupChatCreated { supergroup_chat_created }) => {
                    Some(*supergroup_chat_created)
                }
                _ => None,
            }
        }

        pub fn channel_chat_created(&self) -> Option<True> {
            match &self.kind {
                ChannelChatCreated(MessageChannelChatCreated { channel_chat_created }) => {
                    Some(*channel_chat_created)
                }
                _ => None,
            }
        }

        pub fn migrate_to_chat_id(&self) -> Option<i64> {
            match &self.kind {
                Migrate(MessageMigrate { migrate_to_chat_id, .. }) => Some(*migrate_to_chat_id),
                _ => None,
            }
        }

        pub fn migrate_from_chat_id(&self) -> Option<i64> {
            match &self.kind {
                Migrate(MessageMigrate { migrate_from_chat_id, .. }) => Some(*migrate_from_chat_id),
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

        pub fn reply_markup(&self) -> Option<&types::InlineKeyboardMarkup> {
            match &self.kind {
                Common(MessageCommon { reply_markup, .. }) => reply_markup.as_ref(),
                _ => None,
            }
        }
    }
}

impl Message {
    pub fn url(&self) -> Option<reqwest::Url> {
        match &self.chat.kind {
            ChatKind::Public(ChatPublic {
                kind: PublicChatKind::Channel(PublicChatChannel { username: Some(username) }),
                ..
            })
            | ChatKind::Public(ChatPublic {
                kind:
                    PublicChatKind::Supergroup(PublicChatSupergroup {
                        username: Some(username), ..
                    }),
                ..
            }) => Some(
                reqwest::Url::parse(format!("https://t.me/{0}/{1}/", username, self.id).as_str())
                    .unwrap(),
            ),
            _ => None,
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
