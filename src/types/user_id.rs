use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Identifier of a user.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct UserId(pub u64);

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::types::UserId;

    /// Test that `UserId` is serialized as the underlying integer
    #[test]
    fn deser() {
        let user_id = S {
            user_id: UserId(17),
        };
        let json = r#"{"user_id":17}"#;

        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
        struct S {
            user_id: UserId,
        }

        assert_eq!(serde_json::to_string(&user_id).unwrap(), json);
        assert_eq!(user_id, serde_json::from_str(json).unwrap());
    }
}
