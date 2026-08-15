use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, Chat, Checklist, Contact, Dice, Document, Game, Giveaway, GiveawayWinners,
    Invoice, LinkPreviewOptions, Location, MessageId, MessageOrigin, PaidMediaInfo, PhotoSize,
    Poll, Sticker, Story, Venue, Video, VideoNote, Voice,
};

/// This object contains information about a message that is being replied to,
/// which may come from another chat or forum topic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(from = "ExternalReplyInfoRaw")]
pub struct ExternalReplyInfo {
    /// Origin of the message replied to by the given message.
    pub origin: MessageOrigin,
    /// Chat the original message belongs to. Available only if the chat is a
    /// supergroup or a channel.
    pub chat: Option<Chat>,
    /// Unique message identifier inside the original chat. Available only if
    /// the original chat is a supergroup or a channel.
    #[serde(with = "crate::types::option_msg_id_as_int")]
    #[cfg_attr(test, schemars(with = "Option<i32>"))]
    pub message_id: Option<MessageId>,
    /// Options used for link preview generation for the original message, if it
    /// is a text message.
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// _true_, if the message media is covered by a spoiler animation.
    #[serde(default)]
    pub has_media_spoiler: bool,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ExternalReplyInfoKind>,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
struct ExternalReplyInfoRaw {
    origin: MessageOrigin,
    #[serde(default)]
    chat: Option<Chat>,
    #[serde(default, with = "crate::types::option_msg_id_as_int")]
    #[cfg_attr(test, schemars(with = "Option<i32>"))]
    message_id: Option<MessageId>,
    #[serde(default)]
    link_preview_options: Option<LinkPreviewOptions>,
    #[serde(default)]
    has_media_spoiler: bool,

    #[serde(flatten)]
    kind: serde_json::Value,
}

impl From<ExternalReplyInfoRaw> for ExternalReplyInfo {
    fn from(raw: ExternalReplyInfoRaw) -> Self {
        Self {
            origin: raw.origin,
            chat: raw.chat,
            message_id: raw.message_id,
            link_preview_options: raw.link_preview_options,
            has_media_spoiler: raw.has_media_spoiler,
            kind: serde_json::from_value(raw.kind).ok(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ExternalReplyInfoKind {
    // Note:
    // - `Venue` must be in front of `Location`
    // - `Animation` must be in front of `Document`
    //
    // This is needed so serde doesn't parse `Venue` as `Location` or `Animation` as `Document`
    // (for backward compatability telegram duplicates some fields).
    //
    // See <https://github.com/teloxide/teloxide/issues/481>
    Animation(Animation),
    Audio(Audio),
    Contact(Contact),
    Dice(Dice),
    Document(Document),
    PaidMedia(PaidMediaInfo),
    Game(Game),
    Venue(Venue),
    Location(Location),
    Photo(Vec<PhotoSize>),
    Poll(Poll),
    Checklist(Checklist),
    Sticker(Sticker),
    Story(Story),
    Giveaway(Giveaway),
    GiveawayWinners(GiveawayWinners),
    Video(Video),
    VideoNote(VideoNote),
    Voice(Voice),
    Invoice(Invoice),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_text_only_quote_without_media() {
        let data = r#"
        {
            "origin": {
                "type": "hidden_user",
                "date": 1721161230,
                "sender_user_name": "Test"
            },
            "chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "message_id": 1,
            "link_preview_options": null,
            "has_media_spoiler": false
        }
        "#;

        let info: ExternalReplyInfo = serde_json::from_str(data).unwrap();
        assert_eq!(info.kind, None);
    }

    #[test]
    fn deserialize_quote_with_media() {
        let data = r#"
        {
            "origin": {
                "type": "hidden_user",
                "date": 1721161230,
                "sender_user_name": "Test"
            },
            "has_media_spoiler": false,
            "voice": {
                "file_id": "AwADBAADbXXXXXXXXXXXGBdhD2l6_XX",
                "file_unique_id": "AgADAgADbXXXXXXXXXXXGBdhD2l6_XX",
                "duration": 60,
                "mime_type": "audio/ogg"
            }
        }
        "#;

        let info: ExternalReplyInfo = serde_json::from_str(data).unwrap();
        assert!(matches!(info.kind, Some(ExternalReplyInfoKind::Voice(_))));
    }

    #[test]
    fn serialize_roundtrip_without_kind() {
        let info = ExternalReplyInfo {
            origin: MessageOrigin::HiddenUser {
                date: chrono::DateTime::from_timestamp(1721161230, 0).unwrap(),
                sender_user_name: "Test".to_owned(),
            },
            chat: None,
            message_id: None,
            link_preview_options: None,
            has_media_spoiler: false,
            kind: None,
        };

        let json = serde_json::to_value(&info).unwrap();
        assert!(json.as_object().unwrap().keys().all(|k| k != "voice"));

        let deserialized: ExternalReplyInfo = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, info);
    }
}
