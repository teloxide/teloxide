/// This object represents a file uploaded to Telegram Passport. Currently all
/// Telegram Passport files are in JPEG format when decrypted and don't exceed
/// 10MB.
///
/// [The official docs](https://core.telegram.org/bots/api#passportfile).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PassportFile {
    /// Identifier for this file.
    pub file_id: String,

    /// File size.
    pub file_size: u64,

    /// Unix time when the file was uploaded.
    pub file_date: u64,
}
