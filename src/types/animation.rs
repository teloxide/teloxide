use serde::{Deserialize, Serialize};

use crate::types::{MimeWrapper, PhotoSize};

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video
/// without sound).
///
/// [The official docs](https://core.telegram.org/bots/api#animation).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
    pub mime_type: Option<MimeWrapper>,

    /// A size of a file.
    pub file_size: Option<u32>,
}

impl Animation {
    pub fn new<S1, S2>(
        file_id: S1,
        file_unique_id: S2,
        width: u32,
        height: u32,
        duration: u32,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            width,
            height,
            duration,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
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

    pub fn width(mut self, val: u32) -> Self {
        self.width = val;
        self
    }

    pub fn height(mut self, val: u32) -> Self {
        self.height = val;
        self
    }

    pub fn duration(mut self, val: u32) -> Self {
        self.duration = val;
        self
    }

    pub fn thumb(mut self, val: PhotoSize) -> Self {
        self.thumb = Some(val);
        self
    }

    pub fn file_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.file_name = Some(val.into());
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
            mime_type: Some(MimeWrapper("video/gif".parse().unwrap())),
            file_size: Some(6500),
        };
        let actual = serde_json::from_str::<Animation>(json).unwrap();
        assert_eq!(actual, expected)
    }
}
