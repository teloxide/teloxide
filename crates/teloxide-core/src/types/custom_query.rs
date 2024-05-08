use serde::{Deserialize, Serialize};

use crate::types::{CreateCharge, PaymentsForm};

use super::DeleteSavedCredentials;

/// This object represents a custom query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomQuery {
    /// Unique custom query identifier.
    pub id: String,

    /// Custom query method.
    pub method: CustomQueryMethod,

    /// Custom query kind.
    #[serde(flatten)]
    pub kind: CustomQueryKind,
}

/// Custom query methods.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CustomQueryMethod {
    #[serde(rename = "payments.form")]
    PaymentsForm,
    #[serde(rename = "payments.create_charge")]
    PaymentsCreateCharge,
    #[serde(rename = "payments.create_charge_and_save")]
    PaymentsCreateChargeAndSave,
    #[serde(rename = "payments.create_charge_from_saved")]
    PaymentsCreateChargeFromSaved,
    #[serde(rename = "payments.delete_saved_credentials")]
    DeleteSavedCredentials,
}

/// Supported custom query kinds.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomQueryKind {
    PaymentsForm(PaymentsForm),
    CreateCharge(CreateCharge),
    DeleteSavedCredentials(DeleteSavedCredentials),
}
