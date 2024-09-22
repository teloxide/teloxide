//! Additions to [`Bot`].
//!
//! [`Bot`]: crate::Bot
use crate::{prelude::*, types::*};
use teloxide_core::{
    payloads::*,
    requests::{JsonRequest, MultipartRequest},
};

/// Syntax sugar for [`Message`] manipulations.
///
/// [`Message`]: crate::types::Message
pub trait BotMessagesExt {
    /// This function is the same as [`Bot::forward_message`],
    /// but can take in [`Message`] to forward it.
    ///
    /// [`Bot::forward_message`]: crate::Bot::forward_message
    /// [`Message`]: crate::types::Message
    fn forward<C>(&self, to_chat_id: C, message: &Message) -> JsonRequest<ForwardMessage>
    where
        C: Into<Recipient>;

    /// This function is the same as [`Bot::edit_message_live_location`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_live_location`]: crate::Bot::edit_message_live_location
    /// [`Message`]: crate::types::Message
    fn edit_live_location(
        &self,
        message: &Message,
        latitude: f64,
        longitude: f64,
    ) -> JsonRequest<EditMessageLiveLocation>;

    /// This function is the same as [`Bot::stop_message_live_location`],
    /// but can take in [`Message`] to stop the live location in it.
    ///
    /// [`Bot::stop_message_live_location`]: crate::Bot::stop_message_live_location
    /// [`Message`]: crate::types::Message
    fn stop_live_location(&self, message: &Message) -> JsonRequest<StopMessageLiveLocation>;

    /// This function is the same as [`Bot::set_message_reaction`],
    /// but can take in [`Message`] to set a reaction on it.
    ///
    /// [`Bot::set_message_reaction`]: crate::Bot::set_message_reaction
    /// [`Message`]: crate::types::Message
    fn set_reaction(&self, message: &Message) -> JsonRequest<SetMessageReaction>;

    /// This function is the same as [`Bot::pin_chat_message`],
    /// but can take in [`Message`] to pin it.
    ///
    /// [`Bot::pin_chat_message`]: crate::Bot::pin_chat_message
    /// [`Message`]: crate::types::Message
    fn pin(&self, message: &Message) -> JsonRequest<PinChatMessage>;

    /// This function is the same as [`Bot::unpin_chat_message`],
    /// but can take in [`Message`] to unpin it.
    ///
    /// [`Bot::unpin_chat_message`]: crate::Bot::unpin_chat_message
    /// [`Message`]: crate::types::Message
    fn unpin(&self, message: &Message) -> JsonRequest<UnpinChatMessage>;

    /// This function is the same as [`Bot::edit_message_text`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_text`]: crate::Bot::edit_message_text
    /// [`Message`]: crate::types::Message
    fn edit_text<T>(&self, message: &Message, text: T) -> JsonRequest<EditMessageText>
    where
        T: Into<String>;

    /// This function is the same as [`Bot::edit_message_caption`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_caption`]: crate::Bot::edit_message_caption
    /// [`Message`]: crate::types::Message
    fn edit_caption(&self, message: &Message) -> JsonRequest<EditMessageCaption>;

    /// This function is the same as [`Bot::edit_message_media`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_media`]: crate::Bot::edit_message_media
    /// [`Message`]: crate::types::Message
    fn edit_media(
        &self,
        message: &Message,
        media: InputMedia,
    ) -> MultipartRequest<EditMessageMedia>;

    /// This function is the same as [`Bot::edit_message_reply_markup`],
    /// but can take in [`Message`] to edit it.
    ///
    /// [`Bot::edit_message_reply_markup`]: crate::Bot::edit_message_reply_markup
    /// [`Message`]: crate::types::Message
    fn edit_reply_markup(&self, message: &Message) -> JsonRequest<EditMessageReplyMarkup>;

    /// This function is the same as [`Bot::stop_poll`],
    /// but can take in [`Message`] to stop the poll in it.
    ///
    /// [`Bot::stop_poll`]: crate::Bot::stop_poll
    /// [`Message`]: crate::types::Message
    fn stop_poll_message(&self, message: &Message) -> JsonRequest<StopPoll>;

    /// This function is the same as [`Bot::delete_message`],
    /// but can take in [`Message`] to delete it.
    ///
    /// [`Bot::delete_message`]: crate::Bot::delete_message
    /// [`Message`]: crate::types::Message
    fn delete(&self, message: &Message) -> JsonRequest<DeleteMessage>;

    /// This function is the same as [`Bot::copy_message`],
    /// but can take in [`Message`] to copy it.
    ///
    /// [`Bot::copy_messages`]: crate::Bot::copy_message
    /// [`Message`]: crate::types::Message
    fn copy<C>(&self, to_chat_id: C, message: &Message) -> JsonRequest<CopyMessage>
    where
        C: Into<Recipient>;
}

impl BotMessagesExt for Bot {
    fn forward<C>(&self, to_chat_id: C, message: &Message) -> JsonRequest<ForwardMessage>
    where
        C: Into<Recipient>,
    {
        self.forward_message(to_chat_id, message.chat.id, message.id)
    }
    fn edit_live_location(
        &self,
        message: &Message,
        latitude: f64,
        longitude: f64,
    ) -> JsonRequest<EditMessageLiveLocation> {
        self.edit_message_live_location(message.chat.id, message.id, latitude, longitude)
    }

    fn stop_live_location(&self, message: &Message) -> JsonRequest<StopMessageLiveLocation> {
        self.stop_message_live_location(message.chat.id, message.id)
    }

    fn set_reaction(&self, message: &Message) -> JsonRequest<SetMessageReaction> {
        self.set_message_reaction(message.chat.id, message.id)
    }

    fn pin(&self, message: &Message) -> JsonRequest<PinChatMessage> {
        self.pin_chat_message(message.chat.id, message.id)
    }

    fn unpin(&self, message: &Message) -> JsonRequest<UnpinChatMessage> {
        self.unpin_chat_message(message.chat.id).message_id(message.id)
    }

    fn edit_text<T>(&self, message: &Message, text: T) -> JsonRequest<EditMessageText>
    where
        T: Into<String>,
    {
        self.edit_message_text(message.chat.id, message.id, text)
    }

    fn edit_caption(&self, message: &Message) -> JsonRequest<EditMessageCaption> {
        self.edit_message_caption(message.chat.id, message.id)
    }

    fn edit_media(
        &self,
        message: &Message,
        media: InputMedia,
    ) -> MultipartRequest<EditMessageMedia> {
        self.edit_message_media(message.chat.id, message.id, media)
    }

    fn edit_reply_markup(&self, message: &Message) -> JsonRequest<EditMessageReplyMarkup> {
        self.edit_message_reply_markup(message.chat.id, message.id)
    }

    fn stop_poll_message(&self, message: &Message) -> JsonRequest<StopPoll> {
        self.stop_poll(message.chat.id, message.id)
    }

    fn delete(&self, message: &Message) -> JsonRequest<DeleteMessage> {
        self.delete_message(message.chat.id, message.id)
    }

    fn copy<C>(&self, to_chat_id: C, message: &Message) -> JsonRequest<CopyMessage>
    where
        C: Into<Recipient>,
    {
        self.copy_message(to_chat_id, message.chat.id, message.id)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::ops::Deref;

    use chrono::DateTime;

    use super::*;

    pub(crate) fn make_message(chat_id: ChatId, message_id: MessageId) -> Message {
        let timestamp = 1_569_518_829;
        let date = DateTime::from_timestamp(timestamp, 0).unwrap();
        Message {
            via_bot: None,
            id: message_id,
            thread_id: None,
            from: Some(User {
                id: UserId(109_998_024),
                is_bot: false,
                first_name: String::from("Laster"),
                last_name: None,
                username: Some(String::from("laster_alex")),
                language_code: Some(String::from("en")),
                is_premium: false,
                added_to_attachment_menu: false,
            }),
            sender_chat: None,
            is_topic_message: false,
            sender_business_bot: None,
            date,
            chat: Chat {
                id: chat_id,
                kind: ChatKind::Private(ChatPrivate {
                    username: Some(String::from("Laster")),
                    first_name: Some(String::from("laster_alex")),
                    last_name: None,
                    bio: None,
                    has_private_forwards: None,
                    has_restricted_voice_and_video_messages: None,
                    business_intro: None,
                    business_location: None,
                    business_opening_hours: None,
                    birthdate: None,
                    personal_chat: None,
                }),
                photo: None,
                available_reactions: None,
                pinned_message: None,
                message_auto_delete_time: None,
                has_hidden_members: false,
                has_aggressive_anti_spam_enabled: false,
                chat_full_info: ChatFullInfo::default(),
            },
            kind: MessageKind::Common(MessageCommon {
                reply_to_message: None,
                forward_origin: None,
                external_reply: None,
                quote: None,
                edit_date: None,
                media_kind: MediaKind::Text(MediaText {
                    text: "text".to_owned(),
                    entities: vec![],
                    link_preview_options: None,
                }),
                reply_markup: None,
                author_signature: None,
                is_automatic_forward: false,
                has_protected_content: false,
                reply_to_story: None,
                sender_boost_count: None,
                is_from_offline: false,
                business_connection_id: None,
            }),
        }
    }

    const TO_CHAT_ID: ChatId = ChatId(12345);
    const FROM_CHAT_ID: ChatId = ChatId(6789);
    const CHAT_ID: ChatId = ChatId(12345);
    const MESSAGE_ID: MessageId = MessageId(100);

    #[test]
    fn test_forward() {
        let bot = Bot::new("TOKEN");

        let sugar_req = bot.forward(TO_CHAT_ID, &make_message(FROM_CHAT_ID, MESSAGE_ID));
        let real_req = bot.forward_message(TO_CHAT_ID, FROM_CHAT_ID, MESSAGE_ID);

        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_edit_live_location() {
        let bot = Bot::new("TOKEN");

        let longitude = 1.0;
        let latitude = 1.0;

        let sugar_req =
            bot.edit_live_location(&make_message(CHAT_ID, MESSAGE_ID), latitude, longitude);
        let real_req = bot.edit_message_live_location(CHAT_ID, MESSAGE_ID, latitude, longitude);
        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_stop_live_location() {
        let bot = Bot::new("TOKEN");

        let sugar_req = bot.stop_live_location(&make_message(CHAT_ID, MESSAGE_ID));
        let real_req = bot.stop_message_live_location(CHAT_ID, MESSAGE_ID);
        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_set_reaction() {
        let bot = Bot::new("TOKEN");

        let sugar_req = bot.set_reaction(&make_message(CHAT_ID, MESSAGE_ID));
        let real_req = bot.set_message_reaction(CHAT_ID, MESSAGE_ID);
        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_pin() {
        let bot = Bot::new("TOKEN");

        let sugar_req = bot.pin(&make_message(CHAT_ID, MESSAGE_ID));
        let real_req = bot.pin_chat_message(CHAT_ID, MESSAGE_ID);
        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_unpin() {
        let bot = Bot::new("TOKEN");

        let sugar_req = bot.unpin(&make_message(CHAT_ID, MESSAGE_ID));
        let real_req = bot.unpin_chat_message(CHAT_ID).message_id(MESSAGE_ID);
        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_edit_text() {
        let bot = Bot::new("TOKEN");

        let text = "text";

        let sugar_req = bot.edit_text(&make_message(CHAT_ID, MESSAGE_ID), text);
        let real_req = bot.edit_message_text(CHAT_ID, MESSAGE_ID, text);
        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_edit_caption() {
        let bot = Bot::new("TOKEN");

        let sugar_req = bot.edit_caption(&make_message(CHAT_ID, MESSAGE_ID));
        let real_req = bot.edit_message_caption(CHAT_ID, MESSAGE_ID);
        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_edit_media() {
        let bot = Bot::new("TOKEN");

        let media =
            InputMedia::Document(InputMediaDocument::new(InputFile::memory("Hello World!")));

        let sugar_req = bot.edit_media(&make_message(CHAT_ID, MESSAGE_ID), media.clone());
        let real_req = bot.edit_message_media(CHAT_ID, MESSAGE_ID, media);
        assert_eq!(sugar_req.deref().chat_id, real_req.deref().chat_id);
        assert_eq!(sugar_req.deref().message_id, real_req.deref().message_id);
    }

    #[test]
    fn test_edit_reply_markup() {
        let bot = Bot::new("TOKEN");

        let sugar_req = bot.edit_reply_markup(&make_message(CHAT_ID, MESSAGE_ID));
        let real_req = bot.edit_message_reply_markup(CHAT_ID, MESSAGE_ID);
        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_stop_poll_message() {
        let bot = Bot::new("TOKEN");

        let sugar_req = bot.stop_poll_message(&make_message(CHAT_ID, MESSAGE_ID));
        let real_req = bot.stop_poll(CHAT_ID, MESSAGE_ID);
        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_delete() {
        let bot = Bot::new("TOKEN");

        let sugar_req = bot.delete(&make_message(CHAT_ID, MESSAGE_ID));
        let real_req = bot.delete_message(CHAT_ID, MESSAGE_ID);

        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn test_copy() {
        let bot = Bot::new("TOKEN");

        let sugar_req = bot.copy(TO_CHAT_ID, &make_message(FROM_CHAT_ID, MESSAGE_ID));
        let real_req = bot.copy_message(TO_CHAT_ID, FROM_CHAT_ID, MESSAGE_ID);

        assert_eq!(sugar_req.deref(), real_req.deref())
    }

    #[test]
    fn message_to_message_id() {
        // Just to make sure message still can be in Into<MessageId>
        let message = make_message(ChatId(1), MessageId(1));
        let message_id: MessageId = message.into();
        assert_eq!(message_id, MessageId(1));
    }
}
