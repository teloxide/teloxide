use std::path::PathBuf;

use crate::core::{
    requests::{utils, ChatId},
    types::{InputMedia, ParseMode},
};

use reqwest::r#async::multipart::Form;
use serde::Serialize;

/// This is a convenient struct that builds `reqwest::r#async::multipart::Form`
/// from scratch.
pub struct FormBuilder {
    form: Form,
}

impl FormBuilder {
    pub fn new() -> Self {
        Self { form: Form::new() }
    }

    /// Add the supplied key-value pair to this `FormBuilder`.
    pub fn add<T>(self, name: &str, value: &T) -> Self
    where
        T: ToFormValue + ?Sized,
    {
        Self {
            form: self.form.text(name.to_owned(), value.to_form_value()),
        }
    }

    /// Adds a key-value pair to the supplied `FormBuilder` if `value` is some.
    /// Don't forget to implement `serde::Serialize` for `T`!
    pub fn add_if_some<T>(self, name: &str, value: Option<&T>) -> Self
    where
        T: ToFormValue + ?Sized,
    {
        match value {
            None => Self { form: self.form },
            Some(value) => self.add(name, value),
        }
    }

    pub fn add_file(self, name: &str, path_to_file: &PathBuf) -> Self {
        Self {
            form: self
                .form
                .part(name.to_owned(), utils::file_to_part(path_to_file)),
        }
    }

    pub fn build(self) -> Form {
        self.form
    }
}

pub trait ToFormValue {
    fn to_form_value(&self) -> String;
}

macro_rules! impl_for_struct {
    ($($name:ty),*) => {
        $(
            impl ToFormValue for $name {
                fn to_form_value(&self) -> String {
                    serde_json::to_string(self).expect("serde_json::to_string failed")
                }
            }
        )*
    };
}

impl_for_struct!(bool, i32, i64, Vec<InputMedia>);

impl ToFormValue for str {
    fn to_form_value(&self) -> String {
        self.to_owned()
    }
}

impl ToFormValue for ParseMode {
    fn to_form_value(&self) -> String {
        match self {
            ParseMode::HTML => String::from("HTML"),
            ParseMode::Markdown => String::from("Markdown"),
        }
    }
}

impl ToFormValue for ChatId {
    fn to_form_value(&self) -> String {
        match self {
            ChatId::Id(id) => id.to_string(),
            ChatId::ChannelUsername(username) => username.clone(),
        }
    }
}

impl ToFormValue for String {
    fn to_form_value(&self) -> String {
        self.to_owned()
    }
}
