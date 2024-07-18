use serde::{Deserialize, Serialize};

use crate::types::FileMeta;

/// This object represents one size of a photo or a [file]/[sticker] thumbnail.
///
/// [file]: crate::types::Document
/// [sticker]: crate::types::Sticker
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PhotoSize {
    /// Metadata of the photo file.
    #[serde(flatten)]
    pub file: FileMeta,

    /// Photo width.
    pub width: u32,

    /// Photo height.
    pub height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{"file_id":"id","file_unique_id":"","width":320,"height":320,
                             "file_size":3452}"#;
        let expected = PhotoSize {
            file: FileMeta { id: "id".to_owned(), unique_id: "".to_owned(), size: 3452 },
            width: 320,
            height: 320,
        };
        let actual = serde_json::from_str::<PhotoSize>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
