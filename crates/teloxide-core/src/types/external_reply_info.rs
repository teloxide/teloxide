use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Chat, Contact, Dice, Document, Game, Giveaway, GiveawayWinners, Invoice,
    LinkPreviewOptions, Location, MessageId, MessageOrigin, PhotoSize, Poll, Sticker, Story, Venue,
    Video, VideoNote, Voice,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalReplyInfo {
    /// Origin of the message replied to by the given message
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a
    /// supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if
    /// the original chat is a supergroup or a channel.
    #[serde(with = "crate::types::option_msg_id_as_int")]
    pub message_id: Option<MessageId>,
    /// Options used for link preview generation for the original message, if it
    /// is a text message
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// _true_, if the message media is covered by a spoiler animation
    #[serde(default)]
    pub has_media_spoiler: bool,

    #[serde(flatten)]
    pub kind: ExternalReplyInfoKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalReplyInfoKind {
    // Note:
    // - `Venue` must be in front of `Location`
    // - `Animation` must be in front of `Document`
    //
    // This is needed so serde doesn't parse `Venue` as `Location` or `Animation` as `Document`
    // (for backward compatability telegram duplicates some fields)
    //
    // See <https://github.com/teloxide/teloxide/issues/481>
    Animation(Animation),
    Audio(Audio),
    Contact(Contact),
    Dice(Dice),
    Document(Document),
    Game(Game),
    Venue(Venue),
    Location(Location),
    Photo(Vec<PhotoSize>),
    Poll(Poll),
    Sticker(Sticker),
    Story(Story),
    Giveaway(Giveaway),
    GiveawayWinners(GiveawayWinners),
    Video(Video),
    VideoNote(VideoNote),
    Voice(Voice),
    Invoice(Invoice),
}
