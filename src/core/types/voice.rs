use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// This object represents a voice note.
struct Voice {
    /// Identifier for this file
    file_id: String,
    /// Duration of the audio in seconds as defined by sender
    duration: u32,
    /// Optional. MIME type of the file as defined by sender
    mime_type: Option<String>,
    /// Optional. File size
    file_size: Option<u64>
}