use crate::{
    button_attr::ButtonAttrs, button_enum::ButtonEnum, error::compile_error_at,
    fields_parse::ParserType, Result,
};

pub(crate) struct Button {
    /// Callback data name, with all renames already applied.
    pub data_name: String,
    pub text: String,
    pub row: u8,
    pub url: Option<String>,
    pub parser: ParserType,
}

impl Button {
    pub fn new(
        data_name: &str,
        current_row: u8,
        attributes: &[syn::Attribute],
        global_options: &ButtonEnum,
    ) -> Result<Self> {
        let attrs = ButtonAttrs::from_attributes(attributes)?;
        let ButtonAttrs {
            rename,
            text,
            row,
            url,
            // FIXME: error on/do not ignore separator
            fields_separator: _,
        } = attrs;

        let data_name = match rename {
            Some((rename, _)) => rename,
            None => String::from(data_name),
        };

        let text = match text {
            Some((text, _)) => text,
            None => data_name.clone(),
        };

        let row = match row {
            Some((row, _)) => row,
            None => current_row,
        };

        if let Some(ref url) = url {
            if let Err(e) = reqwest::Url::parse(&url.0) {
                return Err(compile_error_at(&e.to_string(), url.1));
            }
        }

        let url = url.map(|x| x.0);

        Ok(Self { data_name, text, row, url, parser: global_options.parser_type.clone() })
    }
}
