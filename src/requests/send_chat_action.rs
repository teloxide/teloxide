use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::{ChatAction, ChatId, True},
};

///Use this method when you need to tell the user that something is happening
/// on the bot's side. The status is set for 5 seconds or less (when a message
/// arrives from your bot, Telegram clients clear its typing status).
/// Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct SendChatAction<'a> {
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

#[async_trait]
impl Request for SendChatAction<'_> {
    type ReturnValue = True;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl SendChatAction<'_> {
    pub async fn send(self) -> ResponseResult<True> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "sendChatAction",
            &self,
        )
        .await
    }
}

impl<'a> SendChatAction<'a> {
    pub(crate) fn new<Cid, Ca>(
        ctx: RequestContext<'a>,
        chat_id: Cid,
        action: Ca,
    ) -> Self
    where
        Cid: Into<ChatId>,
        Ca: Into<ChatAction>,
    {
        Self {
            ctx,
            chat_id: chat_id.into(),
            action: action.into(),
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
