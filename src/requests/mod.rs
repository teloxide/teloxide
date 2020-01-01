//! API requests.

mod form_builder;
mod utils;

pub mod dynamic;
pub mod json;
pub mod multipart;

/// A type that is returned when making requests to telegram
pub type ResponseResult<T> = Result<T, crate::RequestError>;

/// Signature of telegram method.
pub trait Method {
    /// Return-type of the method.
    type Output;

    /// Name of the method.
    const NAME: &'static str;
}

/// Signature of telegram method.
///
/// Note: this trait is very similar to [`Method`] trait, however it can be used
/// as trait object.
pub trait DynMethod {
    type Output;

    /// Return name of the method.
    fn name(&self) -> &str;
}

impl<T> DynMethod for T
where
    T: Method,
{
    type Output = T::Output;

    fn name(&self) -> &str {
        T::NAME
    }
}

#[rustfmt::skip]
pub mod payloads {
    mod get_updates;
    mod set_webhook;
    mod delete_webhook;
    mod get_webhook_info;
    mod get_me;
    mod send_message;
    mod forward_message;
    mod send_photo;
    mod send_audio;
    mod send_document;
    mod send_video;
    mod send_animation;
    mod send_voice;
    mod send_video_note;
    mod send_media_group;
    mod send_location;
    mod edit_message_live_location_inline;
    mod edit_message_live_location;
    mod stop_message_live_location_inline;
    mod stop_message_live_location;
    mod send_venue;
    mod send_contact;
    mod send_poll;
    mod send_chat_action;
    mod get_user_profile_photos;
    mod get_file;
    mod kick_chat_member;
    mod unban_chat_member;
    mod restrict_chat_member;
    mod promote_chat_member;
    mod set_chat_permissions;
    mod export_chat_invite_link;
    mod set_chat_photo;
    mod delete_chat_photo;
    mod set_chat_title;
    mod set_chat_description;
    mod pin_chat_message;
    mod unpin_chat_message;
    mod leave_chat;
    mod get_chat;
    mod get_chat_administrators;
    mod get_chat_members_count;
    mod get_chat_member;
    mod set_chat_sticker_set;
    mod delete_chat_sticker_set;
    mod answer_callback_query;
    mod edit_message_text_inline;
    mod edit_message_text;
    mod edit_message_caption_inline;
    mod edit_message_caption;
    mod edit_message_media_inline;
    mod edit_message_media;
    mod edit_message_reply_markup_inline;
    mod edit_message_reply_markup;
    mod stop_poll;
    mod delete_message;
    mod send_sticker;
    mod get_sticker_set;
    mod upload_sticker_file;
    mod create_new_sticker_set;
    mod add_sticker_to_set;
    mod set_sticker_position_in_set;
    mod delete_sticker_from_set;
    mod answer_inline_query;
    mod send_invoice;
    mod answer_shipping_query;
    mod answer_pre_checkout_query;
    mod send_game;
    mod set_game_score_inline;
    mod set_game_score;
    mod get_game_high_scores;
    mod get_game_high_scores_inline;
    mod set_chat_administrator_custom_title;

    pub use {
        get_updates::GetUpdates,
        set_webhook::SetWebhook,
        delete_webhook::DeleteWebhook,
        get_webhook_info::GetWebhookInfo,
        get_me::GetMe,
        send_message::SendMessage,
        forward_message::ForwardMessage,
        send_photo::SendPhoto,
        send_audio::SendAudio,
        send_document::SendDocument,
        send_video::SendVideo,
        send_animation::SendAnimation,
        send_voice::SendVoice,
        send_video_note::SendVideoNote,
        send_media_group::SendMediaGroup,
        send_location::SendLocation,
        edit_message_live_location_inline::EditMessageLiveLocationInline,
        edit_message_live_location::EditMessageLiveLocation,
        stop_message_live_location_inline::StopMessageLiveLocationInline,
        stop_message_live_location::StopMessageLiveLocation,
        send_venue::SendVenue,
        send_contact::SendContact,
        send_poll::SendPoll,
        send_chat_action::SendChatAction,
        get_user_profile_photos::GetUserProfilePhoto,
        get_file::GetFile,
        kick_chat_member::KickChatMember,
        unban_chat_member::UnbanChatMember,
        restrict_chat_member::RestrictChatMember,
        promote_chat_member::PromoteChatMember,
        set_chat_permissions::SetChatPermission,
        export_chat_invite_link::ExportChatInviteLink,
        set_chat_photo::SetChatPhoto,
        delete_chat_photo::DeleteChatPhoto,
        set_chat_title::SetChatTitle,
        set_chat_description::SetChatDescription,
        pin_chat_message::PinChatMessage,
        unpin_chat_message::UnpinChatMessage,
        leave_chat::LeaveChat,
        get_chat::GetChat,
        get_chat_administrators::GetChatAdministrator,
        get_chat_members_count::GetChatMembersCount,
        get_chat_member::GetChatMember,
        set_chat_sticker_set::SetChatStickerSet,
        delete_chat_sticker_set::DeleteChatStickerSet,
        answer_callback_query::AnswerCallbackQuery,
        edit_message_text_inline::EditMessageTextInline,
        edit_message_text::EditMessageText,
        edit_message_caption_inline::EditMessageCaptionInline,
        edit_message_caption::EditMessageCaption,
        edit_message_media_inline::EditMessageMediaInline,
        edit_message_media::EditMessageMedia,
        edit_message_reply_markup_inline::EditMessageReplyMarkupInline,
        edit_message_reply_markup::EditMessageReplyMarkup,
        stop_poll::StopPoll,
        delete_message::DeleteMessage,
        send_sticker::SendSticker,
        get_sticker_set::GetStickerSet,
        upload_sticker_file::UploadStickerFile,
        create_new_sticker_set::CreateNewStickerSet,
        add_sticker_to_set::AddStickerToSet,
        set_sticker_position_in_set::SetStickerPositionInSet,
        delete_sticker_from_set::DeleteStickerFromSet,
        answer_inline_query::AnswerInlineQuery,
        send_invoice::SendInvoice,
        answer_shipping_query::AnswerShippingQuery,
        answer_pre_checkout_query::AnswerPreCheckoutQuery,
        send_game::SendGame,
        set_game_score_inline::SetGameScoreInline,
        set_game_score::SetGameScore,
        get_game_high_scores_inline::GetGameHighScoreInline,
        get_game_high_scores::GetGameHighScore,
        set_chat_administrator_custom_title::SetChatAdministratorCustomTitle,
    };
}
