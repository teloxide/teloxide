use crate::{
    bot::Bot,
    requests::{
        AnswerPreCheckoutQuery, AnswerShippingQuery,
        EditMessageLiveLocation, ForwardMessage, GetFile, GetMe,
        KickChatMember, PinChatMessage, PromoteChatMember, RestrictChatMember,
        SendAudio, SendChatAction, SendContact, SendLocation, SendMediaGroup,
        SendMessage, SendPhoto, SendPoll, SendVenue, SendVideoNote, SendVoice,
        StopMessageLiveLocation, UnbanChatMember, UnpinChatMessage,
    },
    types::{ChatPermissions, InputFile, InputMedia, ChatAction, ChatId},
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

    pub fn answer_pre_checkout_query<I, O>(
        &self,
        pre_checkout_query_id: I,
        ok: O,
    ) -> AnswerPreCheckoutQuery
    where
        I: Into<String>,
        O: Into<bool>,
    {
        AnswerPreCheckoutQuery::new(
            self.ctx(),
            pre_checkout_query_id.into(),
            ok.into(),
        )
    }

    pub fn answer_shipping_query<I, O>(
        &self,
        shipping_query_id: I,
        ok: O,
    ) -> AnswerShippingQuery
    where
        I: Into<String>,
        O: Into<bool>,
    {
        AnswerShippingQuery::new(
            self.ctx(),
            shipping_query_id.into(),
            ok.into(),
        )
    }

    pub fn kick_chat_member<C, U>(
        &self,
        chat_id: C,
        user_id: U,
    ) -> KickChatMember
    where
        C: Into<ChatId>,
        U: Into<i32>,
    {
        KickChatMember::new(self.ctx(), chat_id.into(), user_id.into())
    }

    pub fn pin_chat_message<C, M>(
        &self,
        chat_id: C,
        message_id: M,
    ) -> PinChatMessage
    where
        C: Into<ChatId>,
        M: Into<i32>,
    {
        PinChatMessage::new(self.ctx(), chat_id.into(), message_id.into())
    }

    pub fn promote_chat_member<C, U>(
        &self,
        chat_id: C,
        user_id: U,
    ) -> PromoteChatMember
    where
        C: Into<ChatId>,
        U: Into<i32>,
    {
        PromoteChatMember::new(self.ctx(), chat_id.into(), user_id.into())
    }

    pub fn restrict_chat_member<C, U, P>(
        &self,
        chat_id: C,
        user_id: U,
        permissions: P,
    ) -> RestrictChatMember
    where
        C: Into<ChatId>,
        U: Into<i32>,
        P: Into<ChatPermissions>,
    {
        RestrictChatMember::new(
            self.ctx(),
            chat_id.into(),
            user_id.into(),
            permissions.into(),
        )
    }

    pub fn send_chat_action<C, A>(
        &self,
        chat_id: C,
        action: A,
    ) -> SendChatAction
    where
        C: Into<ChatId>,
        A: Into<ChatAction>,
    {
        SendChatAction::new(self.ctx(), chat_id.into(), action.into())
    }

    pub fn send_contact<C, P, F>(
        &self,
        chat_id: C,
        phone_number: P,
        first_name: F,
    ) -> SendContact
    where
        C: Into<ChatId>,
        P: Into<String>,
        F: Into<String>,
    {
        SendContact::new(
            self.ctx(),
            chat_id.into(),
            phone_number.into(),
            first_name.into(),
        )
    }

    pub fn send_poll<C, Q, O>(
        &self,
        chat_id: C,
        question: Q,
        options: O,
    ) -> SendPoll
    where
        C: Into<ChatId>,
        Q: Into<String>,
        O: Into<Vec<String>>,
    {
        SendPoll::new(
            self.ctx(),
            chat_id.into(),
            question.into(),
            options.into(),
        )
    }

    pub fn send_venue<C, Lt, Lg, T, A>(
        &self,
        chat_id: C,
        latitude: Lt,
        longitude: Lg,
        title: T,
        address: A,
    ) -> SendVenue
    where
        C: Into<ChatId>,
        Lt: Into<f64>,
        Lg: Into<f64>,
        T: Into<String>,
        A: Into<String>,
    {
        SendVenue::new(
            self.ctx(),
            chat_id.into(),
            latitude.into(),
            longitude.into(),
            title.into(),
            address.into(),
        )
    }

    pub fn send_video_note<C, V>(
        &self,
        chat_id: C,
        video_note: V,
    ) -> SendVideoNote
    where
        C: Into<ChatId>,
        V: Into<String>, // TODO: InputFile
    {
        SendVideoNote::new(self.ctx(), chat_id.into(), video_note.into())
    }

    pub fn send_voice<C, V>(&self, chat_id: C, voice: V) -> SendVoice
    where
        C: Into<ChatId>,
        V: Into<String>, // TODO: InputFile
    {
        SendVoice::new(self.ctx(), chat_id.into(), voice.into())
    }

    pub fn unban_chat_member<C, U>(
        &self,
        chat_id: C,
        user_id: U,
    ) -> UnbanChatMember
    where
        C: Into<ChatId>,
        U: Into<i32>,
    {
        UnbanChatMember::new(self.ctx(), chat_id.into(), user_id.into())
    }

    pub fn unpin_chat_message<C>(&self, chat_id: C) -> UnpinChatMessage
    where
        C: Into<ChatId>,
    {
        UnpinChatMessage::new(self.ctx(), chat_id.into())
    }
}
