use crate::dispatching::dialogue::GetChatId;
use teloxide_core::{
    payloads::SendMessageSetters,
    requests::{Request, Requester},
    types::{ChatId, InputFile, InputMedia, Message},
};

/// A [`Dispatcher`]'s handler's context of a bot and an update.
///
/// See the [module-level documentation](crate::dispatching) for the design
/// overview.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
#[derive(Debug)]
pub struct UpdateWithCx<R, Upd> {
    pub requester: R,
    pub update: Upd,
}

impl<Upd, R> GetChatId for UpdateWithCx<R, Upd>
where
    Upd: GetChatId,
{
    fn chat_id(&self) -> i64 {
        self.update.chat_id()
    }
}

#[doc(hidden)]
// Now it is used only inside `#[teloxide(subtransition)]` for type inference.
pub trait UpdateWithCxRequesterType {
    type Requester;
}

impl<R, Upd> UpdateWithCxRequesterType for UpdateWithCx<R, Upd> {
    type Requester = R;
}

impl<R> UpdateWithCx<R, Message>
where
    R: Requester,
{
    /// A shortcut for `.answer(text).send().await`.
    #[deprecated(note = "Use .answer(text).await instead")]
    pub async fn answer_str<T>(&self, text: T) -> Result<Message, R::Err>
    where
        T: Into<String>,
        R::SendMessage: std::future::Future,
    {
        self.answer(text).send().await
    }

    pub fn answer<T>(&self, text: T) -> R::SendMessage
    where
        T: Into<String>,
    {
        self.requester.send_message(self.chat_id(), text)
    }

    pub fn reply_to<T>(&self, text: T) -> R::SendMessage
    where
        T: Into<String>,
    {
        self.requester.send_message(self.chat_id(), text).reply_to_message_id(self.update.id)
    }

    pub fn answer_photo(&self, photo: InputFile) -> R::SendPhoto {
        self.requester.send_photo(self.update.chat.id, photo)
    }

    pub fn answer_audio(&self, audio: InputFile) -> R::SendAudio {
        self.requester.send_audio(self.update.chat.id, audio)
    }

    pub fn answer_animation(&self, animation: InputFile) -> R::SendAnimation {
        self.requester.send_animation(self.update.chat.id, animation)
    }

    pub fn answer_document(&self, document: InputFile) -> R::SendDocument {
        self.requester.send_document(self.update.chat.id, document)
    }

    pub fn answer_video(&self, video: InputFile) -> R::SendVideo {
        self.requester.send_video(self.update.chat.id, video)
    }

    pub fn answer_voice(&self, voice: InputFile) -> R::SendVoice {
        self.requester.send_voice(self.update.chat.id, voice)
    }

    pub fn answer_media_group<T>(&self, media_group: T) -> R::SendMediaGroup
    where
        T: IntoIterator<Item = InputMedia>,
    {
        self.requester.send_media_group(self.update.chat.id, media_group)
    }

    pub fn answer_location(&self, latitude: f64, longitude: f64) -> R::SendLocation {
        self.requester.send_location(self.update.chat.id, latitude, longitude)
    }

    pub fn answer_venue<T, U>(
        &self,
        latitude: f64,
        longitude: f64,
        title: T,
        address: U,
    ) -> R::SendVenue
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.requester.send_venue(self.update.chat.id, latitude, longitude, title, address)
    }

    pub fn answer_video_note(&self, video_note: InputFile) -> R::SendVideoNote {
        self.requester.send_video_note(self.update.chat.id, video_note)
    }

    pub fn answer_contact<T, U>(&self, phone_number: T, first_name: U) -> R::SendContact
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.requester.send_contact(self.chat_id(), phone_number, first_name)
    }

    pub fn answer_sticker(&self, sticker: InputFile) -> R::SendSticker {
        self.requester.send_sticker(self.update.chat.id, sticker)
    }

    pub fn forward_to<T>(&self, chat_id: T) -> R::ForwardMessage
    where
        T: Into<ChatId>,
    {
        self.requester.forward_message(chat_id, self.update.chat.id, self.update.id)
    }

    pub fn edit_message_text<T>(&self, text: T) -> R::EditMessageText
    where
        T: Into<String>,
    {
        self.requester.edit_message_text(self.update.chat.id, self.update.id, text)
    }

    pub fn edit_message_caption(&self) -> R::EditMessageCaption {
        self.requester.edit_message_caption(self.update.chat.id, self.update.id)
    }

    pub fn delete_message(&self) -> R::DeleteMessage {
        self.requester.delete_message(self.update.chat.id, self.update.id)
    }

    pub fn pin_message(&self) -> R::PinChatMessage {
        self.requester.pin_chat_message(self.update.chat.id, self.update.id)
    }

    pub fn answer_dice(&self) -> R::SendDice {
        self.requester.send_dice(self.update.chat.id)
    }
}
