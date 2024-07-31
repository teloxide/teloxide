use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct RequestId(pub i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_id_de() {
        #[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
        struct Dummy {
            request_id: RequestId,
        }
        let json = r#"{"request_id":42}"#;
        let dummy = Dummy { request_id: RequestId(42) };

        assert_eq!(serde_json::to_string(&dummy).unwrap(), json);
        assert_eq!(dummy, serde_json::from_str(json).unwrap());
    }
}
