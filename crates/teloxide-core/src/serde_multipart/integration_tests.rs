use std::collections::{BTreeMap, BTreeSet};

use http_body_util::BodyExt as _;
use reqwest::{header::CONTENT_TYPE, Client};
use serde_json::Value;

use super::to_form_ref;
use crate::{
    payloads::{EditEphemeralMessageMedia, SendPoll},
    types::{
        ChatId, InputFile, InputMediaAnimation, InputMediaLivePhoto, InputMediaPhoto,
        InputMediaSticker, InputMediaVideo, InputPollMedia, InputPollOption, InputPollOptionMedia,
        UserId,
    },
};

fn file(contents: &'static [u8], name: &'static str) -> InputFile {
    InputFile::memory(contents).file_name(name)
}

fn populated_poll() -> SendPoll {
    let options = vec![
        InputPollOption::new("sticker").media(InputPollOptionMedia::Sticker(InputMediaSticker {
            media: file(b"sticker", "sticker.bin"),
            emoji: None,
        })),
        InputPollOption::new("live").media(InputPollOptionMedia::LivePhoto(
            InputMediaLivePhoto::new(
                file(b"option-live-video", "option-live-video.bin"),
                file(b"option-live-photo", "option-live-photo.bin"),
            ),
        )),
    ];

    let mut payload = SendPoll::new(ChatId(1), "question", options);
    payload.media = Some(InputPollMedia::Video(
        InputMediaVideo::new(file(b"video", "video.bin"))
            .thumbnail(file(b"thumbnail", "thumbnail.bin"))
            .cover(file(b"cover", "cover.bin")),
    ));
    payload.explanation_media = Some(InputPollMedia::Animation(
        InputMediaAnimation::new(file(b"animation", "animation.bin"))
            .thumbnail(file(b"animation-thumbnail", "animation-thumbnail.bin")),
    ));
    payload
}

fn multipart_parts(body: &[u8], boundary: &str) -> BTreeMap<String, Vec<u8>> {
    let delimiter = format!("--{boundary}");
    let body = String::from_utf8_lossy(body);
    let mut parts = BTreeMap::new();

    for raw_part in body.split(&delimiter).skip(1) {
        let raw_part = raw_part.trim_start_matches("\r\n").trim_end_matches("\r\n");
        if raw_part.is_empty() || raw_part == "--" {
            continue;
        }

        let raw_part = raw_part.strip_suffix("--").unwrap_or(raw_part);
        let (headers, contents) = raw_part.split_once("\r\n\r\n").expect("multipart part");
        let name = headers
            .split("; ")
            .find_map(|item| item.strip_prefix("name=\"").and_then(|v| v.strip_suffix('"')))
            .expect("multipart part name")
            .to_owned();

        assert!(parts.insert(name, contents.as_bytes().to_vec()).is_none(), "duplicate part");
    }

    parts
}

fn collect_attach_ids(value: &Value, into: &mut Vec<String>) {
    match value {
        Value::String(value) => {
            if let Some(id) = value.strip_prefix("attach://") {
                into.push(id.to_owned());
            }
        }
        Value::Array(values) => values.iter().for_each(|value| collect_attach_ids(value, into)),
        Value::Object(values) => values.values().for_each(|value| collect_attach_ids(value, into)),
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

// This crosses the serializer/reqwest boundary and validates the final wire
// representation.
#[tokio::test]
async fn send_poll_attach_ids_match_multipart_file_parts() {
    let form = to_form_ref(&populated_poll()).unwrap().await;
    let mut request =
        Client::new().post("http://localhost.invalid").multipart(form).build().unwrap();

    let boundary = request.headers()[CONTENT_TYPE]
        .to_str()
        .unwrap()
        .split("boundary=")
        .nth(1)
        .expect("multipart boundary")
        .to_owned();
    let body = request.body_mut().take().unwrap().collect().await.unwrap().to_bytes();
    let parts = multipart_parts(&body, &boundary);

    let mut attach_ids = Vec::new();
    for field in ["media", "explanation_media", "options"] {
        let json = parts.get(field).unwrap_or_else(|| panic!("missing {field} part"));
        collect_attach_ids(&serde_json::from_slice(json).unwrap(), &mut attach_ids);
    }

    let file_part_ids: BTreeSet<_> = parts
        .keys()
        .filter(|name| {
            !matches!(
                name.as_str(),
                "chat_id" | "question" | "options" | "media" | "explanation_media"
            )
        })
        .cloned()
        .collect();
    let attach_id_set: BTreeSet<_> = attach_ids.iter().cloned().collect();

    assert_eq!(attach_ids.len(), attach_id_set.len(), "duplicate attach:// id");
    assert_eq!(attach_ids.len(), 8);
    assert_eq!(attach_id_set, file_part_ids);
}

#[tokio::test]
async fn edit_ephemeral_message_media_attach_ids_match_multipart_file_parts() {
    let payload = EditEphemeralMessageMedia::new(
        ChatId(1),
        UserId(2),
        3,
        crate::types::InputMedia::Photo(InputMediaPhoto::new(file(b"photo", "photo.bin"))),
    );
    let form = to_form_ref(&payload).unwrap().await;
    let mut request =
        Client::new().post("http://localhost.invalid").multipart(form).build().unwrap();

    let boundary = request.headers()[CONTENT_TYPE]
        .to_str()
        .unwrap()
        .split("boundary=")
        .nth(1)
        .expect("multipart boundary")
        .to_owned();
    let body = request.body_mut().take().unwrap().collect().await.unwrap().to_bytes();
    let parts = multipart_parts(&body, &boundary);

    let media = parts.get("media").expect("missing media part");
    let mut attach_ids = Vec::new();
    collect_attach_ids(&serde_json::from_slice(media).unwrap(), &mut attach_ids);

    let file_part_ids: BTreeSet<_> = parts
        .keys()
        .filter(|name| {
            !matches!(
                name.as_str(),
                "chat_id" | "receiver_user_id" | "ephemeral_message_id" | "media"
            )
        })
        .cloned()
        .collect();
    let attach_id_set: BTreeSet<_> = attach_ids.iter().cloned().collect();

    assert_eq!(attach_ids, attach_id_set.iter().cloned().collect::<Vec<_>>());
    assert_eq!(attach_id_set, file_part_ids);
    assert_eq!(parts.get(&attach_ids[0]).map(Vec::as_slice), Some(b"photo".as_slice()));
}
