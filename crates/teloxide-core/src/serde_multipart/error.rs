use std::{fmt, sync::Arc};

use serde::ser;

use crate::RequestError;

#[derive(Debug, derive_more::From)]
pub(crate) enum Error {
    Custom(String),
    TopLevelNotStruct,
    Fmt(std::fmt::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(s) => write!(f, "Custom serde error: {s}"),
            Self::TopLevelNotStruct => write!(f, "Multipart supports only structs at top level"),
            Self::Fmt(inner) => write!(f, "Formatting error: {inner}"),
            Self::Io(inner) => write!(f, "Io error: {inner}"),
            Self::Json(inner) => write!(f, "Json (de)serialization error: {inner}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<Error> for RequestError {
    fn from(err: Error) -> Self {
        match err {
            Error::Io(ioerr) => RequestError::Io(Arc::new(ioerr)),

            // This should be ok since we (hopefuly) don't write request those may trigger errors
            // and `Error` is internal.
            e => unreachable!(
                "we don't create requests those fail to serialize (if you see this, open an issue \
                 :|): {}",
                e
            ),
        }
    }
}
