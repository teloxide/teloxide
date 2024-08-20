use serde::{Deserialize, Serialize};

use crate::types::Location;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessLocation {
    /// Address of the business.
    pub address: String,

    /// Location of the business.
    pub location: Option<Location>,
}
