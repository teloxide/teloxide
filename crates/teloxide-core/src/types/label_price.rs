use serde::{Deserialize, Serialize};

/// This object represents a portion of the price for goods or services.
///
/// [The official docs](https://core.telegram.org/bots/api#labeledprice).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct LabeledPrice {
    /// Portion label.
    pub label: String,

    /// Price of the product in the smallest units of the [currency] (integer,
    /// **not** float/double). For example, for a price of `US$ 1.45` pass
    /// `amount = 145`. See the exp parameter in [`currencies.json`], it shows
    /// the number of digits past the decimal point for each currency (2
    /// for the majority of currencies).
    ///
    /// [currency]: https://core.telegram.org/bots/payments#supported-currencies
    /// [`currencies.json`]: https://core.telegram.org/bots/payments/currencies.json
    pub amount: u32,
}

impl LabeledPrice {
    pub fn new<S>(label: S, amount: u32) -> Self
    where
        S: Into<String>,
    {
        Self { label: label.into(), amount }
    }

    pub fn label<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.label = val.into();
        self
    }

    #[must_use]
    pub fn amount(mut self, val: u32) -> Self {
        self.amount = val;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let labeled_price = LabeledPrice { label: "Label".to_string(), amount: 60 };
        let expected = r#"{"label":"Label","amount":60}"#;
        let actual = serde_json::to_string(&labeled_price).unwrap();
        assert_eq!(actual, expected);
    }
}
