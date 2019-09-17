use crate::core::types::{User, Location};

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct InlineQuery {
    pub id: i64,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}
