#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: i32,
    pub heigth: i32,
    pub file_size: Option<u32>
}
