use crate::{
    requests::{
        AddStickerToSet, AnswerCallbackQuery, AnswerInlineQuery,
        AnswerPreCheckoutQuery, AnswerShippingQuery, CreateNewStickerSet,
        DeleteChatPhoto, DeleteChatStickerSet, DeleteMessage,
        DeleteStickerFromSet, DeleteWebhook, EditMessageCaption,
        EditMessageCaptionInline, EditMessageLiveLocation,
        EditMessageLiveLocationInline, EditMessageMedia,
        EditMessageMediaInline, EditMessageReplyMarkup,
        EditMessageReplyMarkupInline, EditMessageText, EditMessageTextInline,
        ExportChatInviteLink, ForwardMessage, GetChat, GetChatAdministrators,
        GetChatMember, GetChatMembersCount, GetFile, GetGameHighScores,
        GetGameHighScoresInline, GetMe, GetStickerSet, GetUpdates,
        GetUserProfilePhotos, GetWebhookInfo, KickChatMember, LeaveChat,
        PinChatMessage, PromoteChatMember, RestrictChatMember, SendAnimation,
        SendAudio, SendChatAction, SendChatActionKind, SendContact,
        SendDocument, SendGame, SendInvoice, SendLocation, SendMediaGroup,
        SendMessage, SendPhoto, SendPoll, SendSticker, SendVenue, SendVideo,
        SendVideoNote, SendVoice, SetChatAdministratorCustomTitle,
        SetChatDescription, SetChatPermissions, SetChatPhoto,
        SetChatStickerSet, SetChatTitle, SetGameScore, SetGameScoreInline,
        SetStickerPositionInSet, SetWebhook, StopMessageLiveLocation,
        StopMessageLiveLocationInline, StopPoll, UnbanChatMember,
        UnpinChatMessage, UploadStickerFile,
    },
    types::{
        ChatId, ChatPermissions, InlineQueryResult, InputFile, InputMedia,
        LabeledPrice,
    },
    Bot,
};

impl Bot {
    pub fn get_updates(&self) -> GetUpdates {
        GetUpdates::new(self)
    }

    pub fn set_webhook<U>(&self, url: U) -> SetWebhook
    where
        U: Into<String>,
    {
        SetWebhook::new(self, url)
    }

    pub fn delete_webhook(&self) -> DeleteWebhook {
        DeleteWebhook::new(self)
    }

    pub fn get_webhook_info(&self) -> GetWebhookInfo {
        GetWebhookInfo::new(self)
    }

    pub fn get_me(&self) -> GetMe {
        GetMe::new(self)
    }

    pub fn send_message<C, T>(&self, chat_id: C, text: T) -> SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SendMessage::new(self, chat_id, text)
    }

    pub fn forward_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: i32,
    ) -> ForwardMessage
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        ForwardMessage::new(self, chat_id, from_chat_id, message_id)
    }

    pub fn send_photo<C>(&self, chat_id: C, photo: InputFile) -> SendPhoto
    where
        C: Into<ChatId>,
    {
        SendPhoto::new(self, chat_id, photo)
    }

    pub fn send_audio<C>(&self, chat_id: C, audio: InputFile) -> SendAudio
    where
        C: Into<ChatId>,
    {
        SendAudio::new(self, chat_id, audio)
    }

    pub fn send_document<C>(
        &self,
        chat_id: C,
        document: InputFile,
    ) -> SendDocument
    where
        C: Into<ChatId>,
    {
        SendDocument::new(self, chat_id, document)
    }

    pub fn send_video<C>(&self, chat_id: C, video: InputFile) -> SendVideo
    where
        C: Into<ChatId>,
    {
        SendVideo::new(self, chat_id, video)
    }

    pub fn send_animation<C>(
        &self,
        chat_id: C,
        animation: InputFile,
    ) -> SendAnimation
    where
        C: Into<ChatId>,
    {
        SendAnimation::new(self, chat_id, animation)
    }

    pub fn send_voice<C>(&self, chat_id: C, voice: InputFile) -> SendVoice
    where
        C: Into<ChatId>,
    {
        SendVoice::new(self, chat_id, voice)
    }

    pub fn send_video_note<C>(
        &self,
        chat_id: C,
        video_note: InputFile,
    ) -> SendVideoNote
    where
        C: Into<ChatId>,
    {
        SendVideoNote::new(self, chat_id, video_note)
    }

    pub fn send_media_group<C, M>(&self, chat_id: C, media: M) -> SendMediaGroup
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        SendMediaGroup::new(self, chat_id, media)
    }

    pub fn send_location<C>(
        &self,
        chat_id: C,
        latitude: f32,
        longitude: f32,
    ) -> SendLocation
    where
        C: Into<ChatId>,
    {
        SendLocation::new(self, chat_id, latitude, longitude)
    }

    pub fn edit_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
        latitude: f32,
        longitude: f32,
    ) -> EditMessageLiveLocationInline
    where
        I: Into<String>,
    {
        EditMessageLiveLocationInline::new(
            self,
            inline_message_id,
            latitude,
            longitude,
        )
    }

    pub fn edit_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
        latitude: f32,
        longitude: f32,
    ) -> EditMessageLiveLocation
    where
        C: Into<ChatId>,
    {
        EditMessageLiveLocation::new(
            self, chat_id, message_id, latitude, longitude,
        )
    }

    pub fn stop_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
    ) -> StopMessageLiveLocationInline
    where
        I: Into<String>,
    {
        StopMessageLiveLocationInline::new(self, inline_message_id)
    }

    pub fn stop_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> StopMessageLiveLocation
    where
        C: Into<ChatId>,
    {
        StopMessageLiveLocation::new(self, chat_id, message_id)
    }

    pub fn send_venue<C, T, A>(
        &self,
        chat_id: C,
        latitude: f32,
        longitude: f32,
        title: T,
        address: A,
    ) -> SendVenue
    where
        C: Into<ChatId>,
        T: Into<String>,
        A: Into<String>,
    {
        SendVenue::new(self, chat_id, latitude, longitude, title, address)
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
        SendContact::new(self, chat_id, phone_number, first_name)
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
        SendPoll::new(self, chat_id, question, options)
    }

    pub fn send_chat_action<C>(
        &self,
        chat_id: C,
        action: SendChatActionKind,
    ) -> SendChatAction
    where
        C: Into<ChatId>,
    {
        SendChatAction::new(self, chat_id, action)
    }

    pub fn get_user_profile_photos(
        &self,
        user_id: i32,
    ) -> GetUserProfilePhotos {
        GetUserProfilePhotos::new(self, user_id)
    }

    pub fn get_file<F>(&self, file_id: F) -> GetFile
    where
        F: Into<String>,
    {
        GetFile::new(self, file_id)
    }

    pub fn kick_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
    ) -> KickChatMember
    where
        C: Into<ChatId>,
    {
        KickChatMember::new(self, chat_id, user_id)
    }

    pub fn unban_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
    ) -> UnbanChatMember
    where
        C: Into<ChatId>,
    {
        UnbanChatMember::new(self, chat_id, user_id)
    }

    pub fn restrict_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
        permissions: ChatPermissions,
    ) -> RestrictChatMember
    where
        C: Into<ChatId>,
    {
        RestrictChatMember::new(self, chat_id, user_id, permissions)
    }

    pub fn promote_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
    ) -> PromoteChatMember
    where
        C: Into<ChatId>,
    {
        PromoteChatMember::new(self, chat_id, user_id)
    }

    pub fn set_chat_permissions<C>(
        &self,
        chat_id: C,
        permissions: ChatPermissions,
    ) -> SetChatPermissions
    where
        C: Into<ChatId>,
    {
        SetChatPermissions::new(self, chat_id, permissions)
    }

    pub fn export_chat_invite_link<C>(&self, chat_id: C) -> ExportChatInviteLink
    where
        C: Into<ChatId>,
    {
        ExportChatInviteLink::new(self, chat_id)
    }

    pub fn set_chat_photo<C>(
        &self,
        chat_id: C,
        photo: InputFile,
    ) -> SetChatPhoto
    where
        C: Into<ChatId>,
    {
        SetChatPhoto::new(self, chat_id, photo)
    }

    pub fn delete_chat_photo<C>(&self, chat_id: C) -> DeleteChatPhoto
    where
        C: Into<ChatId>,
    {
        DeleteChatPhoto::new(self, chat_id)
    }

    pub fn set_chat_title<C, T>(&self, chat_id: C, title: T) -> SetChatTitle
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SetChatTitle::new(self, chat_id, title)
    }

    pub fn set_chat_description<C>(&self, chat_id: C) -> SetChatDescription
    where
        C: Into<ChatId>,
    {
        SetChatDescription::new(self, chat_id)
    }

    pub fn pin_chat_message<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> PinChatMessage
    where
        C: Into<ChatId>,
    {
        PinChatMessage::new(self, chat_id, message_id)
    }

    pub fn unpin_chat_message<C>(&self, chat_id: C) -> UnpinChatMessage
    where
        C: Into<ChatId>,
    {
        UnpinChatMessage::new(self, chat_id)
    }

    pub fn leave_chat<C>(&self, chat_id: C) -> LeaveChat
    where
        C: Into<ChatId>,
    {
        LeaveChat::new(self, chat_id)
    }

    pub fn get_chat<C>(&self, chat_id: C) -> GetChat
    where
        C: Into<ChatId>,
    {
        GetChat::new(self, chat_id)
    }

    pub fn get_chat_administrators<C>(
        &self,
        chat_id: C,
    ) -> GetChatAdministrators
    where
        C: Into<ChatId>,
    {
        GetChatAdministrators::new(self, chat_id)
    }

    pub fn get_chat_members_count<C>(&self, chat_id: C) -> GetChatMembersCount
    where
        C: Into<ChatId>,
    {
        GetChatMembersCount::new(self, chat_id)
    }

    pub fn get_chat_member<C>(&self, chat_id: C, user_id: i32) -> GetChatMember
    where
        C: Into<ChatId>,
    {
        GetChatMember::new(self, chat_id, user_id)
    }

    pub fn set_chat_sticker_set<C, S>(
        &self,
        chat_id: C,
        sticker_set_name: S,
    ) -> SetChatStickerSet
    where
        C: Into<ChatId>,
        S: Into<String>,
    {
        SetChatStickerSet::new(self, chat_id, sticker_set_name)
    }

    pub fn delete_chat_sticker_set<C>(&self, chat_id: C) -> DeleteChatStickerSet
    where
        C: Into<ChatId>,
    {
        DeleteChatStickerSet::new(self, chat_id)
    }

    pub fn answer_callback_query<C>(
        &self,
        callback_query_id: C,
    ) -> AnswerCallbackQuery
    where
        C: Into<String>,
    {
        AnswerCallbackQuery::new(self, callback_query_id)
    }

    pub fn edit_message_text_inline<I, T>(
        &self,
        inline_message_id: I,
        text: T,
    ) -> EditMessageTextInline
    where
        I: Into<String>,
        T: Into<String>,
    {
        EditMessageTextInline::new(self, inline_message_id, text)
    }

    pub fn edit_message_text<C, T>(
        &self,
        chat_id: C,
        message_id: i32,
        text: T,
    ) -> EditMessageText
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        EditMessageText::new(self, chat_id, message_id, text)
    }

    pub fn edit_message_caption_inline<I>(
        &self,
        inline_message_id: I,
    ) -> EditMessageCaptionInline
    where
        I: Into<String>,
    {
        EditMessageCaptionInline::new(self, inline_message_id)
    }

    pub fn edit_message_caption<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> EditMessageCaption
    where
        C: Into<ChatId>,
    {
        EditMessageCaption::new(self, chat_id, message_id)
    }

    pub fn edit_message_media_inline<I>(
        &self,
        inline_message_id: I,
        media: InputMedia,
    ) -> EditMessageMediaInline
    where
        I: Into<String>,
    {
        EditMessageMediaInline::new(self, inline_message_id, media)
    }

    pub fn edit_message_media<C>(
        &self,
        chat_id: C,
        message_id: i32,
        media: InputMedia,
    ) -> EditMessageMedia
    where
        C: Into<ChatId>,
    {
        EditMessageMedia::new(self, chat_id, message_id, media)
    }

    pub fn edit_message_reply_markup_inline<I>(
        &self,
        inline_message_id: I,
    ) -> EditMessageReplyMarkupInline
    where
        I: Into<String>,
    {
        EditMessageReplyMarkupInline::new(self, inline_message_id)
    }

    pub fn edit_message_reply_markup<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> EditMessageReplyMarkup
    where
        C: Into<ChatId>,
    {
        EditMessageReplyMarkup::new(self, chat_id, message_id)
    }

    pub fn stop_poll<C>(&self, chat_id: C, message_id: i32) -> StopPoll
    where
        C: Into<ChatId>,
    {
        StopPoll::new(self, chat_id, message_id)
    }

    pub fn delete_message<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> DeleteMessage
    where
        C: Into<ChatId>,
    {
        DeleteMessage::new(self, chat_id, message_id)
    }

    pub fn send_sticker<C>(&self, chat_id: C, sticker: InputFile) -> SendSticker
    where
        C: Into<ChatId>,
    {
        SendSticker::new(self, chat_id, sticker)
    }

    pub fn get_sticker_set<N>(&self, name: N) -> GetStickerSet
    where
        N: Into<String>,
    {
        GetStickerSet::new(self, name)
    }

    pub fn upload_sticker_file(
        &self,
        user_id: i32,
        png_sticker: InputFile,
    ) -> UploadStickerFile {
        UploadStickerFile::new(self, user_id, png_sticker)
    }

    pub fn create_new_sticker_set<N, T, E>(
        &self,
        user_id: i32,
        name: N,
        title: T,
        png_sticker: InputFile,
        emojis: E,
    ) -> CreateNewStickerSet
    where
        N: Into<String>,
        T: Into<String>,
        E: Into<String>,
    {
        CreateNewStickerSet::new(
            self,
            user_id,
            name,
            title,
            png_sticker,
            emojis,
        )
    }

    pub fn add_sticker_to_set<N, E>(
        &self,
        user_id: i32,
        name: N,
        png_sticker: InputFile,
        emojis: E,
    ) -> AddStickerToSet
    where
        N: Into<String>,
        E: Into<String>,
    {
        AddStickerToSet::new(self, user_id, name, png_sticker, emojis)
    }

    pub fn set_sticker_position_in_set<S>(
        &self,
        sticker: S,
        position: i32,
    ) -> SetStickerPositionInSet
    where
        S: Into<String>,
    {
        SetStickerPositionInSet::new(self, sticker, position)
    }

    pub fn delete_sticker_from_set<S>(&self, sticker: S) -> DeleteStickerFromSet
    where
        S: Into<String>,
    {
        DeleteStickerFromSet::new(self, sticker)
    }

    pub fn answer_inline_query<I, R>(
        &self,
        inline_query_id: I,
        results: R,
    ) -> AnswerInlineQuery
    where
        I: Into<String>,
        R: Into<Vec<InlineQueryResult>>,
    {
        AnswerInlineQuery::new(self, inline_query_id, results)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn send_invoice<T, D, Pl, Pt, S, C, Pr>(
        &self,
        chat_id: i32,
        title: T,
        description: D,
        payload: Pl,
        provider_token: Pt,
        start_parameter: S,
        currency: C,
        prices: Pr,
    ) -> SendInvoice
    where
        T: Into<String>,
        D: Into<String>,
        Pl: Into<String>,
        Pt: Into<String>,
        S: Into<String>,
        C: Into<String>,
        Pr: Into<Vec<LabeledPrice>>,
    {
        SendInvoice::new(
            self,
            chat_id,
            title,
            description,
            payload,
            provider_token,
            start_parameter,
            currency,
            prices,
        )
    }

    pub fn answer_shipping_query<S>(
        &self,
        shipping_query_id: S,
        ok: bool,
    ) -> AnswerShippingQuery
    where
        S: Into<String>,
    {
        AnswerShippingQuery::new(self, shipping_query_id, ok)
    }

    pub fn answer_pre_checkout_query<P>(
        &self,
        pre_checkout_query_id: P,
        ok: bool,
    ) -> AnswerPreCheckoutQuery
    where
        P: Into<String>,
    {
        AnswerPreCheckoutQuery::new(self, pre_checkout_query_id, ok)
    }

    pub fn send_game<G>(&self, chat_id: i32, game_short_name: G) -> SendGame
    where
        G: Into<String>,
    {
        SendGame::new(self, chat_id, game_short_name)
    }

    pub fn set_game_score_inline<I>(
        &self,
        inline_message_id: I,
        user_id: i32,
        score: i32,
    ) -> SetGameScoreInline
    where
        I: Into<String>,
    {
        SetGameScoreInline::new(self, inline_message_id, user_id, score)
    }

    pub fn set_game_score<C>(
        &self,
        chat_id: C,
        message_id: i32,
        user_id: i32,
        score: i32,
    ) -> SetGameScore
    where
        C: Into<ChatId>,
    {
        SetGameScore::new(self, chat_id, message_id, user_id, score)
    }

    pub fn get_game_high_scores_inline<I>(
        &self,
        inline_message_id: I,
        user_id: i32,
    ) -> GetGameHighScoresInline
    where
        I: Into<String>,
    {
        GetGameHighScoresInline::new(self, inline_message_id, user_id)
    }

    pub fn get_game_high_scores<C>(
        &self,
        chat_id: C,
        message_id: i32,
        user_id: i32,
    ) -> GetGameHighScores
    where
        C: Into<ChatId>,
    {
        GetGameHighScores::new(self, chat_id, message_id, user_id)
    }

    pub fn set_chat_administrator_custom_title<C, CT>(
        &self,
        chat_id: C,
        user_id: i32,
        custom_title: CT,
    ) -> SetChatAdministratorCustomTitle
    where
        C: Into<ChatId>,
        CT: Into<String>,
    {
        SetChatAdministratorCustomTitle::new(
            self,
            chat_id,
            user_id,
            custom_title,
        )
    }
}
