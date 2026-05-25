use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{Deserialize, Serialize};

/// This object describes the state of a revenue withdrawal operation.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum RevenueWithdrawalState {
    Pending,
    Succeeded(RevenueWithdrawalStateSucceeded),
    Failed,
}

/// The withdrawal succeeded.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct RevenueWithdrawalStateSucceeded {
    /// Date the withdrawal was completed in Unix time.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub date: DateTime<Utc>,

    /// An HTTPS URL that can be used to see transaction details.
    pub url: Url,
}
