use std::{pin::Pin, sync::Arc};

use futures::{
    future,
    future::{ok, Ready},
    task::{Context, Poll},
    Future,
};
use once_cell::sync::OnceCell;

use crate::{
    payloads::GetMe,
    requests::{HasPayload, Request, Requester},
    types::{ChatId, User, *},
};

/// `get_me` cache.
///
/// Bot's user is hardly ever changed, so sometimes it's reasonable to cache
/// response from `get_me` method.
pub struct CacheMe<B> {
    bot: B,
    me: Arc<OnceCell<User>>,
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
    pub fn clear(&mut self) -> Option<User> {
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
            Some(user) => CachedMeRequest(Inner::Ready(user.clone()), GetMe::new()),
            None => CachedMeRequest(
                Inner::Pending(self.bot.get_me(), Arc::clone(&self.me)),
                GetMe::new(),
            ),
        }
    }

    type SendMessage = B::SendMessage;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        self.bot.send_message(chat_id, text)
    }

    requester_forward! {
        get_updates, set_webhook, delete_webhook, get_webhook_info,
        forward_message, send_photo, send_audio, send_document, send_video,
        send_animation, send_voice, send_video_note, send_media_group, send_location,
        edit_message_live_location, edit_message_live_location_inline,
        stop_message_live_location, stop_message_live_location_inline, send_venue,
        send_contact, send_poll, send_dice, send_chat_action, get_user_profile_photos,
        get_file, kick_chat_member, unban_chat_member, restrict_chat_member,
        promote_chat_member, set_chat_administrator_custom_title, set_chat_permissions,
        export_chat_invite_link, set_chat_photo, delete_chat_photo, set_chat_title,
        set_chat_description, pin_chat_message, unpin_chat_message, leave_chat,
        get_chat, get_chat_administrators, get_chat_members_count,get_chat_member,
        set_chat_sticker_set, delete_chat_sticker_set, answer_callback_query,
        set_my_commands, get_my_commands, answer_inline_query, edit_message_text,
        edit_message_text_inline, edit_message_caption, edit_message_caption_inline,
        edit_message_media, edit_message_media_inline, edit_message_reply_markup,
        edit_message_reply_markup_inline, stop_poll, delete_message, send_sticker,
        get_sticker_set, upload_sticker_file, create_new_sticker_set,
        add_sticker_to_set, set_sticker_position_in_set, delete_sticker_from_set,
        set_sticker_set_thumb, send_invoice, answer_shipping_query,
        answer_pre_checkout_query, set_passport_data_errors, send_game,
        set_game_score, set_game_score_inline, get_game_high_scores => f, fty
    }
}

download_forward! {
    'w
    B
    CacheMe<B>
    { this => this.inner() }
}

pub struct CachedMeRequest<R: Request<Payload = GetMe>>(Inner<R>, GetMe);

enum Inner<R: Request<Payload = GetMe>> {
    Ready(User),
    Pending(R, Arc<OnceCell<User>>),
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
            Inner::Ready(user) => future::Either::Left(ok(user)),
            Inner::Pending(req, cell) => future::Either::Right(Init(req.send(), cell)),
        };
        Send(fut)
    }

    fn send_ref(&self) -> Self::SendRef {
        let fut = match &self.0 {
            Inner::Ready(user) => future::Either::Left(ok(user.clone())),
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

type ReadyUser<Err> = Ready<Result<User, Err>>;

#[pin_project::pin_project]
pub struct Send<R: Request<Payload = GetMe>>(
    #[pin] future::Either<ReadyUser<R::Err>, Init<R::Send, User>>,
);

impl<R: Request<Payload = GetMe>> Future for Send<R> {
    type Output = Result<User, R::Err>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.0.poll(cx)
    }
}

#[pin_project::pin_project]
pub struct SendRef<R: Request<Payload = GetMe>>(
    #[pin] future::Either<ReadyUser<R::Err>, Init<R::SendRef, User>>,
);

impl<R: Request<Payload = GetMe>> Future for SendRef<R> {
    type Output = Result<User, R::Err>;

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
