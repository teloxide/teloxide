use serde::{Deserialize, Serialize};

use crate::types::Sticker;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessIntro {
    /// Title text of the business intro
    pub title: Option<String>,

    /// Message text of the business intro
    pub message: Option<String>,

    /// Sticker of the business intro
    pub sticker: Option<Sticker>,
}
