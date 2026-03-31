use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{TelegramTransactionId, TransactionPartner};

/// Contains a list of Telegram Star transactions.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct StarTransactions {
    /// The list of transactions.
    pub transactions: Vec<StarTransaction>,
}

/// Describes a Telegram Star transaction.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct StarTransaction {
    /// Unique identifier of the transaction. Coincides with the identifer of
    /// the original transaction for refund transactions. Coincides with
    /// [`SuccessfulPayment::telegram_payment_charge_id`] for successful
    /// incoming payments from users.
    ///
    /// [`SuccessfulPayment::telegram_payment_charge_id`]: crate::types::SuccessfulPayment::telegram_payment_charge_id
    pub id: TelegramTransactionId,

    /// Number of Telegram Stars transferred by the transaction.
    pub amount: u32,

    /// The number of 1/1000000000 shares of Telegram Stars transferred by the
    /// transaction
    pub nanostar_amount: Option<u32>,

    /// Date the transaction was created in Unix time.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub date: DateTime<Utc>,

    /// Source of an incoming transaction (e.g., a user purchasing goods or
    /// services, Fragment refunding a failed withdrawal). Only for incoming
    /// transactions.
    pub source: Option<TransactionPartner>,

    /// Receiver of an outgoing transaction (e.g., a user for a purchase refund,
    /// Fragment for a withdrawal). Only for outgoing transactions.
    pub receiver: Option<TransactionPartner>,
}
