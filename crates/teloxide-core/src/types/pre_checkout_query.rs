use derive_more::derive::From;
use serde::{Deserialize, Serialize};

use crate::types::{OrderInfo, User};

/// Unique query identifier.
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From
)]
#[serde(transparent)]
#[from(&'static str)]
pub struct PreCheckoutQueryId(pub String);

/// This object contains information about an incoming pre-checkout query.
///
/// [The official docs](https://core.telegram.org/bots/api#precheckoutquery).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier.
    pub id: PreCheckoutQueryId,

    /// User who sent the query.
    pub from: User,

    /// Three-letter ISO 4217 currency code, see [more on currencies]. Pass
    /// `XTR` for payments in [Telegram Stars].
    ///
    /// [more on currencies]: https://core.telegram.org/bots/payments#supported-currencies
    /// [Telegram Stars]: https://t.me/BotNews/90
    pub currency: String,

    /// Total price in the _smallest units_ of the currency (integer, **not**
    /// float/double). For example, for a price of `US$ 1.45` pass `amount =
    /// 145`. See the exp parameter in [`currencies.json`], it shows the number
    /// of digits past the decimal point for each currency (2 for the
    /// majority of currencies).
    ///
    /// [`currencies.json`]: https://core.telegram.org/bots/payments/currencies.json
    pub total_amount: u32,

    /// Bot specified invoice payload.
    pub invoice_payload: String,

    /// Identifier of the shipping option chosen by the user.
    pub shipping_option_id: Option<String>,

    /// Order info provided by the user.
    #[serde(default)]
    pub order_info: OrderInfo,
}
