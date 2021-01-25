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
