use serde::{Deserialize, Serialize};

use crate::types::{MimeWrapper, PhotoSize};

/// This object represents an audio file to be treated as music by the Telegram
/// clients.
///
/// [The official docs](https://core.telegram.org/bots/api#audio).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Audio {
    /// An identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// A duration of the audio in seconds as defined by a sender.
    pub duration: u32,

    /// A performer of the audio as defined by a sender or by audio tags.
    pub performer: Option<String>,

    /// A title of the audio as defined by sender or by audio tags.
    pub title: Option<String>,

    /// A MIME type of the file as defined by a sender.
    pub mime_type: Option<MimeWrapper>,

    /// A size of a file.
    pub file_size: Option<u32>,

    /// A thumbnail of the album cover to which the music file belongs.
    pub thumb: Option<PhotoSize>,
}

impl Audio {
    pub fn new<S1, S2>(file_id: S1, file_unique_id: S2, duration: u32) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            duration,
            performer: None,
            title: None,
            mime_type: None,
            file_size: None,
            thumb: None,
        }
    }

    pub fn file_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_id = val.into();
        self
    }

    pub fn file_unique_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_unique_id = val.into();
        self
    }

    pub fn duration(mut self, val: u32) -> Self {
        self.duration = val;
        self
    }

    pub fn performer<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.performer = Some(val.into());
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(val.into());
        self
    }

    pub fn mime_type(mut self, val: MimeWrapper) -> Self {
        self.mime_type = Some(val);
        self
    }

    pub fn file_size(mut self, val: u32) -> Self {
        self.file_size = Some(val);
        self
    }

    pub fn thumb(mut self, val: PhotoSize) -> Self {
        self.thumb = Some(val);
        self
    }
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
            file_id: "id".to_string(),
            file_unique_id: "".to_string(),
            duration: 60,
            performer: Some("Performer".to_string()),
            title: Some("Title".to_string()),
            mime_type: Some(serde_json::from_str("\"application/zip\"").unwrap()),
            file_size: Some(123_456),
            thumb: Some(PhotoSize {
                file_id: "id".to_string(),
                file_unique_id: "".to_string(),
                width: 320,
                height: 320,
                file_size: Some(3452),
            }),
        };
        let actual = serde_json::from_str::<Audio>(&json).unwrap();
        assert_eq!(actual, expected)
    }
}
