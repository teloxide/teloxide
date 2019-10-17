use async_trait::async_trait;

use crate::{
    network,
    requests::{Request,  ResponseResult},
    types::{ChatAction, ChatId, True},
};
use crate::bot::Bot;

///Use this method when you need to tell the user that something is happening
/// on the bot's side. The status is set for 5 seconds or less (when a message
/// arrives from your bot, Telegram clients clear its typing status).
/// Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct SendChatAction<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
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
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SendChatAction<'_> {
    pub async fn send(self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendChatAction",
            &self,
        )
        .await
    }
}

impl<'a> SendChatAction<'a> {
    pub(crate) fn new<Cid, Ca>(bot: &'a Bot, chat_id: Cid, action: Ca,) -> Self
    where
        Cid: Into<ChatId>,
        Ca: Into<ChatAction>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            action: action.into(),
        }
    }

    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    pub fn action<T>(mut self, value: T) -> Self
    where
        T: Into<ChatAction>,
    {
        self.action = value.into();
        self
    }
}
