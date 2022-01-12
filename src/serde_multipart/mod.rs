//! Module for serializing into `multipart/form-data`
//! ([`reqwest::multipart::Form`])
//!
//! [`reqwest::multipart::Form`]: reqwest::multipart::Form
//!
//! ## How it works
//!
//! You better not know...
//!
//! This whole module is an awful hack and we'll probably stop using it in next
//! versions (in favor of something less automatic, but more simple).

mod serializers;
mod unserializers;

use std::future::Future;

use reqwest::multipart::Form;
use serde::Serialize;

use serializers::MultipartTopLvlSerializer;

pub(crate) use serializers::Error;

/// Serializes given value into [`Form`]
///
/// [`Form`]:  reqwest::multipart::Form
pub(crate) fn to_form<T: ?Sized + Serialize>(val: &T) -> impl Future<Output = Result<Form, Error>> {
    let fut = val.serialize(MultipartTopLvlSerializer {});
    async { Ok(fut?.await?) }
}

// https://github.com/teloxide/teloxide/issues/473
#[cfg(test)]
#[tokio::test]
async fn issue_473() {
    use crate::{
        payloads::{self, SendPhotoSetters},
        types::{InputFile, MessageEntity, MessageEntityKind},
    };

    to_form(
        &payloads::SendPhoto::new(0, InputFile::file_id("0")).caption_entities([MessageEntity {
            kind: MessageEntityKind::Url,
            offset: 0,
            length: 0,
        }]),
    )
    .await
    .unwrap();
}
