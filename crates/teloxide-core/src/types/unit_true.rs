use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};

/// A type that is always true.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Default)]
pub struct True;

impl std::convert::TryFrom<bool> for True {
    type Error = ();

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        match value {
            true => Ok(True),
            false => Err(()),
        }
    }
}

impl<'de> Deserialize<'de> for True {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        des.deserialize_bool(TrueVisitor)
    }
}

struct TrueVisitor;

impl Visitor<'_> for TrueVisitor {
    type Value = True;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "bool, equal to `true`")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            true => Ok(True),
            false => Err(E::custom("expected `true`, found `false`")),
        }
    }
}

impl Serialize for True {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::True;

    #[test]
    fn unit_true_de() {
        let json = "true";
        let expected = True;
        let actual = from_str(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn unit_true_se() {
        let actual = to_string(&True).unwrap();
        let expected = "true";
        assert_eq!(expected, actual);
    }
}
