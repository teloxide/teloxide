use derive_more::From;
use mime::Mime;
use serde::{
    de::Visitor, export::Formatter, Deserialize, Deserializer, Serialize,
    Serializer,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, From)]
pub struct MimeWrapper {
    pub mime: Mime,
}

impl Serialize for MimeWrapper {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.mime.as_ref())
    }
}

struct MimeVisitor;
impl<'a> Visitor<'a> for MimeVisitor {
    type Value = MimeWrapper;

    fn expecting(
        &self,
        formatter: &mut Formatter<'_>,
    ) -> Result<(), serde::export::fmt::Error> {
        formatter.write_str("mime type")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.parse::<Mime>() {
            Ok(mime_type) => Ok(MimeWrapper { mime: mime_type }),
            Err(e) => Err(E::custom(e)),
        }
    }
}

impl<'de> Deserialize<'de> for MimeWrapper {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MimeVisitor)
    }
}
