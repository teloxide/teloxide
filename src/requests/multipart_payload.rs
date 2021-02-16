use crate::{payloads, requests::Payload};

/// This is a future proof trait. It is `sealed` and can change at any time.
pub trait MultipartPayload: Payload + sealed::Sealed {}

// HACK(waffle): Sealed trait allows us to change `MultipartPayload` without
//               breaking changes & refactor multipart requests later.
pub(crate) mod sealed {
    pub trait Sealed {}
}

impl sealed::Sealed for payloads::SendMediaGroup {}
impl MultipartPayload for payloads::SendMediaGroup {}

impl sealed::Sealed for payloads::EditMessageMedia {}
impl MultipartPayload for payloads::EditMessageMedia {}

impl sealed::Sealed for payloads::EditMessageMediaInline {}
impl MultipartPayload for payloads::EditMessageMediaInline {}
