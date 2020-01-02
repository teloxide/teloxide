use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method when you need to tell the user that something is happening
/// on the bot's side. The status is set for 5 seconds or less (when a message
/// arrives from your bot, Telegram clients clear its typing status). Returns
/// True on success.Example: The ImageBot needs some time to process a request
/// and upload the image. Instead of sending a text message along the lines of
/// “Retrieving image, please wait…”, the bot may use sendChatAction with action
/// = upload_photo. The user will see a “sending photo” status for the bot.We
/// only recommend using this method when a response from the bot will take a
/// noticeable amount of time to arrive.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendChatAction<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Type of action to broadcast. Choose one, depending on what the user is
    /// about to receive: typing for text messages, upload_photo for photos,
    /// record_video or upload_video for videos, record_audio or upload_audio
    /// for audio files, upload_document for general files, find_location for
    /// location data, record_video_note or upload_video_note for video notes.
    action: String,
}

#[async_trait::async_trait]
impl Request<True> for SendChatAction<'_> {
    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendChatAction",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> SendChatAction<'a> {
    pub(crate) fn new<C, A>(bot: &'a Bot, chat_id: C, action: A) -> Self
    where
        C: Into<ChatId>,
        A: Into<String>,
    {
        let chat_id = chat_id.into();
        let action = action.into();
        Self {
            bot,
            chat_id,
            action,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn action<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.action = val.into();
        self
    }
}
