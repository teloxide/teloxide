#![allow(clippy::large_enum_variant)]

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::types::{
    InlineQueryResultArticle, InlineQueryResultAudio, InlineQueryResultCachedAudio,
    InlineQueryResultCachedDocument, InlineQueryResultCachedGif, InlineQueryResultCachedMpeg4Gif,
    InlineQueryResultCachedPhoto, InlineQueryResultCachedSticker, InlineQueryResultCachedVideo,
    InlineQueryResultCachedVoice, InlineQueryResultContact, InlineQueryResultDocument,
    InlineQueryResultGame, InlineQueryResultGif, InlineQueryResultLocation,
    InlineQueryResultMpeg4Gif, InlineQueryResultPhoto, InlineQueryResultVenue,
    InlineQueryResultVideo, InlineQueryResultVoice,
};

/// This object represents one result of an inline query.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresult).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[serde(from = "raw::InlineQueryResult", into = "raw::InlineQueryResult")]
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

mod raw {
    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum AudioKind {
        Cached(InlineQueryResultCachedAudio),
        NonCached(InlineQueryResultAudio),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum DocumentKind {
        Cached(InlineQueryResultCachedDocument),
        NonCached(InlineQueryResultDocument),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum GifKind {
        Cached(InlineQueryResultCachedGif),
        NonCached(InlineQueryResultGif),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum Mpeg4GifKind {
        Cached(InlineQueryResultCachedMpeg4Gif),
        NonCached(InlineQueryResultMpeg4Gif),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum PhotoKind {
        Cached(InlineQueryResultCachedPhoto),
        NonCached(InlineQueryResultPhoto),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum VideoKind {
        Cached(InlineQueryResultCachedVideo),
        NonCached(InlineQueryResultVideo),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum VoiceKind {
        Cached(InlineQueryResultCachedVoice),
        NonCached(InlineQueryResultVoice),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "type")]
    #[serde(rename_all = "snake_case")]
    pub(super) enum InlineQueryResult {
        // Types which have a cached and non-cached variant must be listed here
        Audio(AudioKind),
        Document(DocumentKind),
        Gif(GifKind),
        #[serde(rename = "mpeg4_gif")]
        Mpeg4Gif(Mpeg4GifKind),
        Photo(PhotoKind),
        Video(VideoKind),
        Voice(VoiceKind),

        // Types which have only a cached variant must be listed here
        #[serde(rename = "sticker")]
        CachedSticker(InlineQueryResultCachedSticker),

        // Types which have only a non-cached variant must be listed here
        Article(InlineQueryResultArticle),
        Contact(InlineQueryResultContact),
        Game(InlineQueryResultGame),
        Location(InlineQueryResultLocation),
        Venue(InlineQueryResultVenue),
    }

    impl From<InlineQueryResult> for super::InlineQueryResult {
        fn from(raw: InlineQueryResult) -> Self {
            match raw {
                InlineQueryResult::Audio(AudioKind::Cached(audio)) => {
                    super::InlineQueryResult::CachedAudio(audio)
                }
                InlineQueryResult::Audio(AudioKind::NonCached(audio)) => {
                    super::InlineQueryResult::Audio(audio)
                }
                InlineQueryResult::Document(DocumentKind::Cached(document)) => {
                    super::InlineQueryResult::CachedDocument(document)
                }
                InlineQueryResult::Document(DocumentKind::NonCached(document)) => {
                    super::InlineQueryResult::Document(document)
                }
                InlineQueryResult::Gif(GifKind::Cached(gif)) => {
                    super::InlineQueryResult::CachedGif(gif)
                }
                InlineQueryResult::Gif(GifKind::NonCached(gif)) => {
                    super::InlineQueryResult::Gif(gif)
                }
                InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::Cached(gif)) => {
                    super::InlineQueryResult::CachedMpeg4Gif(gif)
                }
                InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::NonCached(gif)) => {
                    super::InlineQueryResult::Mpeg4Gif(gif)
                }
                InlineQueryResult::Photo(PhotoKind::Cached(photo)) => {
                    super::InlineQueryResult::CachedPhoto(photo)
                }
                InlineQueryResult::Photo(PhotoKind::NonCached(photo)) => {
                    super::InlineQueryResult::Photo(photo)
                }
                InlineQueryResult::Video(VideoKind::Cached(video)) => {
                    super::InlineQueryResult::CachedVideo(video)
                }
                InlineQueryResult::Video(VideoKind::NonCached(video)) => {
                    super::InlineQueryResult::Video(video)
                }
                InlineQueryResult::Voice(VoiceKind::Cached(voice)) => {
                    super::InlineQueryResult::CachedVoice(voice)
                }
                InlineQueryResult::Voice(VoiceKind::NonCached(voice)) => {
                    super::InlineQueryResult::Voice(voice)
                }

                InlineQueryResult::CachedSticker(sticker) => {
                    super::InlineQueryResult::CachedSticker(sticker)
                }

                InlineQueryResult::Article(article) => super::InlineQueryResult::Article(article),
                InlineQueryResult::Contact(contact) => super::InlineQueryResult::Contact(contact),
                InlineQueryResult::Game(game) => super::InlineQueryResult::Game(game),
                InlineQueryResult::Location(location) => {
                    super::InlineQueryResult::Location(location)
                }
                InlineQueryResult::Venue(venue) => super::InlineQueryResult::Venue(venue),
            }
        }
    }

    impl From<super::InlineQueryResult> for InlineQueryResult {
        fn from(raw: super::InlineQueryResult) -> Self {
            match raw {
                super::InlineQueryResult::CachedAudio(audio) => {
                    InlineQueryResult::Audio(AudioKind::Cached(audio))
                }
                super::InlineQueryResult::Audio(audio) => {
                    InlineQueryResult::Audio(AudioKind::NonCached(audio))
                }
                super::InlineQueryResult::CachedDocument(document) => {
                    InlineQueryResult::Document(DocumentKind::Cached(document))
                }
                super::InlineQueryResult::Document(document) => {
                    InlineQueryResult::Document(DocumentKind::NonCached(document))
                }
                super::InlineQueryResult::CachedGif(gif) => {
                    InlineQueryResult::Gif(GifKind::Cached(gif))
                }
                super::InlineQueryResult::Gif(gif) => {
                    InlineQueryResult::Gif(GifKind::NonCached(gif))
                }
                super::InlineQueryResult::CachedMpeg4Gif(gif) => {
                    InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::Cached(gif))
                }
                super::InlineQueryResult::Mpeg4Gif(gif) => {
                    InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::NonCached(gif))
                }
                super::InlineQueryResult::CachedPhoto(photo) => {
                    InlineQueryResult::Photo(PhotoKind::Cached(photo))
                }
                super::InlineQueryResult::Photo(photo) => {
                    InlineQueryResult::Photo(PhotoKind::NonCached(photo))
                }
                super::InlineQueryResult::CachedVideo(video) => {
                    InlineQueryResult::Video(VideoKind::Cached(video))
                }
                super::InlineQueryResult::Video(video) => {
                    InlineQueryResult::Video(VideoKind::NonCached(video))
                }
                super::InlineQueryResult::CachedVoice(voice) => {
                    InlineQueryResult::Voice(VoiceKind::Cached(voice))
                }
                super::InlineQueryResult::Voice(voice) => {
                    InlineQueryResult::Voice(VoiceKind::NonCached(voice))
                }

                super::InlineQueryResult::CachedSticker(sticker) => {
                    InlineQueryResult::CachedSticker(sticker)
                }

                super::InlineQueryResult::Article(article) => InlineQueryResult::Article(article),
                super::InlineQueryResult::Contact(contact) => InlineQueryResult::Contact(contact),
                super::InlineQueryResult::Game(game) => InlineQueryResult::Game(game),
                super::InlineQueryResult::Location(location) => {
                    InlineQueryResult::Location(location)
                }
                super::InlineQueryResult::Venue(venue) => InlineQueryResult::Venue(venue),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        inline_keyboard_markup::InlineKeyboardMarkup, parse_mode::ParseMode, InlineQueryResult,
        InlineQueryResultAudio, InlineQueryResultCachedAudio, InputMessageContent,
        InputMessageContentText,
    };

    #[test]
    fn cached_audio_min_serialize() {
        let structure = InlineQueryResult::CachedAudio(InlineQueryResultCachedAudio {
            id: String::from("id"),
            audio_file_id: String::from("audio_file_id"),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"audio","id":"id","audio_file_id":"audio_file_id"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);

        let structure: InlineQueryResult = serde_json::from_str(&actual_json).unwrap();

        assert!(matches!(structure, InlineQueryResult::CachedAudio(_)));
    }

    #[test]
    fn cached_audio_full_serialize() {
        let structure = InlineQueryResult::CachedAudio(InlineQueryResultCachedAudio {
            id: String::from("id"),
            audio_file_id: String::from("audio_file_id"),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                disable_web_page_preview: Some(true),
                entities: None,
            })),
            caption_entities: None,
        });

        let expected_json = r#"{"type":"audio","id":"id","audio_file_id":"audio_file_id","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","disable_web_page_preview":true}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);

        let structure: InlineQueryResult = serde_json::from_str(&actual_json).unwrap();

        assert!(matches!(structure, InlineQueryResult::CachedAudio(_)));
    }

    #[test]
    fn audio_min_serialize() {
        let structure = InlineQueryResult::Audio(InlineQueryResultAudio {
            id: String::from("id"),
            audio_url: reqwest::Url::parse("http://audio_url/").unwrap(),
            title: String::from("title"),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            performer: None,
            audio_duration: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json =
            r#"{"type":"audio","id":"id","audio_url":"http://audio_url/","title":"title"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);

        let structure: InlineQueryResult = serde_json::from_str(&actual_json).unwrap();

        assert!(matches!(structure, InlineQueryResult::Audio(_)));
    }

    #[test]
    fn audio_full_serialize() {
        let structure = InlineQueryResult::Audio(InlineQueryResultAudio {
            id: String::from("id"),
            audio_url: reqwest::Url::parse("http://audio_url/").unwrap(),
            title: String::from("title"),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                disable_web_page_preview: Some(true),
                entities: None,
            })),
            caption_entities: None,
            performer: Some(String::from("performer")),
            audio_duration: Some("1".into()),
        });

        let expected_json = r#"{"type":"audio","id":"id","audio_url":"http://audio_url/","title":"title","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","disable_web_page_preview":true},"performer":"performer","audio_duration":1}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);

        let structure: InlineQueryResult = serde_json::from_str(&actual_json).unwrap();

        assert!(matches!(structure, InlineQueryResult::Audio(_)));
    }

    // TODO: Add more tests
}
