use crate::{
    bot::Bot,
    requests::{
        edit_message_live_location::EditMessageLiveLocation,
        forward_message::ForwardMessage, get_file::GetFile, get_me::GetMe,
        send_audio::SendAudio, send_location::SendLocation,
        send_media_group::SendMediaGroup, send_message::SendMessage,
        send_photo::SendPhoto,
        stop_message_live_location::StopMessageLiveLocation, ChatId,
    },
    types::{InputFile, InputMedia},
};

/// Telegram functions
impl Bot {
    pub fn get_me(&self) -> GetMe {
        GetMe::new(self.ctx())
    }

    pub fn send_message<C, T>(&self, chat_id: C, text: T) -> SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SendMessage::new(self.ctx(), chat_id.into(), text.into())
    }

    pub fn edit_message_live_location<Lt, Lg>(
        &self,
        latitude: Lt,
        longitude: Lg,
    ) -> EditMessageLiveLocation
    where
        Lt: Into<f64>,
        Lg: Into<f64>,
    {
        EditMessageLiveLocation::new(
            self.ctx(),
            latitude.into(),
            longitude.into(),
        )
    }

    pub fn forward_message<C, F, M>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: M,
    ) -> ForwardMessage
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
        M: Into<i32>,
    {
        ForwardMessage::new(
            self.ctx(),
            chat_id.into(),
            from_chat_id.into(),
            message_id.into(),
        )
    }

    pub fn send_audio<C, A>(&self, chat_id: C, audio: A) -> SendAudio
    where
        C: Into<ChatId>,
        A: Into<InputFile>,
    {
        SendAudio::new(self.ctx(), chat_id.into(), audio.into())
    }

    pub fn send_location<C, Lt, Lg>(
        &self,
        chat_id: C,
        latitude: Lt,
        longitude: Lg,
    ) -> SendLocation
    where
        C: Into<ChatId>,
        Lt: Into<f64>,
        Lg: Into<f64>,
    {
        SendLocation::new(
            self.ctx(),
            chat_id.into(),
            latitude.into(),
            longitude.into(),
        )
    }

    pub fn send_media_group<C, M>(&self, chat_id: C, media: M) -> SendMediaGroup
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        SendMediaGroup::new(self.ctx(), chat_id.into(), media.into())
    }

    pub fn send_photo<C, P>(&self, chat_id: C, photo: P) -> SendPhoto
    where
        C: Into<ChatId>,
        P: Into<InputFile>,
    {
        SendPhoto::new(self.ctx(), chat_id.into(), photo.into())
    }

    pub fn stop_message_live_location(&self) -> StopMessageLiveLocation {
        StopMessageLiveLocation::new(self.ctx())
    }

    pub fn get_file<F>(&self, file_id: F) -> GetFile
    where
        F: Into<String>,
    {
        GetFile::new(self.ctx(), file_id.into())
    }
}
