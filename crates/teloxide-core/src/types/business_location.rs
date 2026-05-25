use serde::{Deserialize, Serialize};

use crate::types::Location;

/// Details about the location of a Business
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BusinessLocation {
    /// Address of the business.
    pub address: String,

    /// Location of the business.
    pub location: Option<Location>,
}
