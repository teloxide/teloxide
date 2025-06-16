use serde::{Deserialize, Serialize};

/// This object represents a service message about the creation of a scheduled
/// giveaway.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct GiveawayCreated {
    /// The number of Telegram Stars to be split between giveaway winners; for
    /// Telegram Star giveaways only
    pub prize_star_count: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"{}"#;
        serde_json::from_str::<GiveawayCreated>(data).unwrap();
    }
}
