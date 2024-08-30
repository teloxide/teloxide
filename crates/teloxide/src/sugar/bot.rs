//! Additions to [`Bot`].
//!
//! [`Bot`]: crate::Bot
use crate::{prelude::*, types::*};
use std::collections::HashSet;
use teloxide_core::{payloads::*, requests::JsonRequest};

/// Adds useful manipulations with [`Message`] structs
///
/// [`Message`]: crate::types::Message
pub trait BotMessagesExt {
    /// This function is the same as [`Bot::forward_messages`],
    /// but can take in [`Message`], including just one.
    ///
    /// [`Bot::forward_messages`]: crate::Bot::forward_messages
    /// [`Message`]: crate::types::Message
    fn forward<C, M>(&self, to_chat_id: C, messages: M) -> JsonRequest<ForwardMessages>
    where
        C: Into<Recipient>,
        M: IntoIterator<Item = Message>;

    /// This function is the same as [`Bot::copy_messages`],
    /// but can take in [`Message`], including just one.
    ///
    /// [`Bot::copy_messages`]: crate::Bot::copy_messages
    /// [`Message`]: crate::types::Message
    fn copy<C, M>(&self, to_chat_id: C, messages: M) -> JsonRequest<CopyMessages>
    where
        C: Into<Recipient>,
        M: IntoIterator<Item = Message>;

    /// This function is the same as [`Bot::delete_messages`],
    /// but can take in [`Message`], including just one.
    ///
    /// [`Bot::delete_messages`]: crate::Bot::delete_messages
    /// [`Message`]: crate::types::Message
    fn delete<M>(&self, messages: M) -> JsonRequest<DeleteMessages>
    where
        M: IntoIterator<Item = Message>;
}

fn compress_chat_messages<M>(messages: M) -> (ChatId, Vec<MessageId>)
where
    M: IntoIterator<Item = Message>,
{
    let (message_ids, unique_chat_ids): (Vec<MessageId>, HashSet<ChatId>) =
        messages.into_iter().map(|m| (m.id, m.chat.id)).unzip();

    if unique_chat_ids.is_empty() {
        panic!("There needs to be at least one message!");
    } else if unique_chat_ids.len() > 1 {
        panic!(
            "Messages shouldn't come from different chats! Current chat ids: {:?}",
            unique_chat_ids.into_iter().map(|c| c.0).collect::<Vec<i64>>()
        );
    }

    // Unwrap: length is checked to be non-zero before
    let chat_id = unique_chat_ids.into_iter().next().unwrap();

    (chat_id, message_ids)
}

impl BotMessagesExt for Bot {
    fn forward<C, M>(&self, to_chat_id: C, messages: M) -> JsonRequest<ForwardMessages>
    where
        C: Into<Recipient>,
        M: IntoIterator<Item = Message>,
    {
        let (from_chat_id, message_ids) = compress_chat_messages(messages);
        self.forward_messages(to_chat_id, from_chat_id, message_ids)
    }

    fn copy<C, M>(&self, to_chat_id: C, messages: M) -> JsonRequest<CopyMessages>
    where
        C: Into<Recipient>,
        M: IntoIterator<Item = Message>,
    {
        let (from_chat_id, message_ids) = compress_chat_messages(messages);
        self.copy_messages(to_chat_id, from_chat_id, message_ids)
    }

    fn delete<M>(&self, messages: M) -> JsonRequest<DeleteMessages>
    where
        M: IntoIterator<Item = Message>,
    {
        let (chat_id, message_ids) = compress_chat_messages(messages);
        self.delete_messages(chat_id, message_ids)
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
        let message_ids = vec![MessageId(100), MessageId(101), MessageId(102)];

        let sugar_forward_req = bot.forward(
            to_chat_id,
            vec![
                make_message(from_chat_id, message_ids[0]),
                make_message(from_chat_id, message_ids[1]),
                make_message(from_chat_id, message_ids[2]),
            ],
        );
        let real_forward_req = bot.forward_messages(to_chat_id, from_chat_id, message_ids);

        assert_eq!(sugar_forward_req.deref(), real_forward_req.deref())
    }

    #[test]
    fn test_copy() {
        let bot = Bot::new("TOKEN");

        let to_chat_id = ChatId(12345);
        let from_chat_id = ChatId(6789);
        let message_ids = vec![MessageId(100), MessageId(101), MessageId(102)];

        let sugar_copy_req = bot.copy(
            to_chat_id,
            vec![
                make_message(from_chat_id, message_ids[0]),
                make_message(from_chat_id, message_ids[1]),
                make_message(from_chat_id, message_ids[2]),
            ],
        );
        let real_copy_req = bot.copy_messages(to_chat_id, from_chat_id, message_ids);

        assert_eq!(sugar_copy_req.deref(), real_copy_req.deref())
    }

    #[test]
    fn test_delete() {
        let bot = Bot::new("TOKEN");

        let chat_id = ChatId(6789);
        let message_ids = vec![MessageId(100), MessageId(101), MessageId(102)];

        let sugar_delete_req = bot.delete(vec![
            make_message(chat_id, message_ids[0]),
            make_message(chat_id, message_ids[1]),
            make_message(chat_id, message_ids[2]),
        ]);
        let real_delete_req = bot.delete_messages(chat_id, message_ids);

        assert_eq!(sugar_delete_req.deref(), real_delete_req.deref())
    }

    #[test]
    #[should_panic]
    fn test_forward_many_chats() {
        // They all use the same validation, only one check is enough
        let bot = Bot::new("TOKEN");

        let _ = bot.forward(
            ChatId(12345),
            vec![
                make_message(ChatId(6789), MessageId(100)),
                make_message(ChatId(6789), MessageId(101)),
                make_message(ChatId(9012), MessageId(102)),
            ],
        );
    }

    #[test]
    fn message_to_iterator() {
        // Just to make sure one message still can be passed in
        let message = make_message(ChatId(1), MessageId(1));
        assert_eq!(message.clone().into_iter().next(), Some(message));
    }

    #[test]
    fn message_to_message_id() {
        // Just to make sure message still can be in Into<MessageId>
        let message = make_message(ChatId(1), MessageId(1));
        let message_id: MessageId = message.into();
        assert_eq!(message_id, MessageId(1));
    }
}
