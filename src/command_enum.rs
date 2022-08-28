use crate::{
    command_attr::CommandAttrs, fields_parse::ParserType,
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
    pub fn try_from(attrs: CommandAttrs) -> Result<Self> {
        let CommandAttrs {
            prefix,
            description,
            rename_rule,
            parser,
            separator,
        } = attrs;
        let mut parser = parser.unwrap_or(ParserType::Default);

        // FIXME: Error on unused separator
        if let (ParserType::Split { separator }, Some(s)) =
            (&mut parser, &separator)
        {
            *separator = Some(s.clone())
        }
        Ok(Self {
            prefix,
            description,
            rename_rule: rename_rule.unwrap_or(RenameRule::Identity),
            parser_type: parser,
        })
    }
}
