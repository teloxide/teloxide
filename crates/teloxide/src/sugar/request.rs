//! Additions to multiple [`payloads`].
//!
//! [`payloads`]: teloxide_core::payloads

use teloxide_core::{payloads::*, types::*};

macro_rules! impl_request_reply_ext {
    ($(($t:ty, $trait:path)),*) => {
        $(
            impl<T> RequestReplyExt<$t> for T
            where
                T: $trait
            {
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
    ($(($t:ty, $trait:path)),*) => {
        $(
            impl<T> RequestLinkPreviewExt<$t> for T
            where
                T: $trait
            {
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
pub trait RequestReplyExt<P> {
    /// Replaces `.reply_parameters(ReplyParameters::new(msg.id))`
    /// with `.reply_to(msg.id)` or `.reply_to(msg)`
    fn reply_to<M>(self, message_id: M) -> Self
    where
        M: Into<MessageId>,
        Self: Sized;
}

/// `.disable_link_preview(is_disabled)` syntax sugar for requests.
pub trait RequestLinkPreviewExt<P> {
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

// NOTE: struct in (struct, trait) is mandatory, as well as the blank <P>
// generic in Request*Ext traits.
// Or you will get `error[E0119]: conflicting implementations of trait`.
// This is a Rust issue. See:
// https://github.com/rust-lang/rust/issues/20400
// https://github.com/rust-lang/rfcs/pull/1672

impl_request_reply_ext! {
    (SendDice, SendDiceSetters),
    (SendInvoice, SendInvoiceSetters),
    (SendPoll, SendPollSetters),
    (SendContact, SendContactSetters),
    (SendGame, SendGameSetters),
    (SendVenue, SendVenueSetters),
    (SendLocation, SendLocationSetters),
    (CopyMessage, CopyMessageSetters),
    (SendMessage, SendMessageSetters),
    (SendSticker, SendStickerSetters),
    (SendMediaGroup, SendMediaGroupSetters),
    (SendAnimation, SendAnimationSetters),
    (SendVideoNote, SendVideoNoteSetters),
    (SendVideo, SendVideoSetters),
    (SendDocument, SendDocumentSetters),
    (SendAudio, SendAudioSetters),
    (SendVoice, SendVoiceSetters),
    (SendPhoto, SendPhotoSetters)
}

impl_request_link_preview_ext! {
    (SendMessage, SendMessageSetters),
    (EditMessageText, EditMessageTextSetters)
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    use teloxide_core::{prelude::*, Bot};

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
    #[cfg(feature = "trace-adaptor")]
    fn test_reply_to_adaptors() {
        use teloxide_core::{adaptors::trace::Settings, requests::HasPayload};

        let bot = Bot::new("TOKEN").trace(Settings::empty()).parse_mode(ParseMode::Html);

        let real_reply_req = bot
            .send_message(ChatId(1234), "test")
            .reply_parameters(ReplyParameters::new(MessageId(1)));
        let sugar_reply_req = bot.send_message(ChatId(1234), "test").reply_to(MessageId(1));

        assert_eq!(
            real_reply_req.payload_ref().payload_ref().reply_parameters,
            sugar_reply_req.payload_ref().payload_ref().reply_parameters
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

    #[test]
    #[cfg(feature = "trace-adaptor")]
    fn test_disable_link_preview_adaptors() {
        use teloxide_core::{adaptors::trace::Settings, requests::HasPayload};

        let link_preview_options = LinkPreviewOptions {
            is_disabled: true,
            url: None,
            prefer_small_media: false,
            prefer_large_media: false,
            show_above_text: false,
        };
        let bot = Bot::new("TOKEN").trace(Settings::empty()).parse_mode(ParseMode::Html);

        let real_link_req =
            bot.send_message(ChatId(1234), "test").link_preview_options(link_preview_options);
        let sugar_link_req = bot.send_message(ChatId(1234), "test").disable_link_preview(true);

        assert_eq!(
            real_link_req.payload_ref().payload_ref().reply_parameters,
            sugar_link_req.payload_ref().payload_ref().reply_parameters
        )
    }
}
