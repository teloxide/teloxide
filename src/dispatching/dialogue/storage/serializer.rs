use serde::{de::DeserializeOwned, ser::Serialize};

pub trait Serializer<D> {
    type Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error>;
    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error>;
}

pub struct JSON;

impl<D> Serializer<D> for JSON
where
    D: Serialize + DeserializeOwned,
{
    type Error = serde_json::Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(val)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        serde_json::from_slice(data)
    }
}

#[cfg(feature = "cbor-serializer")]
pub struct CBOR;

#[cfg(feature = "cbor-serializer")]
impl<D> Serializer<D> for CBOR
where
    D: Serialize + DeserializeOwned,
{
    type Error = serde_cbor::Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        serde_cbor::to_vec(val)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        serde_cbor::from_slice(data)
    }
}

#[cfg(feature = "bincode-serializer")]
pub struct Bincode;

#[cfg(feature = "bincode-serializer")]
impl<D> Serializer<D> for Bincode
where
    D: Serialize + DeserializeOwned,
{
    type Error = bincode::Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        bincode::serialize(val)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        bincode::deserialize(data)
    }
}
