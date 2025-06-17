use serde::{Deserialize, Serialize};

use crate::types::{Gift, PaidMedia, RevenueWithdrawalState, Seconds, User};

/// This object describes the source of a transaction, or its recipient for
/// outgoing transactions.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum TransactionPartner {
    Fragment(TransactionPartnerFragment),
    User(Box<TransactionPartnerUser>),
    TelegramAds,
    TelegramApi(TransactionPartnerTelegramApi),
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

    /// Bot-specified invoice payload.
    pub invoice_payload: Option<String>,

    /// The duration of the paid subscription.
    pub subscription_period: Option<Seconds>,

    /// Information about the paid media bought by the user.
    pub paid_media: Option<Vec<PaidMedia>>,

    /// Bot-specified paid media payload
    pub paid_media_payload: Option<String>,

    /// The gift sent to the user by the bot
    pub gift: Option<Gift>,
}

/// Describes a transaction with payment for paid broadcasting.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TransactionPartnerTelegramApi {
    /// The number of successful requests that exceeded regular limits and were
    /// therefore billed
    pub request_count: u32,
}
