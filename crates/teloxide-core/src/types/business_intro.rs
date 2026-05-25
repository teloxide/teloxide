use serde::{Deserialize, Serialize};

use crate::types::Sticker;

/// An introduction for a Business.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BusinessIntro {
    /// Title text of the business intro
    pub title: Option<String>,

    /// Message text of the business intro
    pub message: Option<String>,

    /// Sticker of the business intro
    pub sticker: Option<Sticker>,
}
