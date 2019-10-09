use std::borrow::Cow;

use crate::{
    bot::Bot,
    requests::{
        AnswerPreCheckoutQuery, AnswerShippingQuery, ChatAction, ChatId,
        EditMessageLiveLocation, ForwardMessage, GetFile, GetMe,
        KickChatMember, PinChatMessage, PromoteChatMember, RestrictChatMember,
        SendAudio, SendChatAction, SendContact, SendLocation, SendMediaGroup,
        SendMessage, SendPhoto, SendPoll, SendVenue, SendVideoNote, SendVoice,
        StopMessageLiveLocation, UnbanChatMember, UnpinChatMessage,
    },
    types::{ChatPermissions, InputFile, InputMedia},
};

/// Telegram functions
impl Bot {
    pub fn get_me(&self) -> GetMe {
        GetMe::new(self.ctx())
    }

    pub fn send_message<'a, C, T>(&'a self, chat_id: C, text: T) -> SendMessage
    where
        T: Into<Cow<'a, str>>,
        C: Into<ChatId<'a>>,
    {
        SendMessage::new(self.ctx(), chat_id, text)
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

    pub fn forward_message<'a, C, F, M>(
        &'a self,
        chat_id: C,
        from_chat_id: C,
        message_id: M,
    ) -> ForwardMessage
    where
        C: Into<ChatId<'a>>,
        M: Into<i32>,
    {
        ForwardMessage::new(
            self.ctx(),
            chat_id.into(),
            from_chat_id.into(),
            message_id.into(),
        )
    }

    pub fn send_audio<'a, C, A>(&'a self, chat_id: C, audio: A) -> SendAudio
    where
        C: Into<ChatId<'a>>,
        A: Into<InputFile<'a>>,
    {
        SendAudio::new(self.ctx(), chat_id.into(), audio.into())
    }

    pub fn send_location<'a, C, Lt, Lg>(
        &'a self,
        chat_id: C,
        latitude: Lt,
        longitude: Lg,
    ) -> SendLocation
    where
        C: Into<ChatId<'a>>,
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

    pub fn send_media_group<'a, C, M>(
        &'a self,
        chat_id: C,
        media: M,
    ) -> SendMediaGroup
    where
        C: Into<ChatId<'a>>,
        M: Into<InputMedia<'a>>,
    {
        SendMediaGroup::new(self.ctx(), chat_id.into(), media.into())
    }

    pub fn send_photo<'a, C, P>(&'a self, chat_id: C, photo: P) -> SendPhoto
    where
        C: Into<ChatId<'a>>,
        P: Into<InputFile<'a>>,
    {
        SendPhoto::new(self.ctx(), chat_id.into(), photo.into())
    }

    pub fn stop_message_live_location(&self) -> StopMessageLiveLocation {
        StopMessageLiveLocation::new(self.ctx())
    }

    pub fn get_file<'a, C>(&'a self, file_id: C) -> GetFile<'a>
    where
        C: Into<Cow<'a, str>>,
    {
        GetFile::new(self.ctx(), file_id.into())
    }

    pub fn answer_pre_checkout_query<'a, I, O>(
        &'a self,
        pre_checkout_query_id: I,
        ok: O,
    ) -> AnswerPreCheckoutQuery
    where
        I: Into<Cow<'a, str>>,
        O: Into<bool>,
    {
        AnswerPreCheckoutQuery::new(
            self.ctx(),
            pre_checkout_query_id.into(),
            ok.into(),
        )
    }

    pub fn answer_shipping_query<'a, I, O>(
        &'a self,
        shipping_query_id: I,
        ok: O,
    ) -> AnswerShippingQuery
    where
        I: Into<Cow<'a, str>>,
        O: Into<bool>,
    {
        AnswerShippingQuery::new(
            self.ctx(),
            shipping_query_id.into(),
            ok.into(),
        )
    }

    pub fn kick_chat_member<'a, C, U>(
        &'a self,
        chat_id: C,
        user_id: U,
    ) -> KickChatMember
    where
        C: Into<ChatId<'a>>,
        U: Into<i32>,
    {
        KickChatMember::new(self.ctx(), chat_id.into(), user_id.into())
    }

    pub fn pin_chat_message<'a, C, M>(
        &'a self,
        chat_id: C,
        message_id: M,
    ) -> PinChatMessage
    where
        C: Into<ChatId<'a>>,
        M: Into<i32>,
    {
        PinChatMessage::new(self.ctx(), chat_id.into(), message_id.into())
    }

    pub fn promote_chat_member<'a, C, U>(
        &'a self,
        chat_id: C,
        user_id: U,
    ) -> PromoteChatMember
    where
        C: Into<ChatId<'a>>,
        U: Into<i32>,
    {
        PromoteChatMember::new(self.ctx(), chat_id.into(), user_id.into())
    }

    pub fn restrict_chat_member<'a, C, U, P>(
        &'a self,
        chat_id: C,
        user_id: U,
        permissions: P,
    ) -> RestrictChatMember
    where
        C: Into<ChatId<'a>>,
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

    pub fn send_chat_action<'a, C, A>(
        &'a self,
        chat_id: C,
        action: A,
    ) -> SendChatAction
    where
        C: Into<ChatId<'a>>,
        A: Into<ChatAction>,
    {
        SendChatAction::new(self.ctx(), chat_id.into(), action.into())
    }

    pub fn send_contact<'a, C, S>(
        &'a self,
        chat_id: C,
        phone_number: S,
        first_name: S,
    ) -> SendContact
    where
        C: Into<ChatId<'a>>,
        S: Into<Cow<'a, str>>,
    {
        SendContact::new(
            self.ctx(),
            chat_id.into(),
            phone_number.into(),
            first_name.into(),
        )
    }

    pub fn send_poll<'a, C, Q, O>(
        &self,
        chat_id: C,
        question: Q,
        options: O,
    ) -> SendPoll
    where
        C: Into<ChatId<'a>>,
        Q: Into<Cow<'a, str>>,
        O: Into<Cow<'a, [str]>>,
    {
        SendPoll::new(
            self.ctx(),
            chat_id.into(),
            question.into(),
            options.into(),
        )
    }

    pub fn send_venue<'a, C, Lt, Lg, T, A>(
        &'a self,
        chat_id: C,
        latitude: Lt,
        longitude: Lg,
        title: T,
        address: A,
    ) -> SendVenue
    where
        C: Into<ChatId<'a>>,
        Lt: Into<f64>,
        Lg: Into<f64>,
        T: Into<Cow<'a, str>>,
        A: Into<Cow<'a, str>>,
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

    pub fn send_video_note<'a, C, V>(
        &'a self,
        chat_id: C,
        video_note: V,
    ) -> SendVideoNote
    where
        C: Into<ChatId<'a>>,
        V: Into<Cow<'a, str>>, // TODO: InputFile
    {
        SendVideoNote::new(self.ctx(), chat_id.into(), video_note.into())
    }

    pub fn send_voice<'a, C, V>(&'a self, chat_id: C, voice: V) -> SendVoice
    where
        C: Into<ChatId<'a>>,
        V: Into<Cow<'a, str>>, // TODO: InputFile
    {
        SendVoice::new(self.ctx(), chat_id.into(), voice.into())
    }

    pub fn unban_chat_member<'a, C, U>(
        &'a self,
        chat_id: C,
        user_id: U,
    ) -> UnbanChatMember
    where
        C: Into<ChatId<'a>>,
        U: Into<i32>,
    {
        UnbanChatMember::new(self.ctx(), chat_id.into(), user_id.into())
    }

    pub fn unpin_chat_message<'a, C>(&'a self, chat_id: C) -> UnpinChatMessage
    where
        C: Into<ChatId<'a>>,
    {
        UnpinChatMessage::new(self.ctx(), chat_id.into())
    }
}
