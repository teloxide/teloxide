use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::requests::{HasPayload, Output, Request, Requester};

/// Send requests automatically.
///
/// Requests returned by `<AutoSend<_> as `[`Requester`]`>` are [`Future`]s
/// which means that you can simply `.await` them instead of using
/// `.send().await`.
///
/// Notes:
/// 1. This wrapper should be the most outer i.e.: `AutoSend<CacheMe<Bot>>`
///    will automatically send requests, while `CacheMe<AutoSend<Bot>>` - won't.
/// 2. After first call to `poll` on a request you will unable to access payload
///    nor could you use [`send_ref`](Request::send_ref)
///
/// ## Examples
///
/// ```rust
/// use teloxide_core::{
///     requests::{Requester, RequesterExt},
///     types::User,
///     Bot,
/// };
///
/// # async {
/// let bot = Bot::new("TOKEN").auto_send();
/// let myself: User = bot.get_me().await?; // No .send()!
/// # Ok::<_, teloxide_core::RequestError>(()) };
/// ```
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

    /// Allows to access inner bot
    pub fn inner(&self) -> &B {
        &self.bot
    }

    /// Unwraps inner bot
    pub fn into_inner(self) -> B {
        self.bot
    }
}

impl<B: Requester> Requester for AutoSend<B> {
    type GetMe = AutoRequest<B::GetMe>;

    fn get_me(&self) -> Self::GetMe {
        AutoRequest::new(self.bot.get_me())
    }
}

#[pin_project::pin_project]
pub struct AutoRequest<R: Request>(#[pin] Inner<R>);

impl<R: Request> AutoRequest<R> {
    pub fn new(inner: R) -> Self {
        Self(Inner::Request(inner))
    }
}

#[pin_project::pin_project(project = InnerProj, project_replace = InnerRepl)]
enum Inner<R: Request> {
    /// Unsent modifiable request
    Request(R),
    /// Sent request
    Future(#[pin] R::Send),
    /// This is mostly redundant variant that is used only to take ownership
    /// over `Request(R)` before creating `Future(R::Send)` in
    /// `AutoRequest::poll`.
    ///
    /// Practically we don't create this variant anywhere else and it can be
    /// ignored.
    Tmp,
}

impl<R: Request> Request for AutoRequest<R> {
    type Err = R::Err;
    type Send = R::Send;
    type SendRef = R::SendRef;

    fn send(self) -> Self::Send {
        match self.0 {
            Inner::Request(req) => req.send(),
            Inner::Future(fut) => fut,
            Inner::Tmp => tmp_unreachable(),
        }
    }

    fn send_ref(&self) -> Self::SendRef {
        match &self.0 {
            Inner::Request(req) => req.send_ref(),
            Inner::Future(_) => already_polled(),
            Inner::Tmp => tmp_unreachable(),
        }
    }
}

impl<R: Request> HasPayload for AutoRequest<R> {
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        match &mut self.0 {
            Inner::Request(req) => req.payload_mut(),
            Inner::Future(_) => already_polled(),
            Inner::Tmp => tmp_unreachable(),
        }
    }

    fn payload_ref(&self) -> &Self::Payload {
        match &self.0 {
            Inner::Request(req) => req.payload_ref(),
            Inner::Future(_) => already_polled(),
            Inner::Tmp => tmp_unreachable(),
        }
    }
}

impl<R: Request> Future for AutoRequest<R> {
    type Output = Result<Output<R>, R::Err>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project();

        if let InnerProj::Future(fut) = this.0.as_mut().project() {
            return fut.poll(cx);
        }

        let inner = this.0.as_mut().project_replace(Inner::Tmp);
        let inner = match inner {
            InnerRepl::Request(req) => Inner::Future(req.send()),
            // Practically this is unreachable
            InnerRepl::Future(_) | InnerRepl::Tmp => tmp_unreachable(),
        };
        this.0.as_mut().project_replace(inner);

        self.poll(cx)
    }
}

#[inline(never)]
fn tmp_unreachable() -> ! {
    unreachable!("tmp is not created outside of AutoRequest::poll")
}

#[inline(never)]
fn already_polled() -> ! {
    panic!("AutoRequest was already polled")
}
