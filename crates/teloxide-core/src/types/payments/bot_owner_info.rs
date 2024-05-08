//! Bot owner info.
use serde::{Deserialize, Serialize};

/// Bot owner info 32bits.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OwnerInfo32 {
    /// Bot id.
    pub bot_id: u32,
    /// Owner id.
    pub bot_owner_id: u32,
}

/// Bot owner info 64 bits.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OwnerInfo64 {
    /// Bot id.
    pub bot_id: u64,
    /// Owner id.
    pub bot_owner_id: u64,
}

/// Bot owner info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OwnerInfo {
    /// Bot owner info 32 bits.
    OwnerInfo32(OwnerInfo32),
    /// Bot owner info 64 bits.
    OwnerInfo64(OwnerInfo64),
}

impl OwnerInfo {
    /// Get bot id.
    pub fn get_bot_id(&self) -> String {
        match self {
            OwnerInfo::OwnerInfo32(info) => info.bot_id.to_string(),
            OwnerInfo::OwnerInfo64(info) => info.bot_id.to_string(),
        }
    }

    /// Get owner id.
    pub fn get_owner_id(&self) -> String {
        match self {
            OwnerInfo::OwnerInfo32(info) => info.bot_owner_id.to_string(),
            OwnerInfo::OwnerInfo64(info) => info.bot_owner_id.to_string(),
        }
    }
}
