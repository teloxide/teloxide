use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessOpeningHoursInterval {
    /// The minute's sequence number in a week, starting on Monday, marking the
    /// start of the time interval during which the business is open;
    /// 0 - 7 * 24* 60
    pub opening_minute: u16,

    /// The minute's sequence number in a week, starting on Monday, marking the
    /// end of the time interval during which the business is open;
    /// 0 - 8 * 24* 60
    pub closing_minute: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessOpeningHours {
    /// Unique name of the time zone for which the opening hours are defined.
    pub time_zone_name: String,

    /// List of time intervals describing business opening hours.
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
