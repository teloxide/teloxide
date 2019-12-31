use std::{borrow::Cow, path::PathBuf};

use reqwest::multipart::Form;

use crate::{
    requests::utils::file_to_part,
    types::{
        ChatId, InlineKeyboardMarkup, InputFile, InputMedia, MaskPosition,
        ParseMode, ReplyMarkup,
    },
};

/// This is a convenient struct that builds `reqwest::multipart::Form`
/// from scratch.
pub(crate) struct FormBuilder {
    form: Form,
}

impl FormBuilder {
    pub fn new() -> Self {
        Self { form: Form::new() }
    }

    /// Add the supplied key-value pair to this `FormBuilder`.
    pub fn add<'a, T, N>(self, name: N, value: &T) -> Self
    where
        N: Into<Cow<'a, str>>,
        T: IntoFormValue,
    {
        let name = name.into().into_owned();
        match value.into_form_value() {
            Some(FormValue::Str(string)) => Self {
                form: self.form.text(name, string),
            },
            Some(FormValue::File(path)) => self.add_file(name, path),
            None => self,
        }
    }

    // used in SendMediaGroup
    pub fn add_file<'a, N>(self, name: N, path_to_file: PathBuf) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        Self {
            form: self
                .form
                .part(name.into().into_owned(), file_to_part(path_to_file)),
        }
    }

    pub fn build(self) -> Form {
        self.form
    }
}

pub(crate) enum FormValue {
    File(PathBuf),
    Str(String),
}

pub(crate) trait IntoFormValue {
    fn into_form_value(&self) -> Option<FormValue>;
}

macro_rules! impl_for_struct {
    ($($name:ty),*) => {
        $(
            impl IntoFormValue for $name {
                fn into_form_value(&self) -> Option<FormValue> {
                    let json = serde_json::to_string(self)
                        .expect("serde_json::to_string failed");
                    Some(FormValue::Str(json))
                }
            }
        )*
    };
}

impl_for_struct!(
    bool,
    i32,
    i64,
    u32,
    ReplyMarkup,
    InlineKeyboardMarkup,
    MaskPosition
);

impl<T> IntoFormValue for Option<T>
where
    T: IntoFormValue,
{
    fn into_form_value(&self) -> Option<FormValue> {
        self.as_ref().and_then(IntoFormValue::into_form_value)
    }
}

// TODO: fix InputMedia implementation of IntoFormValue (for now it doesn't
// encode files :|)
impl IntoFormValue for Vec<InputMedia> {
    fn into_form_value(&self) -> Option<FormValue> {
        let json =
            serde_json::to_string(self).expect("serde_json::to_string failed");
        Some(FormValue::Str(json))
    }
}

impl IntoFormValue for InputMedia {
    fn into_form_value(&self) -> Option<FormValue> {
        let json =
            serde_json::to_string(self).expect("serde_json::to_string failed");
        Some(FormValue::Str(json))
    }
}

impl IntoFormValue for str {
    fn into_form_value(&self) -> Option<FormValue> {
        Some(FormValue::Str(self.to_owned()))
    }
}

impl IntoFormValue for ParseMode {
    fn into_form_value(&self) -> Option<FormValue> {
        let string = match self {
            ParseMode::MarkdownV2 => String::from("MarkdownV2"),
            ParseMode::HTML => String::from("HTML"),
            #[allow(deprecated)]
            ParseMode::Markdown => String::from("Markdown"),
        };
        Some(FormValue::Str(string))
    }
}

impl IntoFormValue for ChatId {
    fn into_form_value(&self) -> Option<FormValue> {
        let string = match self {
            ChatId::Id(id) => id.to_string(),
            ChatId::ChannelUsername(username) => username.clone(),
        };
        Some(FormValue::Str(string))
    }
}

impl IntoFormValue for String {
    fn into_form_value(&self) -> Option<FormValue> {
        Some(FormValue::Str(self.clone()))
    }
}

impl IntoFormValue for InputFile {
    fn into_form_value(&self) -> Option<FormValue> {
        match self {
            InputFile::File(path) => Some(FormValue::File(path.clone())),
            InputFile::Url(url) => Some(FormValue::Str(url.clone())),
            InputFile::FileId(file_id) => Some(FormValue::Str(file_id.clone())),
        }
    }
}
