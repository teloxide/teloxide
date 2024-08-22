use serde::{Deserialize, Serialize};

/// A unique business connection identifier.
#[derive(Default, Clone, Debug, derive_more::Display, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "BusinessConnectionIdRaw", into = "BusinessConnectionIdRaw")]
pub struct BusinessConnectionId(pub String);

#[derive(Serialize, Deserialize)]
struct BusinessConnectionIdRaw {
    business_connection_id: String,
}

impl From<BusinessConnectionIdRaw> for BusinessConnectionId {
    fn from(BusinessConnectionIdRaw { business_connection_id }: BusinessConnectionIdRaw) -> Self {
        BusinessConnectionId(business_connection_id)
    }
}

impl From<BusinessConnectionId> for BusinessConnectionIdRaw {
    fn from(BusinessConnectionId(business_connection_id): BusinessConnectionId) -> Self {
        BusinessConnectionIdRaw { business_connection_id }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::BusinessConnectionId;

    #[test]
    fn business_connection_id_deser() {
        let json = r#"{"business_connection_id": "abcd1234"}"#;
        let bcid: BusinessConnectionId = serde_json::from_str(json).unwrap();
        assert_eq!(bcid, BusinessConnectionId(String::from("abcd1234")));
    }

    #[test]
    fn business_connection_id_ser() {
        let bcid: BusinessConnectionId = BusinessConnectionId(String::from("abcd1234"));
        let json = serde_json::to_string(&bcid).unwrap();
        assert_eq!(json, r#"{"business_connection_id":"abcd1234"}"#);
    }
}
