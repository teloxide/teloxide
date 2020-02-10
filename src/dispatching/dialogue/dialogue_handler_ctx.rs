use crate::{
    dispatching::dialogue::{Dialogue, GetChatId},
    requests::{
        DeleteMessage, EditMessageCaption, EditMessageText, ForwardMessage,
        PinChatMessage, SendAnimation, SendAudio, SendContact, SendDocument,
        SendLocation, SendMediaGroup, SendMessage, SendPhoto, SendSticker,
        SendVenue, SendVideo, SendVideoNote, SendVoice,
    },
    types::{ChatId, ChatOrInlineMessage, InputFile, InputMedia, Message},
    Bot,
};
use std::sync::Arc;

/// A context of a [`DialogueDispatcher`]'s message handler.
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
pub struct DialogueHandlerCtx<Upd, State, T> {
    pub bot: Arc<Bot>,
    pub update: Upd,
    pub dialogue: Dialogue<State, T>,
}

/// Sets a new state.
///
/// Use it like this: `state!(ctx, State::RequestAge)`, where `ctx` is
/// [`DialogueHandlerCtx<Upd, State, T>`] and `State::RequestAge` is of type
/// `State`.
///
/// [`DialogueHandlerCtx<Upd, State, T>`]:
/// crate::dispatching::dialogue::DialogueHandlerCtx
#[macro_export]
macro_rules! state {
    ($ctx:ident, $state:expr) => {
        $ctx.dialogue.state = $state;
    };
}

impl<Upd, State, T> GetChatId for DialogueHandlerCtx<Upd, State, T>
where
    Upd: GetChatId,
{
    fn chat_id(&self) -> i64 {
        self.update.chat_id()
    }
}

impl<State, Data> DialogueHandlerCtx<Message, State, Data> {
    pub fn answer<T>(&self, text: T) -> SendMessage
    where
        T: Into<String>,
    {
        self.bot.send_message(self.chat_id(), text)
    }

    pub fn reply_to<T>(&self, text: T) -> SendMessage
    where
        T: Into<String>,
    {
        self.bot
            .send_message(self.chat_id(), text)
            .reply_to_message_id(self.update.id)
    }

    pub fn answer_photo(&self, photo: InputFile) -> SendPhoto {
        self.bot.send_photo(self.update.chat.id, photo)
    }

    pub fn answer_audio(&self, audio: InputFile) -> SendAudio {
        self.bot.send_audio(self.update.chat.id, audio)
    }

    pub fn answer_animation(&self, animation: InputFile) -> SendAnimation {
        self.bot.send_animation(self.update.chat.id, animation)
    }

    pub fn answer_document(&self, document: InputFile) -> SendDocument {
        self.bot.send_document(self.update.chat.id, document)
    }

    pub fn answer_video(&self, video: InputFile) -> SendVideo {
        self.bot.send_video(self.update.chat.id, video)
    }

    pub fn answer_voice(&self, voice: InputFile) -> SendVoice {
        self.bot.send_voice(self.update.chat.id, voice)
    }

    pub fn answer_media_group<T>(&self, media_group: T) -> SendMediaGroup
    where
        T: Into<Vec<InputMedia>>,
    {
        self.bot.send_media_group(self.update.chat.id, media_group)
    }

    pub fn answer_location(
        &self,
        latitude: f32,
        longitude: f32,
    ) -> SendLocation {
        self.bot
            .send_location(self.update.chat.id, latitude, longitude)
    }

    pub fn answer_venue<T, U>(
        &self,
        latitude: f32,
        longitude: f32,
        title: T,
        address: U,
    ) -> SendVenue
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.bot.send_venue(
            self.update.chat.id,
            latitude,
            longitude,
            title,
            address,
        )
    }

    pub fn answer_video_note(&self, video_note: InputFile) -> SendVideoNote {
        self.bot.send_video_note(self.update.chat.id, video_note)
    }

    pub fn answer_contact<T, U>(
        &self,
        phone_number: T,
        first_name: U,
    ) -> SendContact
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.bot
            .send_contact(self.chat_id(), phone_number, first_name)
    }

    pub fn answer_sticker<T>(&self, sticker: InputFile) -> SendSticker {
        self.bot.send_sticker(self.update.chat.id, sticker)
    }

    pub fn forward_to<T>(&self, chat_id: T) -> ForwardMessage
    where
        T: Into<ChatId>,
    {
        self.bot
            .forward_message(chat_id, self.update.chat.id, self.update.id)
    }

    pub fn edit_message_text<T>(&self, text: T) -> EditMessageText
    where
        T: Into<String>,
    {
        self.bot.edit_message_text(
            ChatOrInlineMessage::Chat {
                chat_id: self.update.chat.id.into(),
                message_id: self.update.id,
            },
            text,
        )
    }

    pub fn edit_message_caption(&self) -> EditMessageCaption {
        self.bot.edit_message_caption(ChatOrInlineMessage::Chat {
            chat_id: self.update.chat.id.into(),
            message_id: self.update.id,
        })
    }

    pub fn delete_message(&self) -> DeleteMessage {
        self.bot.delete_message(self.update.chat.id, self.update.id)
    }

    pub fn pin_message(&self) -> PinChatMessage {
        self.bot
            .pin_chat_message(self.update.chat.id, self.update.id)
    }
}
