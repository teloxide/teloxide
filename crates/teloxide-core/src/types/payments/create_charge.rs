//! Payments create charge webhook payload.

use serde::{Deserialize, Serialize};

use crate::types::{Credentials, OwnerInfo};

/// This object represents a payments.create_charge handled webhook.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCharge {
    /// Customer id.
    pub customer_id: i64,

    /// Owner info.
    #[serde(flatten)]
    pub owner_info: OwnerInfo,

    /// Merchant bot connected account.
    pub bot_account: String,

    /// Merchant bot username.
    pub bot_username: String,

    /// Optional CUSTOMER information requested by merchant bot for
    /// provider (e.g. for sending reciept). Can have email and phone fields.
    pub customer_info: Option<CustomerInfo>,

    /// Invoice ID in the Telegram API, should be stored together with the
    /// transaction info for future reference, format [A-Za-z0-9/=_-]{5,64}
    pub charge_id: String,

    /// Invoice payload by MERCHANT_BOT, should be stored together with the
    /// transaction info for future reference.
    pub payload: String,

    /// Optional data from merchant for the invoice in generic format,
    /// specified by provider, up to 4KB in serialized JSON.
    pub data: Option<serde_json::Value>,

    /// Currency.
    pub currency: String,

    /// Invoice amount in minimum currency item.
    pub total_amount: i64,

    /// Credentials type.
    pub credentials: Credentials,

    /// Value to be passed into verifyPaymentCharge method after completing 3D
    /// Secure etc.
    pub callback: String,
}

/// Customer info.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerInfo {
    /// Customer phone.
    pub phone: Option<String>,
    /// Customer email.
    pub email: Option<String>,
}
