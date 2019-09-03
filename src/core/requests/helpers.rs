use reqwest::r#async::multipart::Form;
use serde::Serialize;

/// Adds a key-value pair to the supplied form if `value` is some. Don't forget
/// to implement `serde::Serialize` for `T`!
pub fn add_to_form_if_some<T>(form: Form, name: &str, value: Option<&T>) -> Form
where
    T: Serialize,
{
    value.map_or_else(
        || form,
        |value| {
            form.text(
                name.to_owned(),
                serde_json::to_string(value).expect("serde_json::to_string failed"),
            )
        },
    )
}
