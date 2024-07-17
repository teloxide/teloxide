use serde::{Deserialize, Serialize};

/// This object represents a service message about the creation of a scheduled
/// giveaway. Currently holds no information.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct GiveawayCreated {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"{}"#;
        serde_json::from_str::<GiveawayCreated>(data).unwrap();
    }
}
