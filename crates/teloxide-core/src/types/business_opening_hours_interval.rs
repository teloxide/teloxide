use serde::{Deserialize, Serialize};

/// Time intervals used to describe the opening hours of a Business.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
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
