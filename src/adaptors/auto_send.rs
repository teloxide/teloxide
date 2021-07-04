use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures::future::FusedFuture;
use url::Url;

use crate::{
    requests::{HasPayload, Output, Request, Requester},
    types::*,
};

/// Send requests automatically.
///
/// Requests returned by `<AutoSend<_> as `[`Requester`]`>` are [`Future`]s
/// which means that you can simply `.await` them instead of using
/// `.send().await`.
///
/// Notes:
/// 1. This wrapper should be the most outer i.e.: `AutoSend<CacheMe<Bot>>`
///    will automatically send requests, while `CacheMe<AutoSend<Bot>>` - won't.
/// 2. After first call to `poll` on a request you will be unable to access
///    payload nor could you use [`send_ref`](Request::send_ref).
///
/// ## Examples
///
/// ```rust
/// use teloxide_core::{
///     requests::{Requester, RequesterExt},
///     types::Me,
///     Bot,
/// };
///
/// # async {
/// let bot = Bot::new("TOKEN").auto_send();
/// let myself: Me = bot.get_me().await?; // No .send()!
/// # Ok::<_, teloxide_core::RequestError>(()) };
/// ```
#[derive(Clone)]
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
        get_me, log_out, close, get_updates, set_webhook, delete_webhook, get_webhook_info,
        forward_message, copy_message, send_message, send_photo, send_audio, send_document,
        send_video, send_animation, send_voice, send_video_note, send_media_group, send_location,
        edit_message_live_location, edit_message_live_location_inline,
        stop_message_live_location, stop_message_live_location_inline, send_venue,
        send_contact, send_poll, send_dice, send_chat_action, get_user_profile_photos,
        get_file, kick_chat_member, ban_chat_member, unban_chat_member, restrict_chat_member,
        promote_chat_member, set_chat_administrator_custom_title, set_chat_permissions,
        export_chat_invite_link, create_chat_invite_link, edit_chat_invite_link,
        revoke_chat_invite_link, set_chat_photo, delete_chat_photo, set_chat_title,
        set_chat_description, pin_chat_message, unpin_chat_message, unpin_all_chat_messages,
        leave_chat, get_chat, get_chat_administrators, get_chat_members_count, get_chat_member_count, get_chat_member,
        set_chat_sticker_set, delete_chat_sticker_set, answer_callback_query,
        set_my_commands, get_my_commands, delete_my_commands, answer_inline_query, edit_message_text,
        edit_message_text_inline, edit_message_caption, edit_message_caption_inline,
        edit_message_media, edit_message_media_inline, edit_message_reply_markup,
        edit_message_reply_markup_inline, stop_poll, delete_message, send_sticker,
        get_sticker_set, upload_sticker_file, create_new_sticker_set,
        add_sticker_to_set, set_sticker_position_in_set, delete_sticker_from_set,
        set_sticker_set_thumb, send_invoice, answer_shipping_query,
        answer_pre_checkout_query, set_passport_data_errors, send_game,
        set_game_score, set_game_score_inline, get_game_high_scores,
        get_updates_fault_tolerant => f, fty
    }
}

download_forward! {
    'w
    B
    AutoSend<B>
    { this => this.inner() }
}

#[pin_project::pin_project]
pub struct AutoRequest<R: Request>(#[pin] Inner<R>);

impl<R> AutoRequest<R>
where
    R: Request,
{
    pub fn new(inner: R) -> Self {
        Self(Inner::Request(inner))
    }
}

/// Data of the `AutoRequest` used to not expose variants (I wish there were
/// private enum variants).
#[pin_project::pin_project(project = InnerProj, project_replace = InnerRepl)]
enum Inner<R: Request> {
    /// An unsent modifiable request.
    Request(R),
    /// A sent request.
    Future(#[pin] R::Send),
    /// Done state. Set after `R::Send::poll` returned `Ready(_)`.
    ///
    /// Also used as a temporary replacement to turn pinned `Request(req)`
    /// into `Future(req.send())` in `AutoRequest::poll`.
    Done,
}

impl<R> Request for AutoRequest<R>
where
    R: Request,
{
    type Err = R::Err;
    type Send = R::Send;
    type SendRef = R::SendRef;

    fn send(self) -> Self::Send {
        match self.0 {
            Inner::Request(req) => req.send(),
            Inner::Future(fut) => fut,
            Inner::Done => done_unreachable(),
        }
    }

    fn send_ref(&self) -> Self::SendRef {
        match &self.0 {
            Inner::Request(req) => req.send_ref(),
            Inner::Future(_) => already_polled(),
            Inner::Done => done_unreachable(),
        }
    }
}

impl<R: Request> HasPayload for AutoRequest<R> {
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        match &mut self.0 {
            Inner::Request(req) => req.payload_mut(),
            Inner::Future(_) => already_polled(),
            Inner::Done => done_unreachable(),
        }
    }

    fn payload_ref(&self) -> &Self::Payload {
        match &self.0 {
            Inner::Request(req) => req.payload_ref(),
            Inner::Future(_) => already_polled(),
            Inner::Done => done_unreachable(),
        }
    }
}

impl<R: Request> Future for AutoRequest<R> {
    type Output = Result<Output<R>, R::Err>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this: Pin<&mut Inner<_>> = self.as_mut().project().0;

        match this.as_mut().project() {
            // Poll the underling future.
            InnerProj::Future(fut) => {
                let res = futures::ready!(fut.poll(cx));
                // We've got the result, so we set the state to done.
                this.set(Inner::Done);
                Poll::Ready(res)
            }

            // This future is fused.
            InnerProj::Done => Poll::Pending,
            // The `AutoRequest` future was polled for the first time after
            // creation. We need to transform it into sent form by calling
            // `R::send` and doing some magic around Pin.
            InnerProj::Request(_) => {
                // Replace `Request(_)` by `Done(_)` to obtain ownership over
                // the former.
                let inner = this.as_mut().project_replace(Inner::Done);
                // Map Request(req) to `Future(req.send())`.
                let inner = match inner {
                    InnerRepl::Request(req) => Inner::Future(req.send()),
                    // Practically this is unreachable, because we've just checked for
                    // both `Future(_)` and `Done` variants.
                    InnerRepl::Future(_) | InnerRepl::Done => done_unreachable(),
                };
                // Set the resulting `Future(_)` back to pin.
                this.set(inner);

                // Poll `self`. This time another brunch will be executed, returning `Poll`.
                self.poll(cx)
            }
        }
    }
}

impl<R: Request> FusedFuture for AutoRequest<R> {
    fn is_terminated(&self) -> bool {
        matches!(&self.0, Inner::Done)
    }
}

#[inline(never)]
fn done_unreachable() -> ! {
    unreachable!("future is completed and as such doesn't provide any functionality")
}

#[inline(never)]
fn already_polled() -> ! {
    panic!("AutoRequest was already polled once")
}
