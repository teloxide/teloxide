use std::{future::IntoFuture, pin::Pin, sync::Arc};

use futures::{
    future,
    future::{ok, Ready},
    task::{Context, Poll},
    Future,
};
use once_cell::sync::OnceCell;
use url::Url;

use crate::{
    payloads::GetMe,
    requests::{HasPayload, Request, Requester},
    types::*,
};

/// `get_me` cache.
///
/// Bot's user is hardly ever changed, so sometimes it's reasonable to cache
/// response from `get_me` method.
#[derive(Clone, Debug)]
pub struct CacheMe<B> {
    bot: B,
    me: Arc<OnceCell<Me>>,
}

impl<B> CacheMe<B> {
    /// Creates new cache.
    ///
    /// Note: it's recommended to use [`RequesterExt::cache_me`] instead.
    ///
    /// [`RequesterExt::cache_me`]: crate::requests::RequesterExt::cache_me
    pub fn new(bot: B) -> CacheMe<B> {
        Self { bot, me: Arc::new(OnceCell::new()) }
    }

    /// Allows to access inner bot
    pub fn inner(&self) -> &B {
        &self.bot
    }

    /// Unwraps inner bot
    pub fn into_inner(self) -> B {
        self.bot
    }

    /// Clear cache.
    ///
    /// Returns cached response from `get_me`, if it was cached.
    ///
    /// Note: internally this uses [`Arc::make_mut`] so this will **not**
    /// clear cache of clones of self.
    pub fn clear(&mut self) -> Option<Me> {
        Arc::make_mut(&mut self.me).take()
    }
}

macro_rules! f {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        $this.inner().$m($($arg),*)
    };
}

macro_rules! fty {
    ($T:ident) => {
        B::$T
    };
}

impl<B> Requester for CacheMe<B>
where
    B: Requester,
{
    type Err = B::Err;

    type GetMe = CachedMeRequest<B::GetMe>;

    fn get_me(&self) -> Self::GetMe {
        match self.me.get() {
            Some(me) => CachedMeRequest(Inner::Ready(me.clone()), GetMe::new()),
            None => CachedMeRequest(
                Inner::Pending(self.bot.get_me(), Arc::clone(&self.me)),
                GetMe::new(),
            ),
        }
    }

    requester_forward! {
        log_out,
        close,
        get_updates,
        set_webhook,
        delete_webhook,
        get_webhook_info,
        forward_message,
        forward_messages,
        copy_message,
        copy_messages,
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
        send_sticker,
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
        send_invoice,
        create_invoice_link,
        answer_shipping_query,
        answer_pre_checkout_query,
        get_star_transactions,
        refund_star_payment,
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
    B
    CacheMe<B>
    { this => this.inner() }
}

#[must_use = "Requests are lazy and do nothing unless sent"]
pub struct CachedMeRequest<R: Request<Payload = GetMe>>(Inner<R>, GetMe);

enum Inner<R: Request<Payload = GetMe>> {
    Ready(Me),
    Pending(R, Arc<OnceCell<Me>>),
}

impl<R> Request for CachedMeRequest<R>
where
    R: Request<Payload = GetMe>,
{
    type Err = R::Err;
    type Send = Send<R>;
    type SendRef = SendRef<R>;

    fn send(self) -> Self::Send {
        let fut = match self.0 {
            Inner::Ready(me) => future::Either::Left(ok(me)),
            Inner::Pending(req, cell) => future::Either::Right(Init(req.send(), cell)),
        };
        Send(fut)
    }

    fn send_ref(&self) -> Self::SendRef {
        let fut = match &self.0 {
            Inner::Ready(me) => future::Either::Left(ok(me.clone())),
            Inner::Pending(req, cell) => {
                future::Either::Right(Init(req.send_ref(), Arc::clone(cell)))
            }
        };
        SendRef(fut)
    }
}

impl<R: Request<Payload = GetMe>> HasPayload for CachedMeRequest<R> {
    type Payload = GetMe;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        &mut self.1
    }

    fn payload_ref(&self) -> &Self::Payload {
        &self.1
    }
}

impl<R: Request<Payload = GetMe>> IntoFuture for CachedMeRequest<R> {
    type Output = Result<Me, R::Err>;
    type IntoFuture = Send<R>;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

type ReadyMe<Err> = Ready<Result<Me, Err>>;

#[pin_project::pin_project]
pub struct Send<R: Request<Payload = GetMe>>(
    #[pin] future::Either<ReadyMe<R::Err>, Init<R::Send, Me>>,
);

impl<R: Request<Payload = GetMe>> Future for Send<R> {
    type Output = Result<Me, R::Err>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.0.poll(cx)
    }
}

#[pin_project::pin_project]
pub struct SendRef<R: Request<Payload = GetMe>>(
    #[pin] future::Either<ReadyMe<R::Err>, Init<R::SendRef, Me>>,
);

impl<R: Request<Payload = GetMe>> Future for SendRef<R> {
    type Output = Result<Me, R::Err>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.0.poll(cx)
    }
}

#[pin_project::pin_project]
struct Init<F, T>(#[pin] F, Arc<OnceCell<T>>);

impl<F: Future<Output = Result<T, E>>, T: Clone, E> Future for Init<F, T> {
    type Output = Result<T, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.0.poll(cx) {
            Poll::Ready(Ok(ok)) => Poll::Ready(Ok(this.1.get_or_init(|| ok).clone())),
            poll @ Poll::Ready(_) | poll @ Poll::Pending => poll,
        }
    }
}
