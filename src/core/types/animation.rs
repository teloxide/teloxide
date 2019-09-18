use crate::core::types::PhotoSize;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video
/// without sound).
pub struct Animation {
    /// Identifier for this file
    pub file_id: String,
    /// Video width as defined by sender
    pub width: u32,
    /// Video height as defined by sender
    pub height: u32,
    /// Duration of the video in seconds as defined by sender
    pub duration: u32,
    /// Optional. Animation thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Optional. Original animation filename as defined by sender
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
        "file_id":"id",
        "width":320,
        "height":320,
        "duration":59,
        "thumb":{
            "file_id":"id",
            "width":320,
            "height":320,
            "file_size":3452
        },
        "file_name":"some",
        "mime_type":"gif",
        "file_size":6500}"#;
        let expected = Animation {
            file_id: "id".to_string(),
            width: 320,
            height: 320,
            duration: 59,
            thumb: Some(PhotoSize {
                file_id: "id".to_string(),
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
