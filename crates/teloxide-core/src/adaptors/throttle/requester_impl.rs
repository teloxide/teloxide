use std::sync::Arc;

use url::Url;

use crate::{
    adaptors::{throttle::ThrottlingRequest, Throttle},
    errors::AsResponseParameters,
    requests::{HasPayload, Requester},
    types::*,
};

macro_rules! f {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        ThrottlingRequest {
            request: Arc::new($this.inner().$m($($arg),*)),
            chat_id: |p| (&p.payload_ref().chat_id).into(),
            worker: $this.queue.clone(),
        }
    };
}

macro_rules! fty {
    ($T:ident) => {
        ThrottlingRequest<B::$T>
    };
}

macro_rules! fid {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        $this.inner().$m($($arg),*)
    };
}

macro_rules! ftyid {
    ($T:ident) => {
        B::$T
    };
}

impl<B: Requester> Requester for Throttle<B>
where
    B::Err: AsResponseParameters,

    B::SendMessage: Clone + Send + Sync + 'static,
    B::ForwardMessage: Clone + Send + Sync + 'static,
    B::ForwardMessages: Clone + Send + Sync + 'static,
    B::CopyMessage: Clone + Send + Sync + 'static,
    B::CopyMessages: Clone + Send + Sync + 'static,
    B::SendPhoto: Clone + Send + Sync + 'static,
    B::SendAudio: Clone + Send + Sync + 'static,
    B::SendDocument: Clone + Send + Sync + 'static,
    B::SendVideo: Clone + Send + Sync + 'static,
    B::SendAnimation: Clone + Send + Sync + 'static,
    B::SendVoice: Clone + Send + Sync + 'static,
    B::SendVideoNote: Clone + Send + Sync + 'static,
    B::SendMediaGroup: Clone + Send + Sync + 'static,
    B::SendLocation: Clone + Send + Sync + 'static,
    B::SendVenue: Clone + Send + Sync + 'static,
    B::SendContact: Clone + Send + Sync + 'static,
    B::SendPoll: Clone + Send + Sync + 'static,
    B::SendDice: Clone + Send + Sync + 'static,
    B::SendSticker: Clone + Send + Sync + 'static,
    B::SendInvoice: Clone + Send + Sync + 'static,
    B::SendGame: Clone + Send + Sync + 'static,
{
    type Err = B::Err;

    requester_forward! {
        send_message,
        forward_message,
        forward_messages,
        copy_message,
        copy_messages,
        send_photo,
        send_audio,
        send_document,
        send_video,
        send_animation,
        send_voice,
        send_video_note,
        send_media_group,
        send_location,
        send_venue,
        send_contact,
        send_poll,
        send_dice,
        send_sticker,
        send_invoice,
        send_game
        => f, fty
    }

    requester_forward! {
        get_me,
        log_out,
        close,
        get_updates,
        set_webhook,
        delete_webhook,
        get_webhook_info,
        edit_message_live_location,
        edit_message_live_location_inline,
        stop_message_live_location,
        stop_message_live_location_inline,
        send_chat_action,
        set_message_reaction,
        get_user_profile_photos,
        get_file,
        kick_chat_member,
        ban_chat_member,
        unban_chat_member,
        restrict_chat_member,
        promote_chat_member,
        set_chat_administrator_custom_title,
        ban_chat_sender_chat,
        unban_chat_sender_chat,
        set_chat_permissions,
        export_chat_invite_link,
        create_chat_invite_link,
        edit_chat_invite_link,
        revoke_chat_invite_link,
        set_chat_photo,
        delete_chat_photo,
        set_chat_title,
        set_chat_description,
        pin_chat_message,
        unpin_chat_message,
        unpin_all_chat_messages,
        leave_chat,
        get_chat,
        get_chat_administrators,
        get_chat_members_count,
        get_chat_member_count,
        get_chat_member,
        set_chat_sticker_set,
        delete_chat_sticker_set,
        get_forum_topic_icon_stickers,
        create_forum_topic,
        edit_forum_topic,
        close_forum_topic,
        reopen_forum_topic,
        delete_forum_topic,
        unpin_all_forum_topic_messages,
        edit_general_forum_topic,
        close_general_forum_topic,
        reopen_general_forum_topic,
        hide_general_forum_topic,
        unhide_general_forum_topic,
        unpin_all_general_forum_topic_messages,
        answer_callback_query,
        get_user_chat_boosts,
        set_my_commands,
        get_business_connection,
        get_my_commands,
        set_my_name,
        get_my_name,
        set_my_description,
        get_my_description,
        set_my_short_description,
        get_my_short_description,
        set_chat_menu_button,
        get_chat_menu_button,
        set_my_default_administrator_rights,
        get_my_default_administrator_rights,
        delete_my_commands,
        answer_inline_query,
        answer_web_app_query,
        edit_message_text,
        edit_message_text_inline,
        edit_message_caption,
        edit_message_caption_inline,
        edit_message_media,
        edit_message_media_inline,
        edit_message_reply_markup,
        edit_message_reply_markup_inline,
        stop_poll,
        delete_message,
        delete_messages,
        get_sticker_set,
        get_custom_emoji_stickers,
        upload_sticker_file,
        create_new_sticker_set,
        add_sticker_to_set,
        set_sticker_position_in_set,
        delete_sticker_from_set,
        replace_sticker_in_set,
        set_sticker_set_thumbnail,
        set_custom_emoji_sticker_set_thumbnail,
        set_sticker_set_title,
        delete_sticker_set,
        set_sticker_emoji_list,
        set_sticker_keywords,
        set_sticker_mask_position,
        answer_shipping_query,
        create_invoice_link,
        answer_pre_checkout_query,
        get_star_transactions,
        refund_star_payment,
        set_passport_data_errors,
        set_game_score,
        set_game_score_inline,
        approve_chat_join_request,
        decline_chat_join_request,
        get_game_high_scores
        => fid, ftyid
    }
}

download_forward! {
    B
    Throttle<B>
    { this => this.inner() }
}
