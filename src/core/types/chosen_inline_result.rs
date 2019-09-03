use serde::Deserialize;

use crate::core::types::user::User;

#[derive(Debug, Deserealize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}
