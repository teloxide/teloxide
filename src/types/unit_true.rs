use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct True;

impl<'de> Deserialize<'de> for True {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        des.deserialize_bool(TrueVisitor)
    }
}

struct TrueVisitor;

impl<'de> Visitor<'de> for TrueVisitor {
    type Value = True;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("bool, equal to `true`")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        match value {
            true => Ok(True),
            _ => Err(E::custom("expected `true`, found `false`"))
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
    use super::True;
    use serde_json::{from_str, to_string};

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
