#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Clone)]
/// This object represents one size of a photo or a [`Document`] /
/// [`Sticker`] thumbnail.
///
/// [`Document`]: crate::types::Document
/// [`Sticker`]: crate::types::Sticker
pub struct PhotoSize {
    /// Identifier for this file
    pub file_id: String,
    /// Photo width
    pub width: i32,
    /// Photo height
    pub height: i32,
    /// Optional. File size
    pub file_size: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{"file_id":"id","width":320,"height":320,
                             "file_size":3452}"#;
        let expected = PhotoSize {
            file_id: "id".to_string(),
            width: 320,
            height: 320,
            file_size: Some(3452),
        };
        let actual = serde_json::from_str::<PhotoSize>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
