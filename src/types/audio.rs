use crate::types::PhotoSize;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct Audio {
    pub file_id: String,
    pub duration: u32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u32>,
    pub thumb: Option<PhotoSize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "file_id":"id",
            "duration":60,
            "performer":"Performer",
            "title":"Title",
            "mime_type":"MimeType",
            "file_size":123456,
            "thumb":{
                "file_id":"id",
                "width":320,
                "height":320,
                "file_size":3452
            }
        }"#;
        let expected = Audio {
            file_id: "id".to_string(),
            duration: 60,
            performer: Some("Performer".to_string()),
            title: Some("Title".to_string()),
            mime_type: Some("MimeType".to_string()),
            file_size: Some(123456),
            thumb: Some(PhotoSize {
                file_id: "id".to_string(),
                width: 320,
                height: 320,
                file_size: Some(3452),
            }),
        };
        let actual = serde_json::from_str::<Audio>(&json).unwrap();
        assert_eq!(actual, expected)
    }
}
