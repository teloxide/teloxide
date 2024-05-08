//! Credentials.

use serde::{Deserialize, Serialize};

/// Credentials type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Credentials {
    /// Type.
    pub r#type: CredentialsType,
    /// Credit card single-use token value or base64-encoded paymentData
    /// value.
    pub parameters: String,
}

/// Credentials parameters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Parameters {
    Token(String),
    Saved {
        /// Credentials id.
        id: String,
        /// Payment method title.
        title: String,
    },
}

/// Credential type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CredentialsType {
    /// Card.
    Card,
    /// Apple pay.
    ApplePay,
    /// Google pay.
    GooglePay,
    /// Saved.
    Saved,
}
