use crate::{
    requests::{
        json, multipart,
        payloads::{
            AddStickerToSet, AnswerCallbackQuery, AnswerInlineQuery,
            AnswerPreCheckoutQuery, AnswerShippingQuery, CreateNewStickerSet,
            DeleteChatPhoto, DeleteChatStickerSet, DeleteMessage,
            DeleteStickerFromSet, DeleteWebhook, EditMessageCaption,
            EditMessageCaptionInline, EditMessageLiveLocation,
            EditMessageLiveLocationInline, EditMessageMedia,
            EditMessageMediaInline, EditMessageReplyMarkup,
            EditMessageReplyMarkupInline, EditMessageText,
            EditMessageTextInline, ExportChatInviteLink, ForwardMessage,
            GetChat, GetChatAdministrator, GetChatMember, GetChatMembersCount,
            GetFile, GetGameHighScore, GetGameHighScoreInline, GetMe,
            GetStickerSet, GetUpdates, GetUserProfilePhoto, GetWebhookInfo,
            KickChatMember, LeaveChat, PinChatMessage, PromoteChatMember,
            RestrictChatMember, SendAnimation, SendAudio, SendChatAction,
            SendContact, SendDocument, SendGame, SendInvoice, SendLocation,
            SendMediaGroup, SendMessage, SendPhoto, SendPoll, SendSticker,
            SendVenue, SendVideo, SendVideoNote, SendVoice,
            SetChatAdministratorCustomTitle, SetChatDescription,
            SetChatPermission, SetChatPhoto, SetChatStickerSet, SetChatTitle,
            SetGameScore, SetGameScoreInline, SetStickerPositionInSet,
            SetWebhook, StopMessageLiveLocation, StopMessageLiveLocationInline,
            StopPoll, UnbanChatMember, UnpinChatMessage, UploadStickerFile,
        },
    },
    types::{
        ChatId, ChatPermissions, InlineQueryResult, InputFile, InputMedia,
        LabeledPrice,
    },
    Bot,
};

impl Bot {
    /// For tg-method documentation see [`GetUpdate`]
    ///
    /// [`GetUpdate`]: crate::requests::payloads::GetUpdate
    pub fn get_updates(&self) -> json::Request<GetUpdates> {
        json::Request::new(self, GetUpdates::new())
    }

    /// For tg-method documentation see [`SetWebhook`]
    ///
    /// [`SetWebhook`]: crate::requests::payloads::SetWebhook
    pub fn set_webhook<U>(&self, url: U) -> json::Request<SetWebhook>
    where
        U: Into<String>,
    {
        json::Request::new(self, SetWebhook::new(url))
    }

    /// For tg-method documentation see [`DeleteWebhook`]
    ///
    /// [`DeleteWebhook`]: crate::requests::payloads::DeleteWebhook
    pub fn delete_webhook(&self) -> json::Request<DeleteWebhook> {
        json::Request::new(self, DeleteWebhook::new())
    }

    /// For tg-method documentation see [`GetWebhookInfo`]
    ///
    /// [`GetWebhookInfo`]: crate::requests::payloads::GetWebhookInfo
    pub fn get_webhook_info(&self) -> json::Request<GetWebhookInfo> {
        json::Request::new(self, GetWebhookInfo::new())
    }

    /// For tg-method documentation see [`GetMe`]
    ///
    /// [`GetMe`]: crate::requests::payloads::GetMe
    pub fn get_me(&self) -> json::Request<GetMe> {
        json::Request::new(self, GetMe::new())
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

    /// For tg-method documentation see [`ForwardMessage`]
    ///
    /// [`ForwardMessage`]: crate::requests::payloads::ForwardMessage
    pub fn forward_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: i32,
    ) -> json::Request<ForwardMessage>
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        json::Request::new(
            self,
            ForwardMessage::new(chat_id, from_chat_id, message_id),
        )
    }

    /// For tg-method documentation see [`SendPhoto`]
    ///
    /// [`SendPhoto`]: crate::requests::payloads::SendPhoto
    pub fn send_photo<C, P>(
        &self,
        chat_id: C,
        photo: P,
    ) -> multipart::Request<SendPhoto>
    where
        C: Into<ChatId>,
        P: Into<InputFile>,
    {
        multipart::Request::new(self, SendPhoto::new(chat_id, photo))
    }

    /// For tg-method documentation see [`SendAudio`]
    ///
    /// [`SendAudio`]: crate::requests::payloads::SendAudio
    pub fn send_audio<C, A>(
        &self,
        chat_id: C,
        audio: A,
    ) -> multipart::Request<SendAudio>
    where
        C: Into<ChatId>,
        A: Into<InputFile>,
    {
        multipart::Request::new(self, SendAudio::new(chat_id, audio))
    }

    /// For tg-method documentation see [`SendDocument`]
    ///
    /// [`SendDocument`]: crate::requests::payloads::SendDocument
    pub fn send_document<C, D>(
        &self,
        chat_id: C,
        document: D,
    ) -> multipart::Request<SendDocument>
    where
        C: Into<ChatId>,
        D: Into<InputFile>,
    {
        multipart::Request::new(self, SendDocument::new(chat_id, document))
    }

    /// For tg-method documentation see [`SendVideo`]
    ///
    /// [`SendVideo`]: crate::requests::payloads::SendVideo
    pub fn send_video<C, V>(
        &self,
        chat_id: C,
        video: V,
    ) -> multipart::Request<SendVideo>
    where
        C: Into<ChatId>,
        V: Into<InputFile>,
    {
        multipart::Request::new(self, SendVideo::new(chat_id, video))
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

    /// For tg-method documentation see [`SendVoice`]
    ///
    /// [`SendVoice`]: crate::requests::payloads::SendVoice
    pub fn send_voice<C, V>(
        &self,
        chat_id: C,
        voice: V,
    ) -> multipart::Request<SendVoice>
    where
        C: Into<ChatId>,
        V: Into<InputFile>,
    {
        multipart::Request::new(self, SendVoice::new(chat_id, voice))
    }

    /// For tg-method documentation see [`SendVideoNote`]
    ///
    /// [`SendVideoNote`]: crate::requests::payloads::SendVideoNote
    pub fn send_video_note<C, V>(
        &self,
        chat_id: C,
        video_note: V,
    ) -> multipart::Request<SendVideoNote>
    where
        C: Into<ChatId>,
        V: Into<InputFile>,
    {
        multipart::Request::new(self, SendVideoNote::new(chat_id, video_note))
    }

    /// For tg-method documentation see [`SendMediaGroup`]
    ///
    /// [`SendMediaGroup`]: crate::requests::payloads::SendMediaGroup
    pub fn send_media_group<C, M>(
        &self,
        chat_id: C,
        media: M,
    ) -> multipart::Request<SendMediaGroup>
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        multipart::Request::new(self, SendMediaGroup::new(chat_id, media))
    }

    /// For tg-method documentation see [`SendLocation`]
    ///
    /// [`SendLocation`]: crate::requests::payloads::SendLocation
    pub fn send_location<C>(
        &self,
        chat_id: C,
        latitude: f32,
        longitude: f32,
    ) -> json::Request<SendLocation>
    where
        C: Into<ChatId>,
    {
        json::Request::new(
            self,
            SendLocation::new(chat_id, latitude, longitude),
        )
    }

    /// For tg-method documentation see [`EditMessageLiveLocationInline`]
    ///
    /// [`EditMessageLiveLocationInline`]:
    /// crate::requests::payloads::EditMessageLiveLocationInline
    pub fn edit_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
        latitude: f32,
        longitude: f32,
    ) -> json::Request<EditMessageLiveLocationInline>
    where
        I: Into<String>,
    {
        json::Request::new(
            self,
            EditMessageLiveLocationInline::new(
                inline_message_id,
                latitude,
                longitude,
            ),
        )
    }

    /// For tg-method documentation see [`EditMessageLiveLocation`]
    ///
    /// [`EditMessageLiveLocation`]:
    /// crate::requests::payloads::EditMessageLiveLocation
    pub fn edit_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
        latitude: f32,
        longitude: f32,
    ) -> json::Request<EditMessageLiveLocation>
    where
        C: Into<ChatId>,
    {
        json::Request::new(
            self,
            EditMessageLiveLocation::new(
                chat_id, message_id, latitude, longitude,
            ),
        )
    }

    /// For tg-method documentation see [`StopMessageLiveLocationInline`]
    ///
    /// [`StopMessageLiveLocationInline`]:
    /// crate::requests::payloads::StopMessageLiveLocationInline
    pub fn stop_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
    ) -> json::Request<StopMessageLiveLocationInline>
    where
        I: Into<String>,
    {
        json::Request::new(
            self,
            StopMessageLiveLocationInline::new(inline_message_id),
        )
    }

    /// For tg-method documentation see [`StopMessageLiveLocation`]
    ///
    /// [`StopMessageLiveLocation`]:
    /// crate::requests::payloads::StopMessageLiveLocation
    pub fn stop_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> json::Request<StopMessageLiveLocation>
    where
        C: Into<ChatId>,
    {
        json::Request::new(
            self,
            StopMessageLiveLocation::new(chat_id, message_id),
        )
    }

    /// For tg-method documentation see [`SendVenue`]
    ///
    /// [`SendVenue`]: crate::requests::payloads::SendVenue
    pub fn send_venue<C, T, A>(
        &self,
        chat_id: C,
        latitude: f32,
        longitude: f32,
        title: T,
        address: A,
    ) -> json::Request<SendVenue>
    where
        C: Into<ChatId>,
        T: Into<String>,
        A: Into<String>,
    {
        json::Request::new(
            self,
            SendVenue::new(chat_id, latitude, longitude, title, address),
        )
    }

    /// For tg-method documentation see [`SendContact`]
    ///
    /// [`SendContact`]: crate::requests::payloads::SendContact
    pub fn send_contact<C, P, F>(
        &self,
        chat_id: C,
        phone_number: P,
        first_name: F,
    ) -> json::Request<SendContact>
    where
        C: Into<ChatId>,
        P: Into<String>,
        F: Into<String>,
    {
        json::Request::new(
            self,
            SendContact::new(chat_id, phone_number, first_name),
        )
    }

    /// For tg-method documentation see [`SendPoll`]
    ///
    /// [`SendPoll`]: crate::requests::payloads::SendPoll
    pub fn send_poll<C, Q, O>(
        &self,
        chat_id: C,
        question: Q,
        options: O,
    ) -> json::Request<SendPoll>
    where
        C: Into<ChatId>,
        Q: Into<String>,
        O: Into<Vec<String>>,
    {
        json::Request::new(self, SendPoll::new(chat_id, question, options))
    }

    /// For tg-method documentation see [`SendChatAction`]
    ///
    /// [`SendChatAction`]: crate::requests::payloads::SendChatAction
    pub fn send_chat_action<C, A>(
        &self,
        chat_id: C,
        action: A,
    ) -> json::Request<SendChatAction>
    where
        C: Into<ChatId>,
        A: Into<String>,
    {
        json::Request::new(self, SendChatAction::new(chat_id, action))
    }

    /// For tg-method documentation see [`GetUserProfilePhoto`]
    ///
    /// [`GetUserProfilePhoto`]: crate::requests::payloads::GetUserProfilePhoto
    pub fn get_user_profile_photos(
        &self,
        user_id: i32,
    ) -> json::Request<GetUserProfilePhoto> {
        json::Request::new(self, GetUserProfilePhoto::new(user_id))
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

    /// For tg-method documentation see [`KickChatMember`]
    ///
    /// [`KickChatMember`]: crate::requests::payloads::KickChatMember
    pub fn kick_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
    ) -> json::Request<KickChatMember>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, KickChatMember::new(chat_id, user_id))
    }

    /// For tg-method documentation see [`UnbanChatMember`]
    ///
    /// [`UnbanChatMember`]: crate::requests::payloads::UnbanChatMember
    pub fn unban_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
    ) -> json::Request<UnbanChatMember>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, UnbanChatMember::new(chat_id, user_id))
    }

    /// For tg-method documentation see [`RestrictChatMember`]
    ///
    /// [`RestrictChatMember`]: crate::requests::payloads::RestrictChatMember
    pub fn restrict_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
        permissions: ChatPermissions,
    ) -> json::Request<RestrictChatMember>
    where
        C: Into<ChatId>,
    {
        json::Request::new(
            self,
            RestrictChatMember::new(chat_id, user_id, permissions),
        )
    }

    /// For tg-method documentation see [`PromoteChatMember`]
    ///
    /// [`PromoteChatMember`]: crate::requests::payloads::PromoteChatMember
    pub fn promote_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
    ) -> json::Request<PromoteChatMember>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, PromoteChatMember::new(chat_id, user_id))
    }

    /// For tg-method documentation see [`SetChatPermission`]
    ///
    /// [`SetChatPermission`]: crate::requests::payloads::SetChatPermission
    pub fn set_chat_permissions<C>(
        &self,
        chat_id: C,
        permissions: ChatPermissions,
    ) -> json::Request<SetChatPermission>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, SetChatPermission::new(chat_id, permissions))
    }

    /// For tg-method documentation see [`ExportChatInviteLink`]
    ///
    /// [`ExportChatInviteLink`]:
    /// crate::requests::payloads::ExportChatInviteLink
    pub fn export_chat_invite_link<C>(
        &self,
        chat_id: C,
    ) -> json::Request<ExportChatInviteLink>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, ExportChatInviteLink::new(chat_id))
    }

    /// For tg-method documentation see [`SetChatPhoto`]
    ///
    /// [`SetChatPhoto`]: crate::requests::payloads::SetChatPhoto
    pub fn set_chat_photo<C>(
        &self,
        chat_id: C,
        photo: InputFile,
    ) -> json::Request<SetChatPhoto>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, SetChatPhoto::new(chat_id, photo))
    }

    /// For tg-method documentation see [`DeleteChatPhoto`]
    ///
    /// [`DeleteChatPhoto`]: crate::requests::payloads::DeleteChatPhoto
    pub fn delete_chat_photo<C>(
        &self,
        chat_id: C,
    ) -> json::Request<DeleteChatPhoto>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, DeleteChatPhoto::new(chat_id))
    }

    /// For tg-method documentation see [`SetChatTitle`]
    ///
    /// [`SetChatTitle`]: crate::requests::payloads::SetChatTitle
    pub fn set_chat_title<C, T>(
        &self,
        chat_id: C,
        title: T,
    ) -> json::Request<SetChatTitle>
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        json::Request::new(self, SetChatTitle::new(chat_id, title))
    }

    /// For tg-method documentation see [`SetChatDescription`]
    ///
    /// [`SetChatDescription`]: crate::requests::payloads::SetChatDescription
    pub fn set_chat_description<C>(
        &self,
        chat_id: C,
    ) -> json::Request<SetChatDescription>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, SetChatDescription::new(chat_id))
    }

    /// For tg-method documentation see [`PinChatMessage`]
    ///
    /// [`PinChatMessage`]: crate::requests::payloads::PinChatMessage
    pub fn pin_chat_message<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> json::Request<PinChatMessage>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, PinChatMessage::new(chat_id, message_id))
    }

    /// For tg-method documentation see [`UnpinChatMessage`]
    ///
    /// [`UnpinChatMessage`]: crate::requests::payloads::UnpinChatMessage
    pub fn unpin_chat_message<C>(
        &self,
        chat_id: C,
    ) -> json::Request<UnpinChatMessage>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, UnpinChatMessage::new(chat_id))
    }

    /// For tg-method documentation see [`LeaveChat`]
    ///
    /// [`LeaveChat`]: crate::requests::payloads::LeaveChat
    pub fn leave_chat<C>(&self, chat_id: C) -> json::Request<LeaveChat>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, LeaveChat::new(chat_id))
    }

    /// For tg-method documentation see [`GetChat`]
    ///
    /// [`GetChat`]: crate::requests::payloads::GetChat
    pub fn get_chat<C>(&self, chat_id: C) -> json::Request<GetChat>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, GetChat::new(chat_id))
    }

    /// For tg-method documentation see [`GetChatAdministrator`]
    ///
    /// [`GetChatAdministrator`]:
    /// crate::requests::payloads::GetChatAdministrator
    pub fn get_chat_administrators<C>(
        &self,
        chat_id: C,
    ) -> json::Request<GetChatAdministrator>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, GetChatAdministrator::new(chat_id))
    }

    /// For tg-method documentation see [`GetChatMembersCount`]
    ///
    /// [`GetChatMembersCount`]: crate::requests::payloads::GetChatMembersCount
    pub fn get_chat_members_count<C>(
        &self,
        chat_id: C,
    ) -> json::Request<GetChatMembersCount>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, GetChatMembersCount::new(chat_id))
    }

    /// For tg-method documentation see [`GetChatMember`]
    ///
    /// [`GetChatMember`]: crate::requests::payloads::GetChatMember
    pub fn get_chat_member<C>(
        &self,
        chat_id: C,
        user_id: i32,
    ) -> json::Request<GetChatMember>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, GetChatMember::new(chat_id, user_id))
    }

    /// For tg-method documentation see [`SetChatStickerSet`]
    ///
    /// [`SetChatStickerSet`]: crate::requests::payloads::SetChatStickerSet
    pub fn set_chat_sticker_set<C, S>(
        &self,
        chat_id: C,
        sticker_set_name: S,
    ) -> json::Request<SetChatStickerSet>
    where
        C: Into<ChatId>,
        S: Into<String>,
    {
        json::Request::new(
            self,
            SetChatStickerSet::new(chat_id, sticker_set_name),
        )
    }

    /// For tg-method documentation see [`DeleteChatStickerSet`]
    ///
    /// [`DeleteChatStickerSet`]:
    /// crate::requests::payloads::DeleteChatStickerSet
    pub fn delete_chat_sticker_set<C>(
        &self,
        chat_id: C,
    ) -> json::Request<DeleteChatStickerSet>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, DeleteChatStickerSet::new(chat_id))
    }

    /// For tg-method documentation see [`AnswerCallbackQuery`]
    ///
    /// [`AnswerCallbackQuery`]: crate::requests::payloads::AnswerCallbackQuery
    pub fn answer_callback_query<C>(
        &self,
        callback_query_id: C,
    ) -> json::Request<AnswerCallbackQuery>
    where
        C: Into<String>,
    {
        json::Request::new(self, AnswerCallbackQuery::new(callback_query_id))
    }

    /// For tg-method documentation see [`EditMessageTextInline`]
    ///
    /// [`EditMessageTextInline`]:
    /// crate::requests::payloads::EditMessageTextInline
    pub fn edit_message_text_inline<I, T>(
        &self,
        inline_message_id: I,
        text: T,
    ) -> json::Request<EditMessageTextInline>
    where
        I: Into<String>,
        T: Into<String>,
    {
        json::Request::new(
            self,
            EditMessageTextInline::new(inline_message_id, text),
        )
    }

    /// For tg-method documentation see [`EditMessageText`]
    ///
    /// [`EditMessageText`]: crate::requests::payloads::EditMessageText
    pub fn edit_message_text<C, T>(
        &self,
        chat_id: C,
        message_id: i32,
        text: T,
    ) -> json::Request<EditMessageText>
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        json::Request::new(
            self,
            EditMessageText::new(chat_id, message_id, text),
        )
    }

    /// For tg-method documentation see [`EditMessageCaptionInline`]
    ///
    /// [`EditMessageCaptionInline`]:
    /// crate::requests::payloads::EditMessageCaptionInline
    pub fn edit_message_caption_inline<I>(
        &self,
        inline_message_id: I,
    ) -> json::Request<EditMessageCaptionInline>
    where
        I: Into<String>,
    {
        json::Request::new(
            self,
            EditMessageCaptionInline::new(inline_message_id),
        )
    }

    /// For tg-method documentation see [`EditMessageCaption`]
    ///
    /// [`EditMessageCaption`]: crate::requests::payloads::EditMessageCaption
    pub fn edit_message_caption<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> json::Request<EditMessageCaption>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, EditMessageCaption::new(chat_id, message_id))
    }

    /// For tg-method documentation see [`EditMessageMediaInline`]
    ///
    /// [`EditMessageMediaInline`]:
    /// crate::requests::payloads::EditMessageMediaInline
    pub fn edit_message_media_inline<I>(
        &self,
        inline_message_id: I,
        media: InputMedia,
    ) -> multipart::Request<EditMessageMediaInline>
    where
        I: Into<String>,
    {
        multipart::Request::new(
            self,
            EditMessageMediaInline::new(inline_message_id, media),
        )
    }

    /// For tg-method documentation see [`EditMessageMedum`]
    ///
    /// [`EditMessageMedum`]: crate::requests::payloads::EditMessageMedum
    pub fn edit_message_media<C>(
        &self,
        chat_id: C,
        message_id: i32,
        media: InputMedia,
    ) -> multipart::Request<EditMessageMedia>
    where
        C: Into<ChatId>,
    {
        multipart::Request::new(
            self,
            EditMessageMedia::new(chat_id, message_id, media),
        )
    }

    /// For tg-method documentation see [`EditMessageReplyMarkupInline`]
    ///
    /// [`EditMessageReplyMarkupInline`]:
    /// crate::requests::payloads::EditMessageReplyMarkupInline
    pub fn edit_message_reply_markup_inline<I>(
        &self,
        inline_message_id: I,
    ) -> json::Request<EditMessageReplyMarkupInline>
    where
        I: Into<String>,
    {
        json::Request::new(
            self,
            EditMessageReplyMarkupInline::new(inline_message_id),
        )
    }

    /// For tg-method documentation see [`EditMessageReplyMarkup`]
    ///
    /// [`EditMessageReplyMarkup`]:
    /// crate::requests::payloads::EditMessageReplyMarkup
    pub fn edit_message_reply_markup<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> json::Request<EditMessageReplyMarkup>
    where
        C: Into<ChatId>,
    {
        json::Request::new(
            self,
            EditMessageReplyMarkup::new(chat_id, message_id),
        )
    }

    /// For tg-method documentation see [`StopPoll`]
    ///
    /// [`StopPoll`]: crate::requests::payloads::StopPoll
    pub fn stop_poll<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> json::Request<StopPoll>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, StopPoll::new(chat_id, message_id))
    }

    /// For tg-method documentation see [`DeleteMessage`]
    ///
    /// [`DeleteMessage`]: crate::requests::payloads::DeleteMessage
    pub fn delete_message<C>(
        &self,
        chat_id: C,
        message_id: i32,
    ) -> json::Request<DeleteMessage>
    where
        C: Into<ChatId>,
    {
        json::Request::new(self, DeleteMessage::new(chat_id, message_id))
    }

    /// For tg-method documentation see [`SendSticker`]
    ///
    /// [`SendSticker`]: crate::requests::payloads::SendSticker
    pub fn send_sticker<C, S>(
        &self,
        chat_id: C,
        sticker: S,
    ) -> multipart::Request<SendSticker>
    where
        C: Into<ChatId>,
        S: Into<InputFile>,
    {
        multipart::Request::new(self, SendSticker::new(chat_id, sticker))
    }

    /// For tg-method documentation see [`GetStickerSet`]
    ///
    /// [`GetStickerSet`]: crate::requests::payloads::GetStickerSet
    pub fn get_sticker_set<N>(&self, name: N) -> json::Request<GetStickerSet>
    where
        N: Into<String>,
    {
        json::Request::new(self, GetStickerSet::new(name))
    }

    /// For tg-method documentation see [`UploadStickerFile`]
    ///
    /// [`UploadStickerFile`]: crate::requests::payloads::UploadStickerFile
    pub fn upload_sticker_file(
        &self,
        user_id: i32,
        png_sticker: InputFile,
    ) -> json::Request<UploadStickerFile> {
        json::Request::new(self, UploadStickerFile::new(user_id, png_sticker))
    }

    /// For tg-method documentation see [`CreateNewStickerSet`]
    ///
    /// [`CreateNewStickerSet`]: crate::requests::payloads::CreateNewStickerSet
    pub fn create_new_sticker_set<N, T, P, E>(
        &self,
        user_id: i32,
        name: N,
        title: T,
        png_sticker: P,
        emojis: E,
    ) -> multipart::Request<CreateNewStickerSet>
    where
        N: Into<String>,
        T: Into<String>,
        P: Into<InputFile>,
        E: Into<String>,
    {
        multipart::Request::new(
            self,
            CreateNewStickerSet::new(user_id, name, title, png_sticker, emojis),
        )
    }

    /// For tg-method documentation see [`AddStickerToSet`]
    ///
    /// [`AddStickerToSet`]: crate::requests::payloads::AddStickerToSet
    pub fn add_sticker_to_set<N, P, E>(
        &self,
        user_id: i32,
        name: N,
        png_sticker: P,
        emojis: E,
    ) -> multipart::Request<AddStickerToSet>
    where
        N: Into<String>,
        P: Into<InputFile>,
        E: Into<String>,
    {
        multipart::Request::new(
            self,
            AddStickerToSet::new(user_id, name, png_sticker, emojis),
        )
    }

    /// For tg-method documentation see [`SetStickerPositionInSet`]
    ///
    /// [`SetStickerPositionInSet`]:
    /// crate::requests::payloads::SetStickerPositionInSet
    pub fn set_sticker_position_in_set<S>(
        &self,
        sticker: S,
        position: i32,
    ) -> json::Request<SetStickerPositionInSet>
    where
        S: Into<String>,
    {
        json::Request::new(
            self,
            SetStickerPositionInSet::new(sticker, position),
        )
    }

    /// For tg-method documentation see [`DeleteStickerFromSet`]
    ///
    /// [`DeleteStickerFromSet`]:
    /// crate::requests::payloads::DeleteStickerFromSet
    pub fn delete_sticker_from_set<S>(
        &self,
        sticker: S,
    ) -> json::Request<DeleteStickerFromSet>
    where
        S: Into<String>,
    {
        json::Request::new(self, DeleteStickerFromSet::new(sticker))
    }

    /// For tg-method documentation see [`AnswerInlineQuery`]
    ///
    /// [`AnswerInlineQuery`]: crate::requests::payloads::AnswerInlineQuery
    pub fn answer_inline_query<I, R>(
        &self,
        inline_query_id: I,
        results: R,
    ) -> json::Request<AnswerInlineQuery>
    where
        I: Into<String>,
        R: Into<Vec<InlineQueryResult>>,
    {
        json::Request::new(
            self,
            AnswerInlineQuery::new(inline_query_id, results),
        )
    }

    /// For tg-method documentation see [`SendInvoice`]
    ///
    /// [`SendInvoice`]: crate::requests::payloads::SendInvoice
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
    ) -> json::Request<SendInvoice>
    where
        T: Into<String>,
        D: Into<String>,
        Pl: Into<String>,
        Pt: Into<String>,
        S: Into<String>,
        C: Into<String>,
        Pr: Into<Vec<LabeledPrice>>,
    {
        json::Request::new(
            self,
            SendInvoice::new(
                chat_id,
                title,
                description,
                payload,
                provider_token,
                start_parameter,
                currency,
                prices,
            ),
        )
    }

    /// For tg-method documentation see [`AnswerShippingQuery`]
    ///
    /// [`AnswerShippingQuery`]: crate::requests::payloads::AnswerShippingQuery
    pub fn answer_shipping_query<S>(
        &self,
        shipping_query_id: S,
        ok: bool,
    ) -> json::Request<AnswerShippingQuery>
    where
        S: Into<String>,
    {
        json::Request::new(
            self,
            AnswerShippingQuery::new(shipping_query_id, ok),
        )
    }

    /// For tg-method documentation see [`AnswerPreCheckoutQuery`]
    ///
    /// [`AnswerPreCheckoutQuery`]:
    /// crate::requests::payloads::AnswerPreCheckoutQuery
    pub fn answer_pre_checkout_query<P>(
        &self,
        pre_checkout_query_id: P,
        ok: bool,
    ) -> json::Request<AnswerPreCheckoutQuery>
    where
        P: Into<String>,
    {
        json::Request::new(
            self,
            AnswerPreCheckoutQuery::new(pre_checkout_query_id, ok),
        )
    }

    /// For tg-method documentation see [`SendGame`]
    ///
    /// [`SendGame`]: crate::requests::payloads::SendGame
    pub fn send_game<G>(
        &self,
        chat_id: i32,
        game_short_name: G,
    ) -> json::Request<SendGame>
    where
        G: Into<String>,
    {
        json::Request::new(self, SendGame::new(chat_id, game_short_name))
    }

    /// For tg-method documentation see [`SetGameScoreInline`]
    ///
    /// [`SetGameScoreInline`]: crate::requests::payloads::SetGameScoreInline
    pub fn set_game_score_inline<I>(
        &self,
        inline_message_id: I,
        user_id: i32,
        score: i32,
    ) -> json::Request<SetGameScoreInline>
    where
        I: Into<String>,
    {
        json::Request::new(
            self,
            SetGameScoreInline::new(inline_message_id, user_id, score),
        )
    }

    /// For tg-method documentation see [`SetGameScore`]
    ///
    /// [`SetGameScore`]: crate::requests::payloads::SetGameScore
    pub fn set_game_score<C>(
        &self,
        chat_id: C,
        message_id: i32,
        user_id: i32,
        score: i32,
    ) -> json::Request<SetGameScore>
    where
        C: Into<ChatId>,
    {
        json::Request::new(
            self,
            SetGameScore::new(chat_id, message_id, user_id, score),
        )
    }

    /// For tg-method documentation see [`GetGameHighScoreInline`]
    ///
    /// [`GetGameHighScoreInline`]:
    /// crate::requests::payloads::GetGameHighScoreInline
    pub fn get_game_high_scores_inline<I>(
        &self,
        inline_message_id: I,
        user_id: i32,
    ) -> json::Request<GetGameHighScoreInline>
    where
        I: Into<String>,
    {
        json::Request::new(
            self,
            GetGameHighScoreInline::new(inline_message_id, user_id),
        )
    }

    /// For tg-method documentation see [`GetGameHighScore`]
    ///
    /// [`GetGameHighScore`]: crate::requests::payloads::GetGameHighScore
    pub fn get_game_high_scores<C>(
        &self,
        chat_id: C,
        message_id: i32,
        user_id: i32,
    ) -> json::Request<GetGameHighScore>
    where
        C: Into<ChatId>,
    {
        json::Request::new(
            self,
            GetGameHighScore::new(chat_id, message_id, user_id),
        )
    }

    /// For tg-method documentation see [`SetChatAdministratorCustomTitle`]
    ///
    /// [`SetChatAdministratorCustomTitle`]:
    /// crate::requests::payloads::SetChatAdministratorCustomTitle
    pub fn set_chat_administrator_custom_title<C, CT>(
        &self,
        chat_id: C,
        user_id: i32,
        custom_title: CT,
    ) -> json::Request<SetChatAdministratorCustomTitle>
    where
        C: Into<ChatId>,
        CT: Into<String>,
    {
        json::Request::new(
            self,
            SetChatAdministratorCustomTitle::new(
                chat_id,
                user_id,
                custom_title,
            ),
        )
    }
}
