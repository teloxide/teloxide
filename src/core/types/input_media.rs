use serde::Deserialize;

use crate::core::types::InputFile;

pub enum InputMedia {
    InputMediaPhoto(InputMediaPhoto),
    InputMediaVideo(InputMediaVideo),
    InputMediaAnimation(InputMediaAnimation),
    InputMediaAudio(InputMediaAudiotype),
    InputMediaDocument(InputMediaDocument),
}

pub enum ThumbKind {
    InputFile,
    String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  InputMediaPhoto {
    type_: String,
    media: String,
    caption: Option<String>,
    parse_mode: Option<String>,
}

#[derive(Debug, Serialize), Deserialize]
pub struct InputMediaVideo {
    type_: String,
    media: String,
    thumb: ThumbKind,
    caption: Option<String>,
    parse_mode: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    duration: Option<i64>,
    supports_streaming: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMediaAnimation {
    type_: String,
    media: String,
    thumb: Option<ThumbKind>,
    caption: Option<String>,
    parse_mode: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    duration: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMediaAudio {
    type_: String,
    media: String,
    thumb: Option<ThumbKind>,
    caption: Option<String>,
    parse_mode: Option<String>,
    duration: Option<i64>,
    performer: Option<i64>,
    title: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMediaDocument {
    type_: String,
    media: String,
    thumb: Option<ThumbKind>,
    caption: Option<String>,
    parse_mode: parse_mode,
}