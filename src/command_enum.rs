use crate::{
    command_attr::CommandAttrs, fields_parse::ParserType,
    rename_rules::RenameRule, Result,
};

#[derive(Debug)]
pub(crate) struct CommandEnum {
    pub prefix: String,
    pub description: Option<String>,
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
            parser,
            separator,
        } = attrs;

        let mut parser = parser.map(|(p, _)| p).unwrap_or(ParserType::Default);

        // FIXME: Error on unused separator
        if let (ParserType::Split { separator }, Some((s, _))) =
            (&mut parser, &separator)
        {
            *separator = Some(s.clone())
        }

        Ok(Self {
            prefix: prefix.map(|(p, _)| p).unwrap_or_else(|| "/".to_owned()),
            description: description.map(|(d, _)| d),
            rename_rule: rename_rule
                .map(|(rr, _)| rr)
                .unwrap_or(RenameRule::Identity),
            parser_type: parser,
        })
    }
}
