use std::fmt;

use mime::Mime;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

pub(crate) mod deser {
    use mime::Mime;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{MimeDe, MimeSer};

    pub(crate) fn serialize<S>(
        this: &Mime,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        MimeSer(this).serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Mime, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        MimeDe::deserialize(deserializer).map(|MimeDe(m)| m)
    }
}

pub(crate) mod opt_deser {
    use mime::Mime;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{MimeDe, MimeSer};

    pub(crate) fn serialize<S>(
        this: &Option<Mime>,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        this.as_ref().map(MimeSer).serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Mime>, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<MimeDe>::deserialize(deserializer).map(|opt| opt.map(|MimeDe(m)| m))
    }
}

struct MimeSer<'a>(&'a Mime);

impl Serialize for MimeSer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_ref())
    }
}

struct MimeVisitor;
impl Visitor<'_> for MimeVisitor {
    type Value = MimeDe;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        formatter.write_str("mime type")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.parse::<Mime>() {
            Ok(mime_type) => Ok(MimeDe(mime_type)),
            Err(e) => Err(E::custom(e)),
        }
    }
}

struct MimeDe(Mime);

impl<'de> Deserialize<'de> for MimeDe {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MimeVisitor)
    }
}
