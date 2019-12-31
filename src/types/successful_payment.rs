use serde::{Deserialize, Serialize};

use crate::types::OrderInfo;

/// This object contains basic information about a successful payment.
///
/// [The official docs](https://core.telegram.org/bots/api#successfulpayment).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 [currency] code.
    ///
    /// [currency]: https://core.telegram.org/bots/payments#supported-currencies
    pub currency: String,

    /// Total price in the smallest units of the currency (integer, not
    /// float/double). For example, for a price of `US$ 1.45` pass `amount =
    /// 145`. See the exp parameter in [`currencies.json`], it shows the
    /// number of digits past the decimal point for each currency (2 for
    /// the majority of currencies).
    ///
    /// [`currencies.json`]: https://core.telegram.org/bots/payments/currencies.json
    pub total_amount: i32,

    /// Bot specified invoice payload.
    pub invoice_payload: String,

    /// Identifier of the shipping option chosen by the user.
    pub shipping_option_id: Option<String>,

    /// Order info provided by the user.
    pub order_info: Option<OrderInfo>,

    /// Telegram payment identifier.
    pub telegram_payment_charge_id: String,

    /// Provider payment identifier.
    pub provider_payment_charge_id: String,
}
