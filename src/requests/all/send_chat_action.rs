use serde::{Deserialize, Serialize};

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};
use std::sync::Arc;

/// Use this method when you need to tell the user that something is happening
/// on the bot's side.
///
/// The status is set for 5 seconds or less (when a message arrives from your
/// bot, Telegram clients clear its typing status).
///
/// ## Note
/// Example: The [ImageBot] needs some time to process a request and upload the
/// image. Instead of sending a text message along the lines of “Retrieving
/// image, please wait…”, the bot may use [`Bot::send_chat_action`] with `action
/// = upload_photo`. The user will see a `sending photo` status for the bot.
///
/// We only recommend using this method when a response from the bot will take a
/// **noticeable** amount of time to arrive.
///
/// [ImageBot]: https://t.me/imagebot
/// [`Bot::send_chat_action`]: crate::Bot::send_chat_action
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendChatAction {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
    action: SendChatActionKind,
}

/// A type of action used in [`SendChatAction`].
///
/// [`SendChatAction`]: crate::requests::SendChatAction
#[derive(Copy, Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SendChatActionKind {
    /// For [text messages](crate::Bot::send_message).
    Typing,

    /// For [photos](crate::Bot::send_photo).
    UploadPhoto,

    /// For [videos](crate::Bot::send_video).
    RecordVideo,

    /// For [videos](crate::Bot::send_video).
    UploadVideo,

    /// For [audio files](crate::Bot::send_audio).
    RecordAudio,

    /// For [audio files](crate::Bot::send_audio).
    UploadAudio,

    /// For [general files](crate::Bot::send_document).
    UploadDocument,

    /// For [location data](crate::Bot::send_location).
    FindLocation,

    /// For [video notes](crate::Bot::send_video_note).
    RecordVideoNote,

    /// For [video notes](crate::Bot::send_video_note).
    UploadVideoNote,
}

#[async_trait::async_trait]
impl Request for SendChatAction {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendChatAction",
            &self,
        )
        .await
    }
}

impl SendChatAction {
    pub(crate) fn new<C>(
        bot: Arc<Bot>,
        chat_id: C,
        action: SendChatActionKind,
    ) -> Self
    where
        C: Into<ChatId>,
    {
        Self { bot, chat_id: chat_id.into(), action }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Type of action to broadcast.
    pub fn action(mut self, val: SendChatActionKind) -> Self {
        self.action = val;
        self
    }
}
