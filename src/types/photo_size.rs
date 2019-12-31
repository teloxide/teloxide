use serde::{Deserialize, Serialize};

/// This object represents one size of a photo or a [file]/[sticker] thumbnail.
///
/// [file]: crate::types::Document
/// [sticker]: crate::types::Sticker
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
