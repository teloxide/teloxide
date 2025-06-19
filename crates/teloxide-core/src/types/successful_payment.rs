use chrono::{DateTime, Utc};
use derive_more::derive::From;
use serde::{Deserialize, Serialize};

use crate::types::OrderInfo;

/// A unique identifier of the Telegram transaction.
#[derive(Clone, Debug, derive_more::Display)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize, From)]
#[serde(transparent)]
#[from(&'static str, String)]
pub struct TelegramTransactionId(pub String);

/// This object contains basic information about a successful payment.
///
/// [The official docs](https://core.telegram.org/bots/api#successfulpayment).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code, see [more on currencies]. Pass
    /// `XTR` for payments in [Telegram Stars].
    ///
    /// [more on currencies]: https://core.telegram.org/bots/payments#supported-currencies
    /// [Telegram Stars]: https://t.me/BotNews/90
    pub currency: String,

    /// Total price in the smallest units of the currency (integer, not
    /// float/double). For example, for a price of `US$ 1.45` pass `amount =
    /// 145`. See the exp parameter in [`currencies.json`], it shows the
    /// number of digits past the decimal point for each currency (2 for
    /// the majority of currencies).
    ///
    /// [`currencies.json`]: https://core.telegram.org/bots/payments/currencies.json
    pub total_amount: u32,

    /// Bot specified invoice payload.
    pub invoice_payload: String,

    /// Expiration date of the subscription, in Unix time; for recurring
    /// payments only.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub subscription_expiration_date: Option<DateTime<Utc>>,

    /// True, if the payment is a recurring payment for a subscription.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_recurring: bool,

    /// True, if the payment is the first payment for a subscription.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_first_recurring: bool,

    /// Identifier of the shipping option chosen by the user.
    pub shipping_option_id: Option<String>,

    /// Order info provided by the user.
    #[serde(default)]
    pub order_info: OrderInfo,

    /// Telegram payment identifier.
    pub telegram_payment_charge_id: TelegramTransactionId,

    /// Provider payment identifier.
    pub provider_payment_charge_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser() {
        let telegram_payment_charge_id =
            S { telegram_payment_charge_id: TelegramTransactionId("id".to_owned()) };
        let json = r#"{"telegram_payment_charge_id":"id"}"#;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct S {
            telegram_payment_charge_id: TelegramTransactionId,
        }

        assert_eq!(serde_json::to_string(&telegram_payment_charge_id).unwrap(), json);
        assert_eq!(telegram_payment_charge_id, serde_json::from_str(json).unwrap());
    }
}
