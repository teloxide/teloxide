use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct VideoNote {
    file_id: String,
    length: u32,
    duration: u32,
    thumb: Option<PhotoSize>,
    file_size: Option<u32>,
}