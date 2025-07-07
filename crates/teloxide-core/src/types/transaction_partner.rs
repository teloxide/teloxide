use std::ops::Deref;

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

    /// Kind of transaction partner user
    #[serde(flatten)]
    pub kind: TransactionPartnerUserKind,
}

/// Type of the transaction. `InvoicePayment` for payments via invoices,
/// `PaidMediaPayment` for payments for paid media, `GiftPurchase` for gifts
/// sent by the bot, `PremiumPurchase` for Telegram Premium subscriptions gifted
/// by the bot, `BusinessAccountTransfer` for direct transfers from managed
/// business accounts
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionPartnerUserType {
    InvoicePayment,
    PaidMediaPayment,
    GiftPurchase,
    PremiumPurchase,
    BusinessAccountTransfer,
}

/// Kind of the transaction. `InvoicePayment` for payments via invoices,
/// `PaidMediaPayment` for payments for paid media, `GiftPurchase` for gifts
/// sent by the bot, `PremiumPurchase` for Telegram Premium subscriptions gifted
/// by the bot, `BusinessAccountTransfer` for direct transfers from managed
/// business accounts
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "transaction_type", rename_all = "snake_case")]
pub enum TransactionPartnerUserKind {
    InvoicePayment(TransactionPartnerUserInvoicePayment),
    PaidMediaPayment(TransactionPartnerUserPaidMediaPayment),
    GiftPurchase(TransactionPartnerUserGiftPurchase),
    PremiumPurchase(TransactionPartnerUserPremiumPurchase),
    BusinessAccountTransfer,
}

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TransactionPartnerUserInvoicePayment {
    /// Information about the affiliate that received a commission via this
    /// transaction
    pub affiliate: Option<AffiliateInfo>,

    /// Bot-specified invoice payload.
    pub invoice_payload: Option<String>,

    /// The duration of the paid subscription.
    pub subscription_period: Option<Seconds>,
}

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TransactionPartnerUserPaidMediaPayment {
    /// Information about the affiliate that received a commission via this
    /// transaction
    pub affiliate: Option<AffiliateInfo>,

    /// Information about the paid media bought by the user.
    pub paid_media: Option<Vec<PaidMedia>>,

    /// Bot-specified paid media payload
    pub paid_media_payload: Option<String>,
}

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TransactionPartnerUserGiftPurchase {
    /// The gift sent to the user by the bot
    pub gift: Option<Gift>,
}

#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TransactionPartnerUserPremiumPurchase {
    /// Number of months the gifted Telegram Premium subscription will be active
    /// for
    pub premium_subscription_duration: Option<u32>,
}

/// This allows calling [`TransactionPartnerUserKind`]'s methods directly on
/// [`TransactionPartnerUser`].
///
/// ```no_run
/// use teloxide_core::types::TransactionPartnerUser;
///
/// let transaction: TransactionPartnerUser = todo!();
///
/// let _ = transaction.gift_purchase();
/// let _ = transaction.kind.gift_purchase();
///
/// let _ = transaction.transaction_type();
/// let _ = transaction.kind.transaction_type();
/// ```
impl Deref for TransactionPartnerUser {
    type Target = TransactionPartnerUserKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl TransactionPartnerUserKind {
    /// Converts [`TransactionPartnerUserKind`] to
    /// [`TransactionPartnerUserType`]
    #[must_use]
    pub fn transaction_type(&self) -> TransactionPartnerUserType {
        match self {
            TransactionPartnerUserKind::GiftPurchase(_) => TransactionPartnerUserType::GiftPurchase,
            TransactionPartnerUserKind::InvoicePayment(_) => {
                TransactionPartnerUserType::InvoicePayment
            }
            TransactionPartnerUserKind::PaidMediaPayment(_) => {
                TransactionPartnerUserType::PaidMediaPayment
            }
            TransactionPartnerUserKind::PremiumPurchase(_) => {
                TransactionPartnerUserType::PremiumPurchase
            }
            TransactionPartnerUserKind::BusinessAccountTransfer => {
                TransactionPartnerUserType::BusinessAccountTransfer
            }
        }
    }

    /// Getter for [`TransactionPartnerUserGiftPurchase`]
    pub fn gift_purchase(&self) -> Option<TransactionPartnerUserGiftPurchase> {
        match self {
            TransactionPartnerUserKind::GiftPurchase(t) => Some(t.clone()),
            _ => None,
        }
    }

    /// Getter for [`TransactionPartnerUserInvoicePayment`]
    pub fn invoice_payment(&self) -> Option<TransactionPartnerUserInvoicePayment> {
        match self {
            TransactionPartnerUserKind::InvoicePayment(t) => Some(t.clone()),
            _ => None,
        }
    }

    /// Getter for [`TransactionPartnerUserPaidMediaPayment`]
    pub fn paid_media_payment(&self) -> Option<TransactionPartnerUserPaidMediaPayment> {
        match self {
            TransactionPartnerUserKind::PaidMediaPayment(t) => Some(t.clone()),
            _ => None,
        }
    }

    /// Getter for [`TransactionPartnerUserPremiumPurchase`]
    pub fn premium_purchase(&self) -> Option<TransactionPartnerUserPremiumPurchase> {
        match self {
            TransactionPartnerUserKind::PremiumPurchase(t) => Some(t.clone()),
            _ => None,
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::types::{
        TransactionPartnerUser, TransactionPartnerUserKind, TransactionPartnerUserPremiumPurchase,
        User, UserId,
    };

    #[test]
    fn test_transaction_partner_user() {
        let transaction = TransactionPartnerUser {
            user: User {
                id: UserId(109_998_024),
                is_bot: false,
                first_name: String::from("Laster"),
                last_name: None,
                username: Some(String::from("laster_alex")),
                language_code: Some(String::from("en")),
                is_premium: false,
                added_to_attachment_menu: false,
            },
            kind: TransactionPartnerUserKind::PremiumPurchase(
                TransactionPartnerUserPremiumPurchase { premium_subscription_duration: Some(1) },
            ),
        };

        let json = r#"{
            "user": {
                "id": 109998024,
                "is_bot": false,
                "first_name": "Laster",
                "username": "laster_alex",
                "language_code": "en"
            },
            "transaction_type": "premium_purchase",
            "premium_subscription_duration": 1
        }"#;

        let json_transaction: TransactionPartnerUser = serde_json::from_str(json).unwrap();

        assert_eq!(json_transaction, transaction);
    }
}
