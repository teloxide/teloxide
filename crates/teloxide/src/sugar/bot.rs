//! Additions to [`Bot`].
//!
//! [`Bot`]: crate::Bot
use crate::{prelude::*, types::*};
use teloxide_core::{payloads::*, requests::JsonRequest};

/// Adds useful manipulations with [`Message`] structs
///
/// [`Message`]: crate::types::Message
pub trait BotMessagesExt {
    /// This function is the same as [`Bot::forward_message`],
    /// but can take in [`Message`] to forward it.
    ///
    /// [`Bot::forward_message`]: crate::Bot::forward_message
    /// [`Message`]: crate::types::Message
    fn forward<C>(&self, to_chat_id: C, message: Message) -> JsonRequest<ForwardMessage>
    where
        C: Into<Recipient>;

    /// This function is the same as [`Bot::copy_message`],
    /// but can take in [`Message`] to copy it.
    ///
    /// [`Bot::copy_messages`]: crate::Bot::copy_message
    /// [`Message`]: crate::types::Message
    fn copy<C>(&self, to_chat_id: C, message: Message) -> JsonRequest<CopyMessage>
    where
        C: Into<Recipient>;

    /// This function is the same as [`Bot::delete_message`],
    /// but can take in [`Message`] to delete it.
    ///
    /// [`Bot::delete_message`]: crate::Bot::delete_message
    /// [`Message`]: crate::types::Message
    fn delete(&self, message: Message) -> JsonRequest<DeleteMessage>;
}

impl BotMessagesExt for Bot {
    fn forward<C>(&self, to_chat_id: C, message: Message) -> JsonRequest<ForwardMessage>
    where
        C: Into<Recipient>,
    {
        self.forward_message(to_chat_id, message.chat.id, message.id)
    }

    fn copy<C>(&self, to_chat_id: C, message: Message) -> JsonRequest<CopyMessage>
    where
        C: Into<Recipient>,
    {
        self.copy_message(to_chat_id, message.chat.id, message.id)
    }

    fn delete(&self, message: Message) -> JsonRequest<DeleteMessage> {
        self.delete_message(message.chat.id, message.id)
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

    #[test]
    fn test_forward() {
        let bot = Bot::new("TOKEN");

        let to_chat_id = ChatId(12345);
        let from_chat_id = ChatId(6789);
        let message_id = MessageId(100);

        let sugar_forward_req = bot.forward(to_chat_id, make_message(from_chat_id, message_id));
        let real_forward_req = bot.forward_message(to_chat_id, from_chat_id, message_id);

        assert_eq!(sugar_forward_req.deref(), real_forward_req.deref())
    }

    #[test]
    fn test_copy() {
        let bot = Bot::new("TOKEN");

        let to_chat_id = ChatId(12345);
        let from_chat_id = ChatId(6789);
        let message_id = MessageId(100);

        let sugar_copy_req = bot.copy(to_chat_id, make_message(from_chat_id, message_id));
        let real_copy_req = bot.copy_message(to_chat_id, from_chat_id, message_id);

        assert_eq!(sugar_copy_req.deref(), real_copy_req.deref())
    }

    #[test]
    fn test_delete() {
        let bot = Bot::new("TOKEN");

        let chat_id = ChatId(6789);
        let message_id = MessageId(100);

        let sugar_delete_req = bot.delete(make_message(chat_id, message_id));
        let real_delete_req = bot.delete_message(chat_id, message_id);

        assert_eq!(sugar_delete_req.deref(), real_delete_req.deref())
    }

    #[test]
    fn message_to_message_id() {
        // Just to make sure message still can be in Into<MessageId>
        let message = make_message(ChatId(1), MessageId(1));
        let message_id: MessageId = message.into();
        assert_eq!(message_id, MessageId(1));
    }
}
