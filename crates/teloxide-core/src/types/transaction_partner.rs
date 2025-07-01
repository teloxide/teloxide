use serde::{Deserialize, Serialize};

use crate::types::{Chat, Gift, PaidMedia, RevenueWithdrawalState, Seconds, User};

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
    Chat(Box<TransactionPartnerChat>),
    AffiliateProgram(TransactionPartnerAffiliateProgram),
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

    /// Information about the affiliate that received a commission via this
    /// transaction
    pub affiliate: Option<AffiliateInfo>,

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

/// Describes a transaction with a chat.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TransactionPartnerChat {
    /// Information about the chat
    pub chat: Chat,

    /// The gift sent to the chat by the bot
    pub gift: Option<Gift>,
}

/// Contains information about the affiliate that received a commission via this
/// transaction.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct AffiliateInfo {
    /// The bot or the user that received an affiliate commission if it was
    /// received by a bot or a user
    affiliate_user: Option<User>,

    /// The chat that received an affiliate commission if it was received by a
    /// chat
    affiliate_chat: Option<Chat>,

    /// The number of Telegram Stars received by the affiliate for each 1000
    /// Telegram Stars received by the bot from referred users
    commission_per_mille: u32,

    /// Integer amount of Telegram Stars received by the affiliate from the
    /// transaction, rounded to 0
    amount: i32, // Can be negative for refunds

    /// The number of 1/1000000000 shares of Telegram Stars received by the
    /// affiliate
    nanostar_amount: Option<i32>, // Can be negative for refunds
}

/// Describes the affiliate program that issued the affiliate commission
/// received via this transaction.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TransactionPartnerAffiliateProgram {
    /// Information about the bot that sponsored the affiliate program
    pub sponsor_user: Option<User>,

    /// The number of Telegram Stars received by the bot for each 1000 Telegram
    /// Stars received by the affiliate program sponsor from referred users
    pub commission_per_mille: u32,
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
