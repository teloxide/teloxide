use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnswerPreCheckoutQuery {
    pre_checkout_query_id: String,
    ok: bool,
    error_message: Option<String>,
}
