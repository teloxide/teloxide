use serde::{Deserialize, Serialize};

/// This object represents one size of a photo or a [file]/[sticker] thumbnail.
///
/// [file]: crate::types::Document
/// [sticker]: crate::types::Sticker
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PhotoSize {
    /// Identifier for this file.
    pub file_id: String,

    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,

    /// Photo width.
    pub width: i32,

    /// Photo height.
    pub height: i32,

    /// File size.
    pub file_size: Option<u32>,
}

impl PhotoSize {
    pub fn new<S1, S2>(file_id: S1, file_unique_id: S2, width: i32, height: i32) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            width,
            height,
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

    pub fn width(mut self, val: i32) -> Self {
        self.width = val;
        self
    }

    pub fn height(mut self, val: i32) -> Self {
        self.height = val;
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
        let json = r#"{"file_id":"id","file_unique_id":"","width":320,"height":320,
                             "file_size":3452}"#;
        let expected = PhotoSize {
            file_id: "id".to_string(),
            file_unique_id: "".to_string(),
            width: 320,
            height: 320,
            file_size: Some(3452),
        };
        let actual = serde_json::from_str::<PhotoSize>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
