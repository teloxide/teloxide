use serde::Deserialize;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct AnswerPreCheckoutQuery {
    pub pre_checkout_query_id: String,
    pub ok: bool,
    pub error_message: Option<String>,
}
