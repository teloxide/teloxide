use crate::{payloads, requests::Payload};

/// Payloads that need to be sent as `multipart/form-data` because they contain
/// files inside.
pub trait MultipartPayload: Payload {}

impl MultipartPayload for payloads::SendMediaGroup {}

impl MultipartPayload for payloads::EditMessageMedia {}

impl MultipartPayload for payloads::EditMessageMediaInline {}
