use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
/// This object represents a voice note.
pub struct Voice {
    /// Identifier for this file
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: u32,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<u64>,
}
