#![allow(clippy::large_enum_variant)]

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::types::{
    InlineQueryResultArticle, InlineQueryResultAudio,
    InlineQueryResultCachedAudio, InlineQueryResultCachedDocument,
    InlineQueryResultCachedGif, InlineQueryResultCachedMpeg4Gif,
    InlineQueryResultCachedPhoto, InlineQueryResultCachedSticker,
    InlineQueryResultCachedVideo, InlineQueryResultCachedVoice,
    InlineQueryResultContact, InlineQueryResultDocument, InlineQueryResultGame,
    InlineQueryResultGif, InlineQueryResultLocation, InlineQueryResultMpeg4Gif,
    InlineQueryResultPhoto, InlineQueryResultVenue, InlineQueryResultVideo,
    InlineQueryResultVoice,
};

/// This object represents one result of an inline query.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresult).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, From)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InlineQueryResult {
    #[serde(rename = "audio")]
    CachedAudio(InlineQueryResultCachedAudio),
    #[serde(rename = "document")]
    CachedDocument(InlineQueryResultCachedDocument),
    #[serde(rename = "gif")]
    CachedGif(InlineQueryResultCachedGif),
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    #[serde(rename = "photo")]
    CachedPhoto(InlineQueryResultCachedPhoto),
    #[serde(rename = "sticker")]
    CachedSticker(InlineQueryResultCachedSticker),
    #[serde(rename = "video")]
    CachedVideo(InlineQueryResultCachedVideo),
    #[serde(rename = "voice")]
    CachedVoice(InlineQueryResultCachedVoice),

    Article(InlineQueryResultArticle),
    Audio(InlineQueryResultAudio),
    Contact(InlineQueryResultContact),
    Game(InlineQueryResultGame),
    Document(InlineQueryResultDocument),
    Gif(InlineQueryResultGif),
    Location(InlineQueryResultLocation),
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    Photo(InlineQueryResultPhoto),
    Venue(InlineQueryResultVenue),
    Video(InlineQueryResultVideo),
    Voice(InlineQueryResultVoice),
}

#[cfg(test)]
mod tests {
    use crate::types::{
        inline_keyboard_markup::InlineKeyboardMarkup, parse_mode::ParseMode,
        InlineQueryResult, InlineQueryResultCachedAudio, InputMessageContent,
    };

    #[test]
    fn cached_audio_min_serialize() {
        let structure =
            InlineQueryResult::CachedAudio(InlineQueryResultCachedAudio {
                id: String::from("id"),
                audio_file_id: String::from("audio_file_id"),
                caption: None,
                parse_mode: None,
                reply_markup: None,
                input_message_content: None,
            });

        let expected_json =
            r#"{"type":"audio","id":"id","audio_file_id":"audio_file_id"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn cached_audio_full_serialize() {
        let structure =
            InlineQueryResult::CachedAudio(InlineQueryResultCachedAudio {
                id: String::from("id"),
                audio_file_id: String::from("audio_file_id"),
                caption: Some(String::from("caption")),
                parse_mode: Some(ParseMode::HTML),
                reply_markup: Some(InlineKeyboardMarkup::new()),
                input_message_content: Some(InputMessageContent::Text {
                    message_text: String::from("message_text"),
                    parse_mode: Some(ParseMode::MarkdownV2),
                    disable_web_page_preview: Some(true),
                }),
            });

        let expected_json = r#"{"type":"audio","id":"id","audio_file_id":"audio_file_id","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","disable_web_page_preview":true}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
    }

    // TODO: Add more tests
}
