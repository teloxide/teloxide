//! Module for serializing into `multipart/form-data`
//! ([`reqwest::multipart::Form`])
//!
//! [`reqwest::multipart::Form`]: reqwest::multipart::Form
//!
//! ## How it works
//!
//! You better not know...

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
