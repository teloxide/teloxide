use serde::Serialize;

use crate::{
    network,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, InlineKeyboardMarkup, InputMedia, Message},
    Bot,
};

/// Use this method to edit animation, audio, document, photo, or video
/// messages. If a message is a part of a message album, then it can be edited
/// only to a photo or a video. Otherwise, message type can be changed
/// arbitrarily. When inline message is edited, new file can't be uploaded. Use
/// previously uploaded file via its file_id or specify a URL. On success, if
/// the edited message was sent by the bot, the edited Message is returned,
/// otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageMedia<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Identifier of the message to edit
    message_id: i32,
    /// A JSON-serialized object for a new media content of the message
    media: InputMedia,
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for EditMessageMedia<'_> {
    async fn send(&self) -> ResponseResult<Message> {
        network::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "editMessageMedia",
            FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .add("message_id", &self.message_id)
                .add("media", &self.media)
                .add("reply_markup", &self.reply_markup)
                .build(),
        )
        .await
    }
}

impl<'a> EditMessageMedia<'a> {
    pub(crate) fn new<C>(
        bot: &'a Bot,
        chat_id: C,
        message_id: i32,
        media: InputMedia,
    ) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot,
            chat_id,
            message_id,
            media,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn message_id(mut self, val: i32) -> Self {
        self.message_id = val;
        self
    }

    pub fn media(mut self, val: InputMedia) -> Self {
        self.media = val;
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
