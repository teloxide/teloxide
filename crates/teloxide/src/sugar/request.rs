//! Additions to [`JsonRequest`].
//!
//! [`JsonRequest`]: teloxide_core::requests::JsonRequest

use teloxide_core::{payloads::*, prelude::Requester, types::*, Bot};

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
                fn disable_link_preview(self) -> Self
                where
                    Self: Sized
                {
                    let link_preview_options = LinkPreviewOptions {
                        is_disabled: true,
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

pub trait RequestReplyExt {
    fn reply_to<M>(self, message_id: M) -> Self
    where
        M: Into<MessageId>,
        Self: Sized;
}

pub trait RequestLinkPreviewExt {
    fn disable_link_preview(self) -> Self
    where
        Self: Sized;
}

impl_request_reply_ext! {
    <Bot as Requester>::SendMessage
}

impl_request_link_preview_ext! {
    <Bot as Requester>::SendMessage
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
        let sugar_link_req = bot.send_message(ChatId(1234), "test").disable_link_preview();

        assert_eq!(real_link_req.deref(), sugar_link_req.deref())
    }
}
