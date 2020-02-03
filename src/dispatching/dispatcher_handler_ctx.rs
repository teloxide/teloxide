use crate::{
    dispatching::session::GetChatId,
    requests::{Request, ResponseResult},
    types::Message,
    Bot,
};
use std::sync::Arc;
use crate::types::{ChatId, InputFile, InputMedia};

/// A [`Dispatcher`]'s handler's context of a bot and an update.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub struct DispatcherHandlerCtx<Upd> {
    pub bot: Arc<Bot>,
    pub update: Upd,
}

impl<Upd> GetChatId for DispatcherHandlerCtx<Upd>
where
    Upd: GetChatId,
{
    fn chat_id(&self) -> i64 {
        self.update.chat_id()
    }
}

impl DispatcherHandlerCtx<Message> {
    pub async fn answer<T>(&self, text: T) -> ResponseResult<Message>
    where
        T: Into<String>,
    {
        self.bot
            .send_message(self.chat_id(), text)
            .send()
            .await
    }

    pub async fn reply_to<T>(&self, text: T) -> ResponseResult<Message>
    where
        T: Into<String>
    {
        self.bot
            .send_message(self.chat_id(), text)
            .reply_to_message_id(self.update.id)
            .send()
            .await
    }

    pub async fn answer_photo(&self, photo: InputFile) -> ResponseResult<Message>
    {
        self.bot
            .send_photo(self.update.chat.id, photo)
            .send()
            .await
    }

    pub async fn answer_audio(&self, audio: InputFile) -> ResponseResult<Message>
    {
        self.bot
            .send_audio(self.update.chat.id, audio)
            .send()
            .await
    }

    pub async fn answer_animation(&self, animation: InputFile) -> ResponseResult<Message>
    {
        self.bot
            .send_animation(self.update.chat.id, animation)
            .send()
            .await
    }

    pub async fn answer_document(&self, document: InputFile) -> ResponseResult<Message>
    {
        self.bot
            .send_document(self.update.chat.id, document)
            .send()
            .await
    }

    pub async fn answer_video(&self, video: InputFile) -> ResponseResult<Message>
    {
        self.bot
            .send_video(self.update.chat.id, video)
            .send()
            .await
    }

    pub async fn answer_voice(&self, voice: InputFile) -> ResponseResult<Message>
    {
        self.bot
            .send_voice(self.update.chat.id, voice)
            .send()
            .await
    }

    pub async fn answer_media_group<T>(&self, media_group: T) -> ResponseResult<Vec<Message>>
    where
        T: Into<Vec<InputMedia>>
    {
        self.bot
            .send_media_group(self.update.chat.id, T)
            .send()
            .await
    }

    pub async fn answer_location(&self, latitude: f32, longitude: f32) -> ResponseResult<Message>
    {
        self.bot
            .send_location(self.update.chat.id, latitude, longitude)
            .send()
            .await
    }

    pub async fn answer_venue<T, U>(&self, latitude: f32, longitude: f32, title: T, address: U) -> ResponseResult<Message>
    where
        T: Into<String>,
        U: Into<String>
    {
        self.bot
            .send_venue(self.update.chat.id, latitude, longitude, title, address)
            .send()
            .await
    }

    pub async fn answer_video_note(&self, video_note: InputFile) -> ResponseResult<Message>
    {
        self.bot
            .send_video_note(self.update.chat.id, video_note)
            .send()
            .await
    }

    pub async fn answer_contact<T, U>(&self, phone_number: T, first_name: U) -> ResponseResult<Message>
    where
        T: Into<String>,
        U: Into<String>
    {
        self.bot
            .send_contact(self.chat_id(), phone_number, first_name)
            .send()
            .await
    }

    pub async fn forward_to<T>(&self, chat_id: T) -> ResponseResult<Message>
    where
        T: Into<ChatId>
    {
        self.bot
            .forward_message(chat_id, self.update.chat.id, self.update.id)
            .send()
            .await
    }
}
