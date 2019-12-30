use serde::{Deserialize, Serialize};

use crate::types::{OrderInfo, User};

/// This object contains information about an incoming pre-checkout query.
///
/// [The official docs](https://core.telegram.org/bots/api#precheckoutquery).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier.
    pub id: String,

    /// User who sent the query.
    pub from: User,

    /// Three-letter ISO 4217 [currency] code.
    ///
    /// [currency]: https://core.telegram.org/bots/payments#supported-currencies
    pub currency: String,

    /// Total price in the _smallest units_ of the currency (integer, **not**
    /// float/double). For example, for a price of `US$ 1.45` pass `amount =
    /// 145`. See the exp parameter in [`currencies.json`], it shows the number
    /// of digits past the decimal point for each currency (2 for the
    /// majority of currencies).
    ///
    /// [`currencies.json`]: https://core.telegram.org/bots/payments/currencies.json
    pub total_amount: i32,

    /// Bot specified invoice payload.
    pub invoice_payload: String,

    /// Identifier of the shipping option chosen by the user.
    pub shipping_option_id: Option<String>,

    /// Order info provided by the user.
    pub order_info: Option<OrderInfo>,
}
