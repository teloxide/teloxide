use crate::{
    net,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{InlineKeyboardMarkup, InputMedia, True},
    Bot,
};

/// Use this method to edit animation, audio, document, photo, or video
/// messages sent via the bot.
///
/// If a message is a part of a message album, then it can be edited only to a
/// photo or a video. Otherwise, message type can be changed arbitrarily. When
/// this method is used, new file can't be uploaded. Use previously
/// uploaded file via its `file_id` or specify a URL. On success, [`True`] is
/// returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagemedia).
///
/// [`True`]: crate::types::True
#[derive(Debug, Clone)]
pub struct EditInlineMessageMedia {
    bot: Bot,
    inline_message_id: String,
    media: InputMedia,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditInlineMessageMedia {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "editMessageMedia",
            FormBuilder::new()
                .add_text("media", &self.media)
                .add_text("reply_markup", &self.reply_markup)
                .add_text("inline_message_id", &self.inline_message_id)
                .build(),
        )
        .await
    }
}

impl EditInlineMessageMedia {
    pub(crate) fn new<I>(bot: Bot, inline_message_id: I, media: InputMedia) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self { bot, inline_message_id, media, reply_markup: None }
    }

    /// Identifier of the inline message.
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = val.into();
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
