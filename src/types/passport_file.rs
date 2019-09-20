#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
pub struct PassportFile {
    pub file_id: String,
    pub file_size: u64,
    pub file_date: u64,
}