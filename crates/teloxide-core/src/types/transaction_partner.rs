use serde::{Deserialize, Serialize};

use crate::types::{RevenueWithdrawalState, User};

/// This object describes the source of a transaction, or its recipient for
/// outgoing transactions.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum TransactionPartner {
    Fragment(TransactionPartnerFragment),
    User(TransactionPartnerUser),
    Other,
}

/// Describes a withdrawal transaction with Fragment.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TransactionPartnerFragment {
    /// State of the transaction if the transaction is outgoing.
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}

/// Describes a transaction with a user.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TransactionPartnerUser {
    /// Information about the user.
    pub user: User,
}
