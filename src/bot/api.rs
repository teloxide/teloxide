use crate::{
    requests::{
        json, multipart,
        payloads::{GetFile, GetMe, GetUpdates, SendAnimation, SendMessage},
    },
    types::{ChatId, InputFile},
    Bot,
};

impl Bot {
    // Methods are sorted as in tg docs (https://core.telegram.org/bots/api)

    // Getting updates
    /// For tg-method documentation see [`GetUpdates`]
    ///
    /// [`GetUpdates`]: crate::requests::payloads::GetUpdates
    pub fn get_updates(&self) -> json::Request<GetUpdates> {
        json::Request::new(self, GetUpdates::new())
    }

    // Available methods
    /// For tg-method documentation see [`GetMe`]
    ///
    /// [`GetMe`]: crate::requests::payloads::GetMe
    pub fn get_me(&self) -> json::Request<GetMe> {
        json::Request::new(self, GetMe {})
    }

    /// For tg-method documentation see [`SendMessage`]
    ///
    /// [`SendMessage`]: crate::requests::payloads::SendMessage
    pub fn send_message<C, T>(
        &self,
        chat_id: C,
        text: T,
    ) -> json::Request<SendMessage>
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        json::Request::new(self, SendMessage::new(chat_id, text))
    }

    /// For tg-method documentation see [`SendAnimation`]
    ///
    /// [`SendAnimation`]: crate::requests::payloads::SendAnimation
    pub fn send_animation<C>(
        &self,
        chat_id: C,
        animation: InputFile,
    ) -> multipart::Request<SendAnimation>
    where
        C: Into<ChatId>,
    {
        multipart::Request::new(self, SendAnimation::new(chat_id, animation))
    }

    /// For tg-method documentation see [`GetFile`]
    ///
    /// [`GetFile`]: crate::requests::payloads::GetFile
    pub fn get_file<F>(&self, file_id: F) -> json::Request<GetFile>
    where
        F: Into<String>,
    {
        json::Request::new(self, GetFile::new(file_id))
    }
}
