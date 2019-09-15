use crate::core::network;
use crate::core::requests::{ChatId, Request, RequestContext, RequestFuture, ResponseResult};
use crate::core::types::Message;

#[derive(Debug, Clone, Serialize)]
struct SendChatAction<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// Type of action to broadcast. Choose one, depending on what the user is
    /// about to receive: typing for text messages, upload_photo for photos,
    /// record_video or upload_video for videos, record_audio or upload_audio
    /// for audio files, upload_document for general files, find_location for
    /// location data, record_video_note or upload_video_note for video notes.
    pub action: ChatAction,
}

#[derive(Debug, Serialize, From, Clone)]
enum ChatAction {
    #[serde(rename = "typing")]
    Typing,
    #[serde(rename = "upload_photo")]
    UploadPhoto,
    #[serde(rename = "record_video")]
    RecordVideo,
    #[serde(rename = "upload_video")]
    UploadVideo,
    #[serde(rename = "record_audio")]
    RecordAudio,
    #[serde(rename = "upload_audio")]
    UploadAudio,
    #[serde(rename = "upload_document")]
    UploadDocument,
    #[serde(rename = "find_location")]
    FindLocation,
    #[serde(rename = "record_video_note")]
    RecordVideoNote,
    #[serde(rename = "upload_video_note")]
    UploadVideoNote,
}

impl<'a> Request<'a> for SendChatAction<'a> {
    type ReturnValue = Message;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request_json(
                &self.ctx.client,
                &self.ctx.token,
                "sendChatAction",
                &self,
            )
                .await
        })
    }
}

impl<'a> SendChatAction<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        action: ChatAction,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            action,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
        where
            T: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }


    pub fn action<T>(mut self, action: T) -> Self
        where
            T: Into<ChatAction>,
    {
        self.action = action.into();
        self
    }
}