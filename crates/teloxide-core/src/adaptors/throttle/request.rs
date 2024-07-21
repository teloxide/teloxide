use std::{
    future::{Future, IntoFuture},
    pin::Pin,
    sync::Arc,
    time::Instant,
};

use futures::{
    future::BoxFuture,
    task::{Context, Poll},
};
use tokio::sync::mpsc;

use crate::{
    adaptors::throttle::{channel, ChatIdHash, FreezeUntil, RequestLock},
    errors::AsResponseParameters,
    requests::{HasPayload, Output, Request},
};

/// Request returned by [`Throttling`](crate::adaptors::Throttle) methods.
#[must_use = "Requests are lazy and do nothing unless sent"]
#[derive(Clone)]
pub struct ThrottlingRequest<R: HasPayload> {
    pub(super) request: Arc<R>,
    pub(super) chat_id: fn(&R::Payload) -> ChatIdHash,
    pub(super) worker: mpsc::Sender<(ChatIdHash, RequestLock)>,
}

/// Future returned by [`ThrottlingRequest`]s.
#[pin_project::pin_project]
pub struct ThrottlingSend<R: Request>(#[pin] BoxFuture<'static, Result<Output<R>, R::Err>>);

enum ShareableRequest<R> {
    Shared(Arc<R>),
    // Option is used to `take` ownership
    Owned(Option<R>),
}

impl<R: HasPayload + Clone> HasPayload for ThrottlingRequest<R> {
    type Payload = R::Payload;

    /// Note that if this request was already executed via `send_ref` and it
    /// didn't yet completed, this method will clone the underlying request.
    fn payload_mut(&mut self) -> &mut Self::Payload {
        Arc::make_mut(&mut self.request).payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.request.payload_ref()
    }
}

impl<R> Request for ThrottlingRequest<R>
where
    R: Request + Clone + Send + Sync + 'static, // TODO: rem static
    R::Err: AsResponseParameters + Send,
    Output<R>: Send,
{
    type Err = R::Err;
    type Send = ThrottlingSend<R>;
    type SendRef = ThrottlingSend<R>;

    fn send(self) -> Self::Send {
        let chat = (self.chat_id)(self.payload_ref());
        let request = match Arc::try_unwrap(self.request) {
            Ok(owned) => ShareableRequest::Owned(Some(owned)),
            Err(shared) => ShareableRequest::Shared(shared),
        };
        let fut = send(request, chat, self.worker);

        ThrottlingSend(Box::pin(fut))
    }

    fn send_ref(&self) -> Self::SendRef {
        let chat = (self.chat_id)(self.payload_ref());
        let request = ShareableRequest::Shared(Arc::clone(&self.request));
        let fut = send(request, chat, self.worker.clone());

        ThrottlingSend(Box::pin(fut))
    }
}

impl<R> IntoFuture for ThrottlingRequest<R>
where
    R: Request + Clone + Send + Sync + 'static,
    R::Err: AsResponseParameters + Send,
    Output<R>: Send,
{
    type Output = Result<Output<Self>, <Self as Request>::Err>;
    type IntoFuture = <Self as Request>::Send;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

impl<R: Request> Future for ThrottlingSend<R>
where
    R::Err: AsResponseParameters,
{
    type Output = Result<Output<R>, R::Err>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.as_mut().project().0.poll(cx)
    }
}

// This diagram explains how `ThrottlingRequest` works/what `send` does
//
//                                          │
//                      ThrottlingRequest   │   worker()
//                                          │
//                      ┌───────────────┐   │  ┌────────────────────────┐
//  ┌──────────────────►│request is sent│   │  │see worker documentation│
//  │                   └───────┬───────┘   │  │and comments for more   │
//  │                           │           │  │information on how it   │
//  │                           ▼           │  │actually works          │
//  │                      ┌─────────┐      │  └────────────────────────┘
//  │ ┌────────────────┐   │send lock│      │
//  │ │has worker died?│◄──┤to worker├─────►:───────────┐
//  │ └─┬─────────────┬┘   └─────────┘      │           ▼
//  │   │             │                     │  ┌──────────────────┐
//  │   Y             └─N───────┐           │  │     *magic*      │
//  │   │                       │           │  └────────┬─────────┘
//  │   ▼                       ▼           │           │
//  │ ┌───────────┐    ┌────────────────┐   │           ▼
//  │ │send inner │    │wait for worker │   │  ┌─────────────────┐
//  │ │request    │    │to allow sending│◄──:◄─┤ `lock.unlock()` │
//  │ └───┬───────┘    │this request    │   │  └─────────────────┘
//  │     │            └────────┬───────┘   │
//  │     │                     │           │
//  │     ▼                     ▼           │
//  │    ┌──────┐  ┌────────────────────┐   │
//  │    │return│  │send inner request  │   │
//  │    │result│  │and check its result│   │
//  │    └──────┘  └─┬─────────┬────────┘   │
//  │     ▲    ▲     │         │            │
//  │     │    │     │ Err(RetryAfter(n))   │
//  │     │    │   else        │            │
//  │     │    │     │         ▼            │
//  │     │    └─────┘  ┌───────────────┐   │
//  │     │             │are retries on?│   │
//  │     │             └┬─────────────┬┘   │
//  │     │              │             │    │
//  │     └────────────N─┘             Y    │
//  │                                  │    │  ┌──────────────────┐
//  │                                  ▼    │  │     *magic*      │
//  │                ┌──────────────────┐   │  └──────────────────┘
// ┌┴────────────┐   │notify worker that│   │           ▲
// │retry request│◄──┤RetryAfter error  ├──►:───────────┘
// └─────────────┘   │has happened      │   │
//                   └──────────────────┘   │
//                                          │

/// Actual implementation of the `ThrottlingSend` future
async fn send<R>(
    mut request: ShareableRequest<R>,
    chat: ChatIdHash,
    worker: mpsc::Sender<(ChatIdHash, RequestLock)>,
) -> Result<Output<R>, R::Err>
where
    R: Request + Send + Sync + 'static,
    R::Err: AsResponseParameters + Send,
    Output<R>: Send,
{
    // We use option in `ShareableRequest` to `take` when sending by value.
    //
    // All unwraps down below will succeed because we always return immediately
    // after taking.

    loop {
        let (lock, wait) = channel();

        // The worker is unlikely to drop queue before sending all requests,
        // but just in case it has dropped the queue, we want to just send the
        // request.
        if worker.send((chat, lock)).await.is_err() {
            log::error!("Worker dropped the queue before sending all requests");

            let res = match &mut request {
                ShareableRequest::Shared(shared) => shared.send_ref().await,
                ShareableRequest::Owned(owned) => owned.take().unwrap().await,
            };

            return res;
        };

        let (retry, freeze) = wait.await;

        let res = match (retry, &mut request) {
            // Retries are turned on, use `send_ref` even if we have owned access
            (true, request) => {
                let request = match request {
                    ShareableRequest::Shared(shared) => &**shared,
                    ShareableRequest::Owned(owned) => owned.as_ref().unwrap(),
                };

                request.send_ref().await
            }
            (false, ShareableRequest::Shared(shared)) => shared.send_ref().await,
            (false, ShareableRequest::Owned(owned)) => owned.take().unwrap().await,
        };

        let retry_after = res.as_ref().err().and_then(<_>::retry_after);
        if let Some(retry_after) = retry_after {
            let after = retry_after.duration();
            let until = Instant::now() + after;

            // If we'll retry, we check that worker hasn't died at the start of the loop
            // otherwise we don't care if the worker is alive or not
            let _ = freeze.send(FreezeUntil { until, after, chat }).await;

            if retry {
                log::warn!("Freezing, before retrying: {:?}", retry_after);
                tokio::time::sleep_until(until.into()).await;
            }
        }

        match res {
            Err(_) if retry && retry_after.is_some() => continue,
            res => break res,
        };
    }
}
