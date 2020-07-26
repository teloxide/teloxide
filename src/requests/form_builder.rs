use std::{borrow::Cow, path::PathBuf};

use reqwest::multipart::Form;

use crate::{
    requests::utils::{file_from_memory_to_part, file_to_part},
    types::{
        ChatId, InlineKeyboardMarkup, InputFile, InputMedia, MaskPosition, ParseMode, ReplyMarkup,
    },
};

/// This is a convenient struct that builds `reqwest::multipart::Form`
/// from scratch.
pub(crate) struct FormBuilder {
    form: Form,
}

impl FormBuilder {
    pub(crate) fn new() -> Self {
        Self { form: Form::new() }
    }

    pub fn add_text<'a, T, N>(self, name: N, value: &T) -> Self
    where
        N: Into<Cow<'a, str>>,
        T: IntoFormText,
    {
        match value.into_form_text() {
            Some(val) => Self { form: self.form.text(name.into().into_owned(), val) },
            None => self,
        }
    }

    pub async fn add_input_file<'a, N>(self, name: N, value: &InputFile) -> tokio::io::Result<Self>
    where
        N: Into<Cow<'a, str>>,
    {
        Ok(match value {
            InputFile::File(path) => self.add_file(name, path.clone()).await?,
            InputFile::Memory { file_name, data } => {
                self.add_file_from_memory(name, file_name.clone(), data.clone())
            }
            InputFile::Url(url) => self.add_text(name, url),
            InputFile::FileId(file_id) => self.add_text(name, file_id),
        })
    }

    // used in SendMediaGroup
    pub async fn add_file<'a, N>(self, name: N, path_to_file: PathBuf) -> tokio::io::Result<Self>
    where
        N: Into<Cow<'a, str>>,
    {
        Ok(Self {
            form: self.form.part(name.into().into_owned(), file_to_part(path_to_file).await?),
        })
    }

    fn add_file_from_memory<'a, N>(
        self,
        name: N,
        file_name: String,
        data: Cow<'static, [u8]>,
    ) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        Self {
            form: self
                .form
                .part(name.into().into_owned(), file_from_memory_to_part(data, file_name)),
        }
    }

    pub fn build(self) -> Form {
        self.form
    }
}

pub(crate) trait IntoFormText {
    fn into_form_text(&self) -> Option<String>;
}

macro_rules! impl_for_struct {
    ($($name:ty),*) => {
        $(
            impl IntoFormText for $name {
                fn into_form_text(&self) -> Option<String> {
                    let json = serde_json::to_string(self)
                        .expect("serde_json::to_string failed");
                    Some(json)
                }
            }
        )*
    };
}

impl_for_struct!(bool, i32, i64, u32, ReplyMarkup, InlineKeyboardMarkup, MaskPosition);

impl<T> IntoFormText for Option<T>
where
    T: IntoFormText,
{
    fn into_form_text(&self) -> Option<String> {
        self.as_ref().and_then(IntoFormText::into_form_text)
    }
}

// TODO: fix InputMedia implementation of IntoFormValue (for now it doesn't
// encode files :|)
impl IntoFormText for Vec<InputMedia> {
    fn into_form_text(&self) -> Option<String> {
        let json = serde_json::to_string(self).expect("serde_json::to_string failed");
        Some(json)
    }
}

impl IntoFormText for InputMedia {
    fn into_form_text(&self) -> Option<String> {
        let json = serde_json::to_string(self).expect("serde_json::to_string failed");
        Some(json)
    }
}

impl IntoFormText for str {
    fn into_form_text(&self) -> Option<String> {
        Some(self.to_owned())
    }
}

impl IntoFormText for ParseMode {
    fn into_form_text(&self) -> Option<String> {
        let string = match self {
            ParseMode::MarkdownV2 => String::from("MarkdownV2"),
            ParseMode::HTML => String::from("HTML"),
            #[allow(deprecated)]
            ParseMode::Markdown => String::from("Markdown"),
        };
        Some(string)
    }
}

impl IntoFormText for ChatId {
    fn into_form_text(&self) -> Option<String> {
        let string = match self {
            ChatId::Id(id) => id.to_string(),
            ChatId::ChannelUsername(username) => username.clone(),
        };
        Some(string)
    }
}

impl IntoFormText for String {
    fn into_form_text(&self) -> Option<String> {
        Some(self.clone())
    }
}
