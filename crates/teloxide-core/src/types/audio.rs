use mime::Mime;
use serde::{Deserialize, Serialize};

use crate::types::{FileMeta, PhotoSize, Seconds};

/// This object represents an audio file to be treated as music by the Telegram
/// clients.
///
/// [The official docs](https://core.telegram.org/bots/api#audio).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    /// Metadata of the audio file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// A duration of the audio in seconds as defined by a sender.
    pub duration: Seconds,

    /// A performer of the audio as defined by a sender or by audio tags.
    pub performer: Option<String>,

    /// A title of the audio as defined by sender or by audio tags.
    pub title: Option<String>,

    /// Original filename as defined by sender
    pub file_name: Option<String>,

    /// A MIME type of the file as defined by a sender.
    #[serde(with = "crate::types::non_telegram_types::mime::opt_deser")]
    pub mime_type: Option<Mime>,

    /// A thumbnail of the album cover to which the music file belongs.
    pub thumb: Option<PhotoSize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "file_id":"id",
            "file_unique_id":"",
            "duration":60,
            "performer":"Performer",
            "title":"Title",
            "mime_type":"application/zip",
            "file_size":123456,
            "thumb":{
                "file_id":"id",
                "file_unique_id":"",
                "width":320,
                "height":320,
                "file_size":3452
            }
        }"#;
        let expected = Audio {
            file: FileMeta { id: "id".to_string(), unique_id: "".to_string(), size: 123_456 },
            duration: Seconds::from_seconds(60),
            performer: Some("Performer".to_string()),
            title: Some("Title".to_string()),
            mime_type: Some("application/zip".parse().unwrap()),
            thumb: Some(PhotoSize {
                file: FileMeta { id: "id".to_owned(), unique_id: "".to_owned(), size: 3452 },
                width: 320,
                height: 320,
            }),
            file_name: None,
        };
        let actual = serde_json::from_str::<Audio>(json).unwrap();
        assert_eq!(actual, expected)
    }
}
