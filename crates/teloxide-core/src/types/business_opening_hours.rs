use serde::{Deserialize, Serialize};

use crate::types::BusinessOpeningHoursInterval;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessOpeningHours {
    /// Unique name of the time zone for which the opening hours are defined.
    pub time_zone_name: String,

    /// List of time intervals describing business opening hours.
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
