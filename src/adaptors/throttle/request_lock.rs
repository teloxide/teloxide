use std::pin::Pin;

use futures::{
    task::{Context, Poll},
    Future,
};
use tokio::sync::{
    mpsc,
    oneshot::{self, Receiver, Sender},
};

use crate::adaptors::throttle::FreezeUntil;

pub(super) fn channel() -> (RequestLock, RequestWaiter) {
    let (tx, rx) = oneshot::channel();
    let tx = RequestLock(tx);
    let rx = RequestWaiter(rx);
    (tx, rx)
}

#[must_use]
pub(super) struct RequestLock(Sender<(bool, mpsc::Sender<FreezeUntil>)>);

#[must_use]
#[pin_project::pin_project]
pub(super) struct RequestWaiter(#[pin] Receiver<(bool, mpsc::Sender<FreezeUntil>)>);

impl RequestLock {
    pub(super) fn unlock(self, retry: bool, freeze: mpsc::Sender<FreezeUntil>) -> Result<(), ()> {
        self.0.send((retry, freeze)).map_err(drop)
    }
}

impl Future for RequestWaiter {
    type Output = (bool, mpsc::Sender<FreezeUntil>);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.0.poll(cx) {
            Poll::Ready(Ok(ret)) => Poll::Ready(ret),
            Poll::Ready(Err(_)) => panic!("`RequestLock` is dropped by the throttle worker"),
            Poll::Pending => Poll::Pending,
        }
    }
}
