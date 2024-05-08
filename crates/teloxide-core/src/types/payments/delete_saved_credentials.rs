//! Credentials.

use serde::{Deserialize, Serialize};

use crate::types::Credentials;

/// Credentials type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteSavedCredentials {
    /// Customer id.
    pub customer_id: String,
    /// Credentials.
    pub credentials: Credentials,
}
