use crate::{
    attr, command::parse_attrs, fields_parse::ParserType,
    rename_rules::RenameRule, Result,
};

#[derive(Debug)]
pub(crate) struct CommandEnum {
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub rename_rule: RenameRule,
    pub parser_type: ParserType,
}

impl CommandEnum {
    pub fn try_from(attrs: attr::CommandAttrs) -> Result<Self> {
        let attrs = parse_attrs(attrs)?;

        let prefix = attrs.prefix;
        let description = attrs.description;
        let rename = attrs.rename_rule;
        let separator = attrs.separator;
        let mut parser = attrs.parser.unwrap_or(ParserType::Default);
        if let (ParserType::Split { separator }, Some(s)) =
            (&mut parser, &separator)
        {
            *separator = Some(s.clone())
        }
        Ok(Self {
            prefix,
            description,
            rename_rule: rename,
            parser_type: parser,
        })
    }
}
