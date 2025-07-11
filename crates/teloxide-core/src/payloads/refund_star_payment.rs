//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{TelegramTransactionId, True, UserId};

impl_payload! {
    /// Refunds a successful payment in [Telegram Stars]. Returns True on success.
    ///
    /// [Telegram Stars]: https://t.me/BotNews/90
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub RefundStarPayment (RefundStarPaymentSetters) => True {
        required {
            /// Identifier of the user whose payment will be refunded
            pub user_id: UserId,
            /// Telegram payment identifier
            pub telegram_payment_charge_id: TelegramTransactionId,
        }
    }
}
