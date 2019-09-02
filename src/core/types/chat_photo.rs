use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}
