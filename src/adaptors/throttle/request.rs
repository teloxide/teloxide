use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    time::{Duration, Instant},
};

use either::Either;
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

#[must_use = "Requests are lazy and do nothing unless sent"]
pub struct ThrottlingRequest<R: HasPayload> {
    pub(super) request: Arc<R>,
    pub(super) chat_id: fn(&R::Payload) -> ChatIdHash,
    pub(super) worker: mpsc::Sender<(ChatIdHash, RequestLock)>,
}

#[pin_project::pin_project]
pub struct ThrottlingSend<R: Request>(#[pin] BoxFuture<'static, Result<Output<R>, R::Err>>);

impl<R: HasPayload + Clone> HasPayload for ThrottlingRequest<R> {
    type Payload = R::Payload;

    /// Note that if this request was already executed via `send_ref` and it
    /// didn't yet completed, this method will clone the underlying request
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
        let request = Either::from(Arc::try_unwrap(self.request));

        ThrottlingSend(Box::pin(send(request, chat, self.worker)))
    }

    fn send_ref(&self) -> Self::SendRef {
        let chat = (self.chat_id)(self.payload_ref());
        let request = Either::Left(Arc::clone(&self.request));

        ThrottlingSend(Box::pin(send(request, chat, self.worker.clone())))
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

/// Actual implementation of the `ThrottlingSend` future
async fn send<R>(
    request: Either<Arc<R>, R>,
    chat: ChatIdHash,
    worker: mpsc::Sender<(ChatIdHash, RequestLock)>,
) -> Result<Output<R>, R::Err>
where
    R: Request + Send + Sync + 'static,
    R::Err: AsResponseParameters + Send,
    Output<R>: Send,
{
    // We use option to `take` when sending by value.
    //
    // All unwraps down below will succed because we always return immediately after
    // taking.
    let mut request: Either<Arc<R>, Option<R>> = request.map_right(Some);

    loop {
        let (lock, wait) = channel();

        // The worker is unlikely to drop queue before sending all requests,
        // but just in case it has dropped the queue, we want to just send the
        // request.
        if let Err(_) = worker.send((chat, lock)).await {
            return match &mut request {
                Either::Left(shared) => shared.send_ref().await,
                Either::Right(owned) => owned.take().unwrap().send().await,
            };
        };

        let (retry, freeze) = wait.await;

        let res = match (retry, &mut request) {
            (true, request) => {
                request
                    .as_ref()
                    .either(|r| &**r, |r| r.as_ref().unwrap())
                    .send_ref()
                    .await
            }
            (false, Either::Left(shared)) => shared.send_ref().await,
            (false, Either::Right(owned)) => owned.take().unwrap().send().await,
        };

        let retry_after = res.as_ref().err().and_then(<_>::retry_after);
        if let Some(retry_after) = retry_after {
            let after = Duration::from_secs(retry_after.into());

            if retry {
                log::warn!("Freezing, before retrying: {}", retry_after);
            }

            let (lock, wait) = channel();

            // Error here means that the worker died, so we can't really do anything about
            // it
            let _ = freeze
                .send(FreezeUntil {
                    until: Instant::now(), // TODO: this is obviously wrong
                    after,
                    chat,
                    retry: Some(lock),
                })
                .await;

            wait.await;
        }

        match res {
            res @ Ok(_) => break res,
            res @ Err(_) if !retry => break res,
            Err(_) => {
                // Next iteration will retry
            }
        };
    }
}
