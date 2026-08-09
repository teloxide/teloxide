use syn::Fields;

use crate::{
    button_attr::ButtonAttrs,
    button_enum::ButtonEnum,
    error::{compile_error_at, compile_error_multiple},
    fields_parse::ParserType,
    Result,
};

pub(crate) struct Button {
    /// Callback data name, with all renames already applied.
    pub data_name: String,
    pub text: String,
    pub row: u8,
    pub url: Option<String>,
    pub login_url: Option<String>,
    pub webapp_url: Option<String>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub copy_text: Option<String>,
    pub game: Option<bool>,
    pub pay: Option<bool>,
    pub parser: ParserType,
}

impl Button {
    pub fn new(
        data_name: &str,
        current_row: u8,
        attributes: &[syn::Attribute],
        global_options: &ButtonEnum,
        fields: &Fields,
    ) -> Result<Self> {
        let attrs = ButtonAttrs::from_attributes(attributes)?;
        let ButtonAttrs {
            rename,
            text,
            row,
            url,
            login_url,
            webapp_url,
            switch_inline_query,
            switch_inline_query_current_chat,
            copy_text,
            game,
            pay,
            // FIXME: error on/do not ignore separator
            fields_separator: _,
        } = attrs;

        let binding_1 = game.map(|g| (g.0.to_string(), g.1)); // Type doesnt matter for this
        let binding_2 = pay.map(|p| (p.0.to_string(), p.1));

        let exclusive_fields = [
            url.as_ref(),
            login_url.as_ref(),
            webapp_url.as_ref(),
            switch_inline_query.as_ref(),
            switch_inline_query_current_chat.as_ref(),
            copy_text.as_ref(),
            binding_1.as_ref(),
            binding_2.as_ref(),
        ];

        let present: Vec<_> = exclusive_fields.iter().filter_map(|f| *f).collect();

        if present.len() > 1 {
            return Err(compile_error_multiple(
                "Exactly one of the exclusive fields must be provided",
                present.into_iter().map(|f| f.1).collect(),
            ));
        }

        if fields != &Fields::Unit && !present.is_empty() {
            return Err(compile_error_at(
                "This attribute cant exist on a variant with fields. Please remove all fields or \
                 this attribute",
                present.first().unwrap().1,
            ));
        }

        let data_name = match rename {
            Some((rename, _)) => rename,
            None => String::from(data_name),
        };

        let text = match text {
            Some((text, _)) => text,
            None => data_name.clone(),
        };

        let row = match row {
            Some((row, row_span)) => {
                if row < 1 {
                    return Err(compile_error_at("Row number cant be less than 1", row_span));
                }
                row
            }
            None => current_row,
        };

        if let Some(ref url) = url {
            if let Err(e) = reqwest::Url::parse(&url.0) {
                return Err(compile_error_at(&e.to_string(), url.1));
            }
        }

        if let Some(ref login_url) = login_url {
            if let Err(e) = reqwest::Url::parse(&login_url.0) {
                return Err(compile_error_at(&e.to_string(), login_url.1));
            }
        }

        if let Some(ref webapp_url) = webapp_url {
            if let Err(e) = reqwest::Url::parse(&webapp_url.0) {
                return Err(compile_error_at(&e.to_string(), webapp_url.1));
            }
        }

        let url = url.map(|x| x.0);
        let login_url = login_url.map(|x| x.0);
        let webapp_url = webapp_url.map(|x| x.0);
        let switch_inline_query = switch_inline_query.map(|x| x.0);
        let switch_inline_query_current_chat = switch_inline_query_current_chat.map(|x| x.0);
        let copy_text = copy_text.map(|x| x.0);
        let game = game.map(|x| x.0);
        let pay = pay.map(|x| x.0);

        Ok(Self {
            data_name,
            text,
            row,
            url,
            login_url,
            webapp_url,
            switch_inline_query,
            switch_inline_query_current_chat,
            copy_text,
            game,
            pay,
            parser: global_options.parser_type.clone(),
        })
    }
}
