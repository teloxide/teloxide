use crate::{
    command_attr::CommandAttrs, error::compile_error_at, fields_parse::ParserType,
    rename_rules::RenameRule, Result,
};

pub(crate) struct CommandEnum {
    pub prefix: String,
    /// The bool is true if the description contains a doc comment
    pub description: Option<(String, bool)>,
    pub command_separator: String,
    pub rename_rule: RenameRule,
    pub parser_type: ParserType,
}

impl CommandEnum {
    pub fn from_attributes(attributes: &[syn::Attribute]) -> Result<Self> {
        let attrs = CommandAttrs::from_attributes(attributes)?;
        let CommandAttrs {
            prefix,
            description,
            rename_rule,
            rename,
            parser,
            separator,
            command_separator,
            hide,
        } = attrs;

        if let Some((_rename, sp)) = rename {
            return Err(compile_error_at(
                "`rename` attribute can only be applied to enums *variants*",
                sp,
            ));
        } else if let Some((_hide, sp)) = hide {
            return Err(compile_error_at(
                "`hide` attribute can only be applied to enums *variants*",
                sp,
            ));
        }

        let mut parser = parser.map(|(p, _)| p).unwrap_or(ParserType::Default);

        // FIXME: Error on unused separator
        if let (ParserType::Split { separator }, Some((s, _))) = (&mut parser, &separator) {
            *separator = Some(s.clone())
        }

        Ok(Self {
            prefix: prefix.map(|(p, _)| p).unwrap_or_else(|| "/".to_owned()),
            description: description.map(|(d, is_doc, _)| (d, is_doc)),
            command_separator: command_separator
                .map(|(s, _)| s)
                .unwrap_or_else(|| String::from(" ")),
            rename_rule: rename_rule.map(|(rr, _)| rr).unwrap_or(RenameRule::Identity),
            parser_type: parser,
        })
    }
}
