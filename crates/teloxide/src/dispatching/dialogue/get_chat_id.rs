use crate::types::{
    BotCommandScope, CallbackQuery, Chat, ChatId, ChatJoinRequest, ChatMemberUpdated, Message,
    MessageCommon, Recipient, ResponseParameters, TargetMessage, Update, UpdateKind,
};

/// Something that may have a chat ID.
pub trait GetChatId {
    #[must_use]
    fn chat_id(&self) -> Option<ChatId>;
}

impl GetChatId for CallbackQuery {
    fn chat_id(&self) -> Option<ChatId> {
        self.message.as_ref().map(|mes| mes.chat.id)
    }
}

impl GetChatId for MessageCommon {
    fn chat_id(&self) -> Option<ChatId> {
        self.sender_chat.as_ref().map(|chat| chat.id)
    }
}

impl GetChatId for Update {
    fn chat_id(&self) -> Option<ChatId> {
        self.chat().map(|chat| chat.id)
    }
}

impl GetChatId for Recipient {
    fn chat_id(&self) -> Option<ChatId> {
        match self {
            Recipient::Id(chat_id) => Some(*chat_id),
            Recipient::ChannelUsername(_) => None,
        }
    }
}

impl GetChatId for BotCommandScope {
    fn chat_id(&self) -> Option<ChatId> {
        match self {
            BotCommandScope::Default
            | BotCommandScope::AllPrivateChats
            | BotCommandScope::AllGroupChats
            | BotCommandScope::AllChatAdministrators => None,
            BotCommandScope::Chat { chat_id: recipient }
            | BotCommandScope::ChatAdministrators { chat_id: recipient }
            | BotCommandScope::ChatMember { chat_id: recipient, .. } => recipient.chat_id(),
        }
    }
}

impl GetChatId for Chat {
    fn chat_id(&self) -> Option<ChatId> {
        Some(self.id)
    }
}

impl GetChatId for UpdateKind {
    fn chat_id(&self) -> Option<ChatId> {
        match self {
            UpdateKind::Message(message)
            | UpdateKind::EditedMessage(message)
            | UpdateKind::ChannelPost(message)
            | UpdateKind::EditedChannelPost(message) => GetChatId::chat_id(message),
            UpdateKind::CallbackQuery(callback_query) => callback_query.chat_id(),
            UpdateKind::MyChatMember(chat_member_updated) => chat_member_updated.chat_id(),
            UpdateKind::ChatMember(chat_member_updated) => chat_member_updated.chat_id(),
            UpdateKind::ChatJoinRequest(chat_join_request) => chat_join_request.chat_id(),
            UpdateKind::InlineQuery(_)
            | UpdateKind::ChosenInlineResult(_)
            | UpdateKind::ShippingQuery(_)
            | UpdateKind::PreCheckoutQuery(_)
            | UpdateKind::Poll(_)
            | UpdateKind::PollAnswer(_)
            | UpdateKind::Error(_) => None,
        }
    }
}

impl GetChatId for ResponseParameters {
    fn chat_id(&self) -> Option<ChatId> {
        match self {
            ResponseParameters::MigrateToChatId(chat_id) => Some(*chat_id),
            ResponseParameters::RetryAfter(_) => None,
        }
    }
}

impl GetChatId for TargetMessage {
    fn chat_id(&self) -> Option<ChatId> {
        match self {
            TargetMessage::Common { chat_id: recipient, .. } => recipient.chat_id(),
            TargetMessage::Inline { .. } => None,
        }
    }
}

/// Implements [`GetChatId`] for all types passed in, as long as they have a
/// `chat` field with a `Chat` type
macro_rules! impl_GetChatId_for_chat_field {
    // Comma-separated list of types bound to `t`
    ($($t:ty),* $(,)?) => {
        $(
            impl GetChatId for $t {
                fn chat_id(&self) -> Option<ChatId> {
                    Some(self.chat.id)
                }
            }
        )*
    };
}

impl_GetChatId_for_chat_field!(Message, ChatMemberUpdated, ChatJoinRequest);
