use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::TransactionPartner;

/// Contains a list of Telegram Star transactions.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct StarTransactions {
    /// The list of transactions.
    pub transactions: Vec<StarTransaction>,
}

/// Describes a Telegram Star transaction.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct StarTransaction {
    /// Unique identifier of the transaction. Coincides with the identifer of
    /// the original transaction for refund transactions. Coincides with
    /// [`SuccessfulPayment::telegram_payment_charge_id`] for successful
    /// incoming payments from users.
    ///
    /// [`SuccessfulPayment::telegram_payment_charge_id`]: crate::types::SuccessfulPayment::telegram_payment_charge_id
    pub id: String,

    /// Number of Telegram Stars transferred by the transaction.
    pub amount: u32,

    /// Date the transaction was created in Unix time.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub date: DateTime<Utc>,

    /// Source of an incoming transaction (e.g., a user purchasing goods or
    /// services, Fragment refunding a failed withdrawal). Only for incoming
    /// transactions.
    pub source: Option<TransactionPartner>,

    /// Receiver of an outgoing transaction (e.g., a user for a purchase refund,
    /// Fragment for a withdrawal). Only for outgoing transactions.
    pub receiver: Option<TransactionPartner>,
}
