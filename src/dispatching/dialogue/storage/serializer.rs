use serde::{de::DeserializeOwned, ser::Serialize};
use thiserror::Error;
use Serializer::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed parsing/serializing JSON: {0}")]
    JSONError(#[from] serde_json::Error),
    #[cfg(feature = "cbor-serializer")]
    #[error("failed parsing/serializing CBOR: {0}")]
    CBORError(#[from] serde_cbor::Error),
    #[cfg(feature = "bincode-serializer")]
    #[error("failed parsing/serializing Bincode: {0}")]
    BincodeError(#[from] bincode::Error),
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Serializer {
    JSON,
    #[cfg(feature = "cbor-serializer")]
    CBOR,
    #[cfg(feature = "bincode-serializer")]
    Bincode,
}

impl Serializer {
    pub fn serialize<D>(&self, val: &D) -> Result<Vec<u8>>
    where
        D: Serialize,
    {
        Ok(match self {
            JSON => serde_json::to_vec(val)?,
            #[cfg(feature = "cbor-serializer")]
            CBOR => serde_cbor::to_vec(val)?,
            #[cfg(feature = "bincode-serializer")]
            Bincode => bincode::serialize(val)?,
        })
    }

    pub fn deserialize<'de, D>(&self, data: &'de [u8]) -> Result<D>
    where
        D: DeserializeOwned,
    {
        Ok(match self {
            JSON => serde_json::from_slice(data)?,
            #[cfg(feature = "cbor-serializer")]
            CBOR => serde_cbor::from_slice(data)?,
            #[cfg(feature = "bincode-serializer")]
            Bincode => bincode::deserialize(data)?,
        })
    }
}
