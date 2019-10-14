#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordAudio,
    UploadAudio,
    UploadDocument,
    FindLocation,
    RecordVideoNote,
    UploadVideoNote,
}
