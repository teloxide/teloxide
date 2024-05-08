//! Payments form webhook payload.

use serde::{Deserialize, Serialize};

use crate::types::OwnerInfo;

/// This object represents a payments.form handled webhook.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaymentsForm {
    /// Customer id.
    pub customer_id: i64,

    /// Optional Customer's IETF language tag.
    pub customer_language_code: Option<String>,

    /// Owner info.
    #[serde(flatten)]
    pub owner_info: OwnerInfo,

    /// Merchant bot connected account.
    pub bot_account: String,

    /// Merchant bot username.
    pub bot_username: String,

    /// Optional (if the CUSTOMER’s app supports it) color schema variables
    /// to be used in the form. Provider can use it to adapt design of credit
    /// card form to user’s current theme.
    pub theme_params: Option<ThemeParams>,
}

/// Theme params.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeParams {
    /// Accent text color.
    pub accent_text_color: String,
    /// Bg color.
    pub bg_color: String,
    /// Button color.
    pub button_color: String,
    /// Button text color.
    pub button_text_color: String,
    /// Destructive text color.
    pub destructive_text_color: String,
    /// Header bg color.
    pub header_bg_color: String,
    /// Hint color.
    pub hint_color: String,
    /// Link color.
    pub link_color: String,
    /// Secondary bg color.
    pub secondary_bg_color: String,
    /// Section bg color.
    pub section_bg_color: String,
    /// Section header text color.
    pub section_header_text_color: String,
    /// Subtitle text color.
    pub subtitle_text_color: String,
    /// Text color.
    pub text_color: String,
}
