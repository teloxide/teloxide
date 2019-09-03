use reqwest::r#async::multipart::Form;
use std::fmt::Display;

/// Adds a key-value pair to the supplied form if `value` is some. Don't forget to implement `std::fmt::Display` for `T`!
pub fn add_to_form_if_some<T>(form: Form, name: &str, value: Option<T>) -> Form
where
    T: Display,
{
    value.map_or_else(
        || form,
        |value| form.text(name.to_owned(), format!("{}", value)),
    )
}
