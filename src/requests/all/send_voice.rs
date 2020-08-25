use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup},
    Bot,
};

/// Use this method to send audio files, if you want Telegram clients to display
/// the file as a playable voice message.
///
/// For this to work, your audio must be in an .ogg file encoded with OPUS
/// (other formats may be sent as [`Audio`] or [`Document`]). Bots can currently
/// send voice messages of up to 50 MB in size, this limit may be changed in the
/// future.
///
/// [The official docs](https://core.telegram.org/bots/api#sendvoice).
///
/// [`Audio`]: crate::types::Audio
/// [`Document`]: crate::types::Document
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendVoice {
    #[serde(skip_serializing)]
    bot: Bot,
    pub chat_id: ChatId,
    pub voice: InputFile,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub duration: Option<i32>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl RequestOld for SendVoice {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_multipart(self.bot.client(), self.bot.token(), "sendVoice", self).await
    }
}

impl SendVoice {
    pub(crate) fn new<C>(bot: Bot, chat_id: C, voice: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            voice,
            caption: None,
            parse_mode: None,
            duration: None,
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

    /// Audio file to send.
    ///
    /// Pass [`InputFile::File`] to send a file that exists on
    /// the Telegram servers (recommended), pass an [`InputFile::Url`] for
    /// Telegram to get a .webp file from the Internet, or upload a new one
    /// using [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub fn voice(mut self, val: InputFile) -> Self {
        self.voice = val;
        self
    }

    /// Voice message caption, 0-1024 characters.
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

    /// Duration of the voice message in seconds.
    pub fn duration(mut self, val: i32) -> Self {
        self.duration = Some(val);
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

    /// Additional interface options.
    ///
    /// A JSON-serialized object for an [inline keyboard], [custom reply
    /// keyboard], instructions to remove reply keyboard or to force a reply
    /// from the user.
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
