#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct File {
    pub file_id: String,
    pub file_size: u32,
    pub file_path: String
}
