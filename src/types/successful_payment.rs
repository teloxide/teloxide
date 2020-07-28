use serde::{Deserialize, Serialize};

use crate::types::{Currency, OrderInfo};

/// This object contains basic information about a successful payment.
///
/// [The official docs](https://core.telegram.org/bots/api#successfulpayment).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 [currency] code.
    ///
    /// [currency]: https://core.telegram.org/bots/payments#supported-currencies
    pub currency: Currency,

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

impl SuccessfulPayment {
    pub fn new<S1, S2, S3>(
        currency: Currency,
        total_amount: i32,
        invoice_payload: S1,
        telegram_payment_charge_id: S2,
        provider_payment_charge_id: S3,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Self {
            currency,
            total_amount,
            invoice_payload: invoice_payload.into(),
            shipping_option_id: None,
            order_info: None,
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            provider_payment_charge_id: provider_payment_charge_id.into(),
        }
    }

    pub fn currency<S>(mut self, val: Currency) -> Self {
        self.currency = val;
        self
    }

    pub fn total_amount(mut self, val: i32) -> Self {
        self.total_amount = val;
        self
    }

    pub fn invoice_payload<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.invoice_payload = val.into();
        self
    }

    pub fn shipping_option_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.shipping_option_id = Some(val.into());
        self
    }

    pub fn order_info(mut self, val: OrderInfo) -> Self {
        self.order_info = Some(val);
        self
    }

    pub fn telegram_payment_charge_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.telegram_payment_charge_id = val.into();
        self
    }

    pub fn provider_payment_charge_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.provider_payment_charge_id = val.into();
        self
    }
}
