//! Additions to [`JsonRequest`] and [`MultipartRequest`].
//!
//! [`JsonRequest`]: teloxide_core::requests::JsonRequest
//! [`MultipartRequest`]: teloxide_core::requests::MultipartRequest

use teloxide_core::{
    payloads::*,
    requests::{JsonRequest, MultipartRequest},
    types::*,
};

macro_rules! impl_request_reply_ext {
    ($($t:ty),*) => {
        $(
            impl RequestReplyExt for $t {
                fn reply_to<M>(self, message_id: M) -> Self
                where
                    M: Into<MessageId>,
                    Self: Sized,
                {
                    self.reply_parameters(ReplyParameters::new(message_id.into()))
                }
            }
        )*
    };
}

macro_rules! impl_request_link_preview_ext {
    ($($t:ty),*) => {
        $(
            impl RequestLinkPreviewExt for $t {
                fn disable_link_preview(self, is_disabled: bool) -> Self
                where
                    Self: Sized
                {
                    let link_preview_options = LinkPreviewOptions {
                        is_disabled,
                        url: None,
                        prefer_small_media: false,
                        prefer_large_media: false,
                        show_above_text: false,
                    };
                    self.link_preview_options(link_preview_options)
                }
            }
        )*
    };
}

/// `.reply_to(msg)` syntax sugar for requests.
pub trait RequestReplyExt {
    /// Replaces `.reply_parameters(ReplyParameters::new(msg.id))`
    /// with `.reply_to(msg.id)` or `.reply_to(msg)`
    fn reply_to<M>(self, message_id: M) -> Self
    where
        M: Into<MessageId>,
        Self: Sized;
}

/// `.disable_link_preview(is_disabled)` syntax sugar for requests.
pub trait RequestLinkPreviewExt {
    /// Replaces
    /// `.link_preview_options(LinkPreviewOptions {
    ///     is_disabled: true,
    ///     url: None,
    ///     prefer_small_media: false,
    ///     prefer_large_media: false,
    ///     show_above_text: false
    /// };)`
    ///
    /// with `.disable_link_preview(true)`.
    fn disable_link_preview(self, is_disabled: bool) -> Self
    where
        Self: Sized;
}

impl_request_reply_ext! {
    JsonRequest<SendDice>,
    JsonRequest<SendInvoice>,
    JsonRequest<SendPoll>,
    JsonRequest<SendContact>,
    JsonRequest<SendGame>,
    JsonRequest<SendVenue>,
    JsonRequest<SendLocation>,
    JsonRequest<CopyMessage>,
    JsonRequest<SendMessage>,
    MultipartRequest<SendSticker>,
    MultipartRequest<SendMediaGroup>,
    MultipartRequest<SendAnimation>,
    MultipartRequest<SendVideoNote>,
    MultipartRequest<SendVideo>,
    MultipartRequest<SendDocument>,
    MultipartRequest<SendAudio>,
    MultipartRequest<SendVoice>,
    MultipartRequest<SendPhoto>
}

impl_request_link_preview_ext! {
    JsonRequest<SendMessage>,
    JsonRequest<EditMessageText>
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    use teloxide_core::{prelude::Requester, Bot};

    #[test]
    fn test_reply_to() {
        let bot = Bot::new("TOKEN");

        let real_reply_req = bot
            .send_message(ChatId(1234), "test")
            .reply_parameters(ReplyParameters::new(MessageId(1)));
        let sugar_reply_req = bot.send_message(ChatId(1234), "test").reply_to(MessageId(1));

        assert_eq!(real_reply_req.deref(), sugar_reply_req.deref())
    }

    #[test]
    fn test_reply_to_multipart() {
        let bot = Bot::new("TOKEN");
        let document = InputFile::memory("hello world!");

        let real_reply_req = bot
            .send_document(ChatId(1234), document.clone())
            .reply_parameters(ReplyParameters::new(MessageId(1)));
        let sugar_reply_req = bot.send_document(ChatId(1234), document).reply_to(MessageId(1));

        assert_eq!(
            real_reply_req.deref().reply_parameters,
            sugar_reply_req.deref().reply_parameters
        )
    }

    #[test]
    fn test_disable_link_preview() {
        let link_preview_options = LinkPreviewOptions {
            is_disabled: true,
            url: None,
            prefer_small_media: false,
            prefer_large_media: false,
            show_above_text: false,
        };
        let bot = Bot::new("TOKEN");

        let real_link_req =
            bot.send_message(ChatId(1234), "test").link_preview_options(link_preview_options);
        let sugar_link_req = bot.send_message(ChatId(1234), "test").disable_link_preview(true);

        assert_eq!(real_link_req.deref(), sugar_link_req.deref())
    }
}
