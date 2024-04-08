use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, PhotoSize, Seconds};

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video
/// without sound).
///
/// [The official docs](https://core.telegram.org/bots/api#animation).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    /// Metadata of the animation file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// A video width as defined by a sender.
    pub width: u32,

    /// A video height as defined by a sender.
    pub height: u32,

    /// A duration of the video in seconds as defined by a sender.
    pub duration: Seconds,

    /// An animation thumbnail as defined by a sender.
    pub thumb: Option<PhotoSize>,

    /// An original animation filename as defined by a sender.
    pub file_name: Option<String>,

    /// A MIME type of the file as defined by a sender.
    #[serde(with = "crate::types::non_telegram_types::mime::opt_deser")]
    pub mime_type: Option<Mime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
        "file_id":"id",
        "file_unique_id":"",
        "width":320,
        "height":320,
        "duration":59,
        "thumb":{
            "file_id":"id",
            "file_unique_id":"",
            "width":320,
            "height":320,
            "file_size":3452
        },
        "file_name":"some",
        "mime_type":"video/gif",
        "file_size":6500}"#;
        let expected = Animation {
            file: FileMeta { id: "id".to_string(), unique_id: "".to_string(), size: 6500 },
            width: 320,
            height: 320,
            duration: Seconds::from_seconds(59),
            thumb: Some(PhotoSize {
                file: FileMeta { id: "id".to_owned(), unique_id: "".to_owned(), size: 3452 },
                width: 320,
                height: 320,
            }),
            file_name: Some("some".to_string()),
            mime_type: Some("video/gif".parse().unwrap()),
        };
        let actual = serde_json::from_str::<Animation>(json).unwrap();
        assert_eq!(actual, expected)
    }
}
