use std::future::IntoFuture;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    bot::Bot,
    requests::{HasPayload, MultipartPayload, Payload, Request, ResponseResult},
    RequestError,
};

/// A ready-to-send Telegram request whose payload is sent using
/// [multipart/form-data].
///
/// [multipart/form-data]: https://core.telegram.org/bots/api#making-requests
#[must_use = "Requests are lazy and do nothing unless sent"]
#[derive(Clone)]
pub struct MultipartRequest<P> {
    bot: Bot,
    payload: P,
}

impl<P> MultipartRequest<P> {
    pub const fn new(bot: Bot, payload: P) -> Self {
        Self { bot, payload }
    }
}

impl<P> Request for MultipartRequest<P>
where
    // FIXME(waffle):
    //   this is required on stable because of
    //   https://github.com/rust-lang/rust/issues/76882
    //   when it's resolved or `type_alias_impl_trait` feature
    //   stabilized, we should remove 'static restriction
    //
    // (though critically, currently we have no
    // non-'static payloads)
    P: 'static,
    P: Payload + MultipartPayload + Serialize,
    P::Output: DeserializeOwned,
{
    type Err = RequestError;
    type Send = Send<P>;
    type SendRef = SendRef<P>;

    fn send(self) -> Self::Send {
        Send::new(self)
    }

    fn send_ref(&self) -> Self::SendRef {
        SendRef::new(self)
    }
}

impl<P> IntoFuture for MultipartRequest<P>
where
    P: 'static,
    P: Payload + MultipartPayload + Serialize,
    P::Output: DeserializeOwned,
{
    type Output = Result<P::Output, RequestError>;
    type IntoFuture = <Self as Request>::Send;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

impl<P> HasPayload for MultipartRequest<P>
where
    P: Payload,
{
    type Payload = P;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        &mut self.payload
    }

    fn payload_ref(&self) -> &Self::Payload {
        &self.payload
    }
}

impl<P> core::ops::Deref for MultipartRequest<P>
where
    P: 'static,
    P: Payload + MultipartPayload,
    P::Output: DeserializeOwned,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.payload_ref()
    }
}

impl<P> core::ops::DerefMut for MultipartRequest<P>
where
    P: 'static,
    P: Payload + MultipartPayload,
    P::Output: DeserializeOwned,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.payload_mut()
    }
}

req_future! {
    def: |it: MultipartRequest<U>| {
        it.bot.execute_multipart(&mut {it.payload})
    }
    pub Send<U> (inner0) -> ResponseResult<U::Output>
    where
        U: 'static,
        U: Payload + MultipartPayload + Serialize,
        U::Output: DeserializeOwned,
}

req_future! {
    def: |it: &MultipartRequest<U>| {
        it.bot.execute_multipart_ref(&it.payload)
    }
    pub SendRef<U> (inner1) -> ResponseResult<U::Output>
    where
        U: 'static,
        U: Payload + MultipartPayload + Serialize,
        U::Output: DeserializeOwned,
}
