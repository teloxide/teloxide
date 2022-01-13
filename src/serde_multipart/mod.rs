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

mod error;
mod serializers;

use std::future::Future;

use reqwest::multipart::Form;
use serde::Serialize;

use crate::requests::MultipartPayload;
use error::Error;
use serializers::MultipartSerializer;

/// Serializes given value into [`Form`] **taking all input files out**.
///
/// [`Form`]:  reqwest::multipart::Form
pub(crate) fn to_form<T>(val: &mut T) -> Result<impl Future<Output = Form>, Error>
where
    T: Serialize + MultipartPayload,
{
    let mut form = val.serialize(MultipartSerializer::new())?;

    let mut vec = Vec::with_capacity(1);
    val.move_files(&mut |f| vec.push(f));
    let iter = vec.into_iter();

    let fut = async move {
        for file in iter {
            if file.needs_attach() {
                let id = file.id().to_owned();
                if let Some(part) = file.into_part() {
                    form = form.part(id, part.await);
                }
            }
        }

        form
    };

    Ok(fut)
}

/// Serializes given value into [`Form`].
///
/// [`Form`]:  reqwest::multipart::Form
pub(crate) fn to_form_ref<T: ?Sized>(val: &T) -> Result<impl Future<Output = Form>, Error>
where
    T: Serialize + MultipartPayload,
{
    let mut form = val.serialize(MultipartSerializer::new())?;
    let mut vec = Vec::with_capacity(1);
    val.copy_files(&mut |f| vec.push(f));

    let iter = vec.into_iter();

    let fut = async move {
        for file in iter {
            if file.needs_attach() {
                let id = file.id().to_owned();
                if let Some(part) = file.into_part() {
                    form = form.part(id, part.await);
                }
            }
        }

        form
    };

    Ok(fut)
}

// https://github.com/teloxide/teloxide/issues/473
#[cfg(test)]
#[tokio::test]
async fn issue_473() {
    use crate::{
        payloads::{self, SendPhotoSetters},
        types::{InputFile, MessageEntity, MessageEntityKind},
    };

    to_form_ref(
        &payloads::SendPhoto::new(0, InputFile::file_id("0")).caption_entities([MessageEntity {
            kind: MessageEntityKind::Url,
            offset: 0,
            length: 0,
        }]),
    )
    .unwrap()
    .await;
}
