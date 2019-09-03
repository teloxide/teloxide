use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Voice {
    file_id: String,
    duration: u32,
    mime_type: Option<String>,
    file_size: Option<u64>
}