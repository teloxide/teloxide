use crate::{attr::Attr, command::parse_attrs, fields_parse::ParserType};

#[derive(Debug)]
pub struct CommandEnum {
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub rename_rule: Option<String>,
    pub parser_type: ParserType,
}

impl CommandEnum {
    pub fn try_from(attrs: &[Attr]) -> Result<Self, String> {
        let attrs = parse_attrs(attrs)?;

        let prefix = attrs.prefix;
        let description = attrs.description;
        let rename = attrs.rename;
        let separator = attrs.separator;
        let mut parser = attrs.parser.unwrap_or(ParserType::Default);
        if let (ParserType::Split { separator }, Some(s)) =
            (&mut parser, &separator)
        {
            *separator = Some(s.clone())
        }
        if let Some(rename_rule) = &rename {
            match rename_rule.as_str() {
                "lowercase" => {}
                _ => return Err("disallowed value".to_owned()),
            }
        }
        Ok(Self {
            prefix,
            description,
            rename_rule: rename,
            parser_type: parser,
        })
    }
}
