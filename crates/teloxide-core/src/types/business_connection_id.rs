use serde::{Deserialize, Serialize};

/// A unique business connection identifier.
#[derive(Default, Clone, Debug, derive_more::Display, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct BusinessConnectionId(pub String);

#[cfg(test)]
mod tests {
    use crate::types::BusinessConnectionId;

    #[test]
    fn business_connection_id_deser() {
        let json = r#""abcd1234""#;
        let bcid: BusinessConnectionId = serde_json::from_str(json).unwrap();
        assert_eq!(bcid, BusinessConnectionId(String::from("abcd1234")));
    }

    #[test]
    fn business_connection_id_ser() {
        let bcid: BusinessConnectionId = BusinessConnectionId(String::from("abcd1234"));
        let json = serde_json::to_string(&bcid).unwrap();
        assert_eq!(json, r#""abcd1234""#);
    }
}
