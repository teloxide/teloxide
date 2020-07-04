use crate::{
    net,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup},
    Bot,
};
use std::sync::Arc;

/// Use this method to send photos.
///
/// [The official docs](https://core.telegram.org/bots/api#sendphoto).
#[derive(Debug, Clone)]
pub struct SendPhoto {
    bot: Arc<Bot>,
    chat_id: ChatId,
    photo: InputFile,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i32>,
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request for SendPhoto {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendPhoto",
            FormBuilder::new()
                .add_text("chat_id", &self.chat_id)
                .add_input_file("photo", &self.photo)
                .await
                .add_text("caption", &self.caption)
                .add_text("parse_mode", &self.parse_mode)
                .add_text("disable_notification", &self.disable_notification)
                .add_text("reply_to_message_id", &self.reply_to_message_id)
                .add_text("reply_markup", &self.reply_markup)
                .build(),
        )
        .await
    }
}

impl SendPhoto {
    pub(crate) fn new<C>(bot: Arc<Bot>, chat_id: C, photo: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            photo,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
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

    /// Photo to send.
    ///
    /// Pass [`InputFile::File`] to send a photo that exists on
    /// the Telegram servers (recommended), pass an [`InputFile::Url`] for
    /// Telegram to get a .webp file from the Internet, or upload a new one
    /// using [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    ///
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub fn photo(mut self, val: InputFile) -> Self {
        self.photo = val;
        self
    }

    ///Photo caption (may also be used when resending photos by file_id),
    /// 0-1024 characters.
    pub fn caption<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    /// Send [Markdown] or [HTML], if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: crate::types::ParseMode::Markdown
    /// [HTML]: crate::types::ParseMode::HTML
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::types::ParseMode
    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    /// Sends the message [silently]. Users will receive a notification with no
    /// sound.
    ///
    /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    /// If the message is a reply, ID of the original message.
    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    /// Additional interface options. A JSON-serialized object for an [inline
    /// keyboard], [custom reply keyboard], instructions to remove reply
    /// keyboard or to force a reply from the user.
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
