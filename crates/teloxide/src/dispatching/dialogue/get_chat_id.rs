use crate::types::{
    BotCommandScope, CallbackQuery, Chat, ChatId, ChatJoinRequest, ChatMemberUpdated, Message,
    MessageCommon, Recipient, ResponseParameters, TargetMessage, Update,
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

impl GetChatId for Message {
    fn chat_id(&self) -> Option<ChatId> {
        Some(self.chat.id)
    }
}

impl GetChatId for ChatMemberUpdated {
    fn chat_id(&self) -> Option<ChatId> {
        Some(self.chat.id)
    }
}

impl GetChatId for ChatJoinRequest {
    fn chat_id(&self) -> Option<ChatId> {
        Some(self.chat.id)
    }
}
