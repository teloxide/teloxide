use std::future::IntoFuture;

use url::Url;

use crate::{
    requests::{HasPayload, Output, Request, Requester},
    types::*,
};

/// Previously was used to send requests automatically.
///
/// Before addition of [`IntoFuture`] you could only `.await` [`Future`]s.
/// This adaptor turned requests into futures, allowing to `.await` them,
/// without calling `.send()`.
///
/// Now, however, all requests are required to implement `IntoFuture`, allowing
/// you to `.await` them directly. This adaptor is noop, and shouldn't be used.
///
/// [`Future`]: std::future::Future
#[derive(Clone, Debug)]
pub struct AutoSend<B> {
    bot: B,
}

impl<B> AutoSend<B> {
    /// Creates new `AutoSend`.
    ///
    /// Note: it's recommended to use [`RequesterExt::auto_send`] instead.
    ///
    /// [`RequesterExt::auto_send`]: crate::requests::RequesterExt::auto_send
    pub fn new(inner: B) -> AutoSend<B> {
        Self { bot: inner }
    }

    /// Allows to access the inner bot.
    pub fn inner(&self) -> &B {
        &self.bot
    }

    /// Unwraps the inner bot.
    pub fn into_inner(self) -> B {
        self.bot
    }
}

macro_rules! f {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        AutoRequest::new($this.inner().$m($($arg),*))
    };
}

macro_rules! fty {
    ($T:ident) => {
        AutoRequest<B::$T>
    };
}

impl<B> Requester for AutoSend<B>
where
    B: Requester,
{
    type Err = B::Err;

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
        send_message,
        send_photo,
        send_audio,
        send_document,
        send_video,
        send_animation,
        send_voice,
        send_video_note,
        send_media_group,
        send_location,
        edit_message_live_location,
        edit_message_live_location_inline,
        stop_message_live_location,
        stop_message_live_location_inline,
        send_venue,
        send_contact,
        send_poll,
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
        unpin_all_forum_topic_messages,
        edit_general_forum_topic,
        close_general_forum_topic,
        reopen_general_forum_topic,
        hide_general_forum_topic,
        unhide_general_forum_topic,
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
        => f, fty
    }
}

download_forward! {
    'w
    B
    AutoSend<B>
    { this => this.inner() }
}

#[must_use = "Futures are lazy and do nothing unless polled or awaited"]
pub struct AutoRequest<R>(R);

impl<R> AutoRequest<R>
where
    R: Request,
{
    pub fn new(inner: R) -> Self {
        Self(inner)
    }
}

impl<R> Request for AutoRequest<R>
where
    R: Request,
{
    type Err = R::Err;
    type Send = R::Send;
    type SendRef = R::SendRef;

    fn send(self) -> Self::Send {
        self.0.send()
    }

    fn send_ref(&self) -> Self::SendRef {
        self.0.send_ref()
    }
}

impl<R: Request> IntoFuture for AutoRequest<R> {
    type Output = Result<Output<Self>, <Self as Request>::Err>;
    type IntoFuture = <Self as Request>::Send;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

impl<R: Request> HasPayload for AutoRequest<R> {
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.0.payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.0.payload_ref()
    }
}
