use std::{
    fmt::Debug,
    future::{Future, IntoFuture},
    pin::Pin,
    task::{self, Poll},
};

use futures::ready;
use url::Url;

use crate::{
    requests::{HasPayload, Output, Payload, Request, Requester},
    types::*,
};

/// Trace requests and responses.
///
/// This is a tool for debugging.
///
/// Depending on [`Settings`] and `log` facade this adaptor may output messages
/// like these:
/// ```text
/// TRACE teloxide_core::adaptors::trace > Sending `SendDice` request
/// TRACE teloxide_core::adaptors::trace > Got response from `SendDice` request
/// TRACE teloxide_core::adaptors::trace > Sending `SendDice` request: SendDice { chat_id: Id(0), emoji: Some(Dice), disable_notification: None, reply_to_message_id: None, allow_sending_without_reply: None, reply_markup: None }
/// TRACE teloxide_core::adaptors::trace > Got response from `SendDice` request: Ok(Message { id: 13812, date: 1625926524, chat: Chat { .. }, via_bot: None, kind: Dice(MessageDice { dice: Dice { emoji: Dice, value: 3 } }) })
/// ```
#[derive(Clone, Debug)]
pub struct Trace<B> {
    inner: B,
    settings: Settings,
}

impl<B> Trace<B> {
    pub fn new(inner: B, settings: Settings) -> Self {
        Self { inner, settings }
    }

    pub fn inner(&self) -> &B {
        &self.inner
    }

    pub fn into_inner(self) -> B {
        self.inner
    }

    pub fn settings(&self) -> Settings {
        self.settings
    }
}

bitflags::bitflags! {
    /// [`Trace`] settings that determine what will be logged.
    ///
    /// ## Examples
    ///
    /// ```
    /// use teloxide_core::adaptors::trace::Settings;
    ///
    /// // Trace nothing
    /// let _ = Settings::empty();
    /// // Trace only requests
    /// let _ = Settings::TRACE_REQUESTS;
    /// // Trace requests verbosely and responses (non verbosely)
    /// let _ = Settings::TRACE_REQUESTS_VERBOSE | Settings::TRACE_RESPONSES;
    /// ```
    pub struct Settings: u8 {
        /// Trace requests (only request kind, e.g. `send_message`)
        const TRACE_REQUESTS = 1;

        /// Trace requests verbosely (with all parameters).
        ///
        /// Implies [`TRACE_REQUESTS`]
        const TRACE_REQUESTS_VERBOSE = (1 << 1) | Self::TRACE_REQUESTS.bits;

        /// Trace responses (only request kind, e.g. `send_message`)
        const TRACE_RESPONSES = 1 << 2;

        /// Trace responses verbosely (with full response).
        ///
        /// Implies [`TRACE_RESPONSES`]
        const TRACE_RESPONSES_VERBOSE = (1 << 3) | Self::TRACE_RESPONSES.bits;

        /// Trace everything.
        ///
        /// Implies [`TRACE_REQUESTS`] and [`TRACE_RESPONSES`].
        const TRACE_EVERYTHING = Self::TRACE_REQUESTS.bits | Self::TRACE_RESPONSES.bits;

        /// Trace everything verbosely.
        ///
        /// Implies [`TRACE_REQUESTS_VERBOSE`] and [`TRACE_RESPONSES_VERBOSE`].
        const TRACE_EVERYTHING_VERBOSE = Self::TRACE_REQUESTS_VERBOSE.bits | Self::TRACE_RESPONSES_VERBOSE.bits;
    }
}

macro_rules! fty {
    ($T:ident) => {
        TraceRequest<B::$T>
    };
}

macro_rules! fwd_inner {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        TraceRequest {
            inner: $this.inner().$m($($arg),*),
            settings: $this.settings
        }
    };
}

impl<B> Requester for Trace<B>
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
        => fwd_inner, fty
    }
}

#[must_use = "Requests are lazy and do nothing unless sent"]
pub struct TraceRequest<R> {
    inner: R,
    settings: Settings,
}

impl<R> TraceRequest<R>
where
    R: Request,
{
    fn trace_request(&self)
    where
        R::Payload: Debug,
    {
        if self.settings.contains(Settings::TRACE_REQUESTS_VERBOSE) {
            log::trace!(
                "Sending `{}` request: {:?}",
                <R::Payload as Payload>::NAME,
                self.inner.payload_ref()
            );
        } else if self.settings.contains(Settings::TRACE_REQUESTS) {
            log::trace!("Sending `{}` request", R::Payload::NAME);
        }
    }

    fn trace_response_fn(&self) -> fn(&Result<Output<R>, R::Err>)
    where
        Output<R>: Debug,
        R::Err: Debug,
    {
        if self.settings.contains(Settings::TRACE_RESPONSES_VERBOSE) {
            |response| {
                log::trace!("Got response from `{}` request: {:?}", R::Payload::NAME, response)
            }
        } else if self.settings.contains(Settings::TRACE_RESPONSES) {
            |_| log::trace!("Got response from `{}` request", R::Payload::NAME)
        } else {
            |_| {}
        }
    }
}

impl<R> HasPayload for TraceRequest<R>
where
    R: HasPayload,
{
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.inner.payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.inner.payload_ref()
    }
}

impl<R> Request for TraceRequest<R>
where
    R: Request,
    Output<R>: Debug,
    R::Err: Debug,
    R::Payload: Debug,
{
    type Err = R::Err;

    type Send = Send<R::Send>;

    type SendRef = Send<R::SendRef>;

    fn send(self) -> Self::Send {
        self.trace_request();

        Send { trace_fn: self.trace_response_fn(), inner: self.inner.send() }
    }

    fn send_ref(&self) -> Self::SendRef {
        self.trace_request();

        Send { trace_fn: self.trace_response_fn(), inner: self.inner.send_ref() }
    }
}

impl<R> IntoFuture for TraceRequest<R>
where
    R: Request,
    Output<R>: Debug,
    R::Err: Debug,
    R::Payload: Debug,
{
    type Output = Result<Output<Self>, <Self as Request>::Err>;
    type IntoFuture = <Self as Request>::Send;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

#[pin_project::pin_project]
pub struct Send<F>
where
    F: Future,
{
    trace_fn: fn(&F::Output),
    #[pin]
    inner: F,
}

impl<F> Future for Send<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let ret = ready!(this.inner.poll(cx));
        (this.trace_fn)(&ret);
        Poll::Ready(ret)
    }
}
