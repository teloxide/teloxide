use serde::{Deserialize, Serialize};

use crate::types::LabeledPrice;

/// This object represents one shipping option.
///
/// [The official docs](https://core.telegram.org/bots/api#shippingoption).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ShippingOption {
    /// Shipping option identifier.
    pub id: String,

    /// Option title.
    pub title: String,

    /// List of price portions.
    pub prices: Vec<LabeledPrice>,
}

impl ShippingOption {
    pub fn new<S1, S2, P>(id: S1, title: S2, prices: P) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        P: IntoIterator<Item = LabeledPrice>,
    {
        Self { id: id.into(), title: title.into(), prices: prices.into_iter().collect() }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn prices<P>(mut self, val: P) -> Self
    where
        P: IntoIterator<Item = LabeledPrice>,
    {
        self.prices = val.into_iter().collect();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let shipping_option = ShippingOption {
            id: "0".to_string(),
            title: "Option".to_string(),
            prices: vec![LabeledPrice { label: "Label".to_string(), amount: 60 }],
        };
        let expected = r#"{"id":"0","title":"Option","prices":[{"label":"Label","amount":60}]}"#;
        let actual = serde_json::to_string(&shipping_option).unwrap();
        assert_eq!(actual, expected);
    }
}
