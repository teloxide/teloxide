use async_trait::async_trait;

use crate::bot::Bot;
use crate::types::{ChatId, ChatPermissions, True};
use crate::requests::{ResponseResult, Request};
use crate::network;

#[derive(Debug, Clone, Serialize)]
pub struct SetChatPermissions<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    chat_id: ChatId,
    permissions: ChatPermissions
}

#[async_trait]
impl Request for SetChatPermissions<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SetChatPermissions<'_> {
    async fn send(self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatPermissions",
            &self
        ).await
    }
}

impl<'a> SetChatPermissions<'a> {
    pub(crate) fn new<C, CP>(
        bot: &'a Bot,
        chat_id: C,
        permissions: CP,
    ) -> Self
    where
        C: Into<ChatId>,
        CP: Into<ChatPermissions>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            permissions: permissions.into(),
        }
    }

    pub fn chat_id<C>(mut self, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn permissions<CP>(mut self, permissions: CP) -> Self
    where
        CP: Into<ChatPermissions>
    {
        self.permissions = permissions.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let bot = Bot::new("token");
        let chat_id = 123;
        let permissions = ChatPermissions {
            can_send_messages: Some(true),
            can_send_media_messages: None,
            can_send_polls: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None
        };
        let method = SetChatPermissions::new(&bot, chat_id, permissions);

        let expected = r#"{"chat_id":123,"permissions":{"can_send_messages":true}}"#;
        let actual = serde_json::to_string::<SetChatPermissions>(&method).unwrap();
        assert_eq!(actual, expected);
    }
}
