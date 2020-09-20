use serde::{de::DeserializeOwned, Serialize};

use crate::{
    bot::Bot,
    requests::{HasPayload, Payload, Request, ResponseResult},
    RequestError,
};

/// Ready-to-send telegram request.
///
/// Note: payload will be sent to telegram using [`multipart/form-data`]
///
/// [`multipart/form-data`]: https://core.telegram.org/bots/api#making-requests
#[must_use = "requests do nothing until sent"]
pub struct MultipartRequest<P> {
    bot: Bot,
    payload: P,
}

impl<P> MultipartRequest<P> {
    pub fn new(bot: Bot, payload: P) -> Self {
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
    P: Payload + Serialize,
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

impl<P> HasPayload for MultipartRequest<P>
where
    P: Payload,
{
    type Payload = P;
}

impl<P> AsRef<P> for MultipartRequest<P> {
    fn as_ref(&self) -> &P {
        &self.payload
    }
}

impl<P> AsMut<P> for MultipartRequest<P> {
    fn as_mut(&mut self) -> &mut P {
        &mut self.payload
    }
}

impl<P> core::ops::Deref for MultipartRequest<P>
where
    P: 'static,
    P: Payload + Serialize,
    P::Output: DeserializeOwned,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<P> core::ops::DerefMut for MultipartRequest<P>
where
    P: 'static,
    P: Payload + Serialize,
    P::Output: DeserializeOwned,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

req_future! {
    def: |it: MultipartRequest<U>| {
        it.bot.execute_multipart(&it.payload)
    }
    pub Send<U> (inner0) -> ResponseResult<U::Output>
    where
        U: 'static,
        U: Payload + Serialize,
        U::Output: DeserializeOwned,
}

req_future! {
    def: |it: &MultipartRequest<U>| {
        it.bot.execute_multipart(&it.payload)
    }
    pub SendRef<U> (inner1) -> ResponseResult<U::Output>
    where
        U: 'static,
        U: Payload + Serialize,
        U::Output: DeserializeOwned,
}
