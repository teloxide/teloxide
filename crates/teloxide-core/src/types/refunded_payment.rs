use serde::{Deserialize, Serialize};

/// This object contains basic information about a refunded payment.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefundedPayment {
    /// Three-letter ISO 4217 currency code, or “XTR” for payments in Telegram
    /// Stars. Currently, always “XTR”
    pub currency: String,

    /// Total refunded price in the smallest units of the currency (integer, not
    /// float/double). For example, for a price of US$ 1.45, total_amount =
    /// 145. See the exp parameter in currencies.json, it shows the number
    /// of digits past the decimal point for each currency (2 for the majority
    /// of currencies).
    pub total_amount: u32,

    /// Bot-specified invoice payload
    pub invoice_payload: String,

    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,

    /// Provider payment identifier
    pub provider_payment_charge_id: Option<String>,
}
