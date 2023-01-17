use url::Url;

use crate::{
    prelude::Requester,
    requests::HasPayload,
    types::{InputFile, ParseMode, Recipient, *},
};

/// Default parse mode adaptor, see
/// [`RequesterExt::parse_mode`](crate::requests::RequesterExt::parse_mode).
#[derive(Clone, Debug)]
pub struct DefaultParseMode<B> {
    bot: B,
    mode: ParseMode,
}

impl<B> DefaultParseMode<B> {
    /// Creates new [`DefaultParseMode`].
    ///
    /// Note: it's recommended to use [`RequesterExt::parse_mode`] instead.
    ///
    /// [`RequesterExt::parse_mode`]: crate::requests::RequesterExt::parse_mode
    pub fn new(bot: B, parse_mode: ParseMode) -> Self {
        Self { bot, mode: parse_mode }
    }

    /// Allows to access the inner bot.
    pub fn inner(&self) -> &B {
        &self.bot
    }

    /// Unwraps the inner bot.
    pub fn into_inner(self) -> B {
        self.bot
    }

    /// Returns currently used [`ParseMode`].
    pub fn parse_mode(&self) -> ParseMode {
        self.mode
    }
}

macro_rules! f {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        {
            let mut req = $this.inner().$m($($arg),*);
            req.payload_mut().parse_mode = Some($this.mode);
            req
        }
    };
}

macro_rules! fty {
    ($T:ident) => {
        B::$T
    };
}

macro_rules! fid {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        $this.inner().$m($($arg),*)
    };
}

impl<B: Requester> Requester for DefaultParseMode<B> {
    type Err = B::Err;

    requester_forward! {
        send_message,
        send_photo,
        send_video,
        send_audio,
        send_document,
        send_animation,
        send_voice,
        edit_message_text,
        edit_message_text_inline,
        edit_message_caption,
        edit_message_caption_inline => f, fty
    }

    type SendPoll = B::SendPoll;

    fn send_poll<C, Q, O>(&self, chat_id: C, question: Q, options: O) -> Self::SendPoll
    where
        C: Into<Recipient>,
        Q: Into<String>,
        O: IntoIterator<Item = String>,
    {
        let mut req = self.inner().send_poll(chat_id, question, options);
        req.payload_mut().explanation_parse_mode = Some(self.mode);
        req
    }

    requester_forward! {
        get_me,
        log_out,
        close,
        get_updates,
        set_webhook,
        delete_webhook,
        get_webhook_info,
        forward_message,
        copy_message,
        send_video_note,
        send_media_group,
        send_location,
        edit_message_live_location,
        edit_message_live_location_inline,
        stop_message_live_location,
        stop_message_live_location_inline,
        send_venue,
        send_contact,
        send_dice,
        send_chat_action,
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
        edit_general_forum_topic,
        close_general_forum_topic,
        reopen_general_forum_topic,
        hide_general_forum_topic,
        unhide_general_forum_topic,
        unpin_all_forum_topic_messages,
        answer_callback_query,
        set_my_commands,
        get_my_commands,
        set_chat_menu_button,
        get_chat_menu_button,
        set_my_default_administrator_rights,
        get_my_default_administrator_rights,
        delete_my_commands,
        answer_inline_query,
        answer_web_app_query,
        edit_message_media,
        edit_message_media_inline,
        edit_message_reply_markup,
        edit_message_reply_markup_inline,
        stop_poll,
        delete_message,
        send_sticker,
        get_sticker_set,
        get_custom_emoji_stickers,
        upload_sticker_file,
        create_new_sticker_set,
        add_sticker_to_set,
        set_sticker_position_in_set,
        delete_sticker_from_set,
        set_sticker_set_thumb,
        send_invoice,
        create_invoice_link,
        answer_shipping_query,
        answer_pre_checkout_query,
        set_passport_data_errors,
        send_game,
        set_game_score,
        set_game_score_inline,
        get_game_high_scores,
        approve_chat_join_request,
        decline_chat_join_request
        => fid, fty
    }
}

download_forward! {
    'w
    B
    DefaultParseMode<B>
    { this => this.inner() }
}
