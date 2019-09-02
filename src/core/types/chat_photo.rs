use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub big_file_id: String,
}
