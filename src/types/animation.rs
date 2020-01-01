use serde::{Deserialize, Serialize};

use crate::types::PhotoSize;

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video
/// without sound).
///
/// [The official docs](https://core.telegram.org/bots/api#animation).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Animation {
    /// An identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// A video width as defined by a sender.
    pub width: u32,

    /// A video height as defined by a sender.
    pub height: u32,

    /// A duration of the video in seconds as defined by a sender.
    pub duration: u32,

    /// An animation thumbnail as defined by a sender.
    pub thumb: Option<PhotoSize>,

    /// An original animation filename as defined by a sender.
    pub file_name: Option<String>,

    /// A MIME type of the file as defined by a sender.
    pub mime_type: Option<String>,

    /// A size of a file.
    pub file_size: Option<u32>,
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
        "mime_type":"gif",
        "file_size":6500}"#;
        let expected = Animation {
            file_id: "id".to_string(),
            file_unique_id: "".to_string(),
            width: 320,
            height: 320,
            duration: 59,
            thumb: Some(PhotoSize {
                file_id: "id".to_string(),
                file_unique_id: "".to_string(),
                width: 320,
                height: 320,
                file_size: Some(3452),
            }),
            file_name: Some("some".to_string()),
            mime_type: Some("gif".to_string()),
            file_size: Some(6500),
        };
        let actual = serde_json::from_str::<Animation>(json).unwrap();
        assert_eq!(actual, expected)
    }
}
