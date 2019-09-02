use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Document {
    file_id: String,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}
