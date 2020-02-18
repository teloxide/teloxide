use crate::{
    net,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatOrInlineMessage, InlineKeyboardMarkup, InputMedia, Message},
    Bot,
};
use std::sync::Arc;

/// Use this method to edit animation, audio, document, photo, or video
/// messages.
///
/// If a message is a part of a message album, then it can be edited only to a
/// photo or a video. Otherwise, message type can be changed arbitrarily. When
/// inline message is edited, new file can't be uploaded. Use previously
/// uploaded file via its `file_id` or specify a URL. On success, if the edited
/// message was sent by the bot, the edited [`Message`] is returned,
/// otherwise [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagemedia).
///
/// [`Message`]: crate::types::Message
/// [`True`]: crate::types::True
#[derive(Debug, Clone)]
pub struct EditMessageMedia {
    bot: Arc<Bot>,
    chat_or_inline_message: ChatOrInlineMessage,
    media: InputMedia,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditMessageMedia {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        let mut params = FormBuilder::new();

        match &self.chat_or_inline_message {
            ChatOrInlineMessage::Chat { chat_id, message_id } => {
                params = params
                    .add("chat_id", chat_id)
                    .await
                    .add("message_id", message_id)
                    .await;
            }
            ChatOrInlineMessage::Inline { inline_message_id } => {
                params =
                    params.add("inline_message_id", inline_message_id).await;
            }
        }

        net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "editMessageMedia",
            params
                .add("media", &self.media)
                .await
                .add("reply_markup", &self.reply_markup)
                .await
                .build(),
        )
        .await
    }
}

impl EditMessageMedia {
    pub(crate) fn new(
        bot: Arc<Bot>,
        chat_or_inline_message: ChatOrInlineMessage,
        media: InputMedia,
    ) -> Self {
        Self { bot, chat_or_inline_message, media, reply_markup: None }
    }

    pub fn chat_or_inline_message(mut self, val: ChatOrInlineMessage) -> Self {
        self.chat_or_inline_message = val;
        self
    }

    /// A JSON-serialized object for a new media content of the message.
    pub fn media(mut self, val: InputMedia) -> Self {
        self.media = val;
        self
    }

    /// A JSON-serialized object for a new [inline keyboard].
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
