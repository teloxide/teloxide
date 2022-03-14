use crate::{
    attr::{Attr, BotCommandAttribute},
    command_enum::CommandEnum,
    fields_parse::ParserType,
    rename_rules::rename_by_rule,
};

pub struct Command {
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub parser: Option<ParserType>,
    pub name: String,
    pub renamed: bool,
}

impl Command {
    pub fn try_from(attrs: &[Attr], name: &str) -> Result<Self, String> {
        let attrs = parse_attrs(attrs)?;
        let mut new_name = name.to_string();
        let mut renamed = false;

        let prefix = attrs.prefix;
        let description = attrs.description;
        let rename = attrs.rename;
        let parser = attrs.parser;
        if let Some(rename_rule) = rename {
            new_name = rename_by_rule(name, &rename_rule);
            renamed = true;
        }
        Ok(Self { prefix, description, parser, name: new_name, renamed })
    }

    pub fn get_matched_value(&self, global_parameters: &CommandEnum) -> String {
        let prefix = if let Some(prefix) = &self.prefix {
            prefix
        } else if let Some(prefix) = &global_parameters.prefix {
            prefix
        } else {
            "/"
        };
        if let Some(rule) = &global_parameters.rename_rule {
            String::from(prefix) + &rename_by_rule(&self.name, rule.as_str())
        } else {
            String::from(prefix) + &self.name
        }
    }

    pub fn get_matched_value2(
        &self,
        global_parameters: &CommandEnum,
    ) -> (String, String) {
        let prefix = if let Some(prefix) = &self.prefix {
            prefix
        } else if let Some(prefix) = &global_parameters.prefix {
            prefix
        } else {
            "/"
        };
        if let Some(rule) = &global_parameters.rename_rule {
            (String::from(prefix), rename_by_rule(&self.name, rule.as_str()))
        } else {
            (String::from(prefix), self.name.clone())
        }
    }
}

pub struct CommandAttrs {
    pub(crate) prefix: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) rename: Option<String>,
    pub(crate) parser: Option<ParserType>,
    pub(crate) separator: Option<String>,
}

pub fn parse_attrs(attrs: &[Attr]) -> Result<CommandAttrs, String> {
    let mut prefix = None;
    let mut description = None;
    let mut rename_rule = None;
    let mut parser = None;
    let mut separator = None;

    for attr in attrs {
        match attr.name() {
            BotCommandAttribute::Prefix => prefix = Some(attr.value()),
            BotCommandAttribute::Description => {
                description = Some(attr.value())
            }
            BotCommandAttribute::RenameRule => rename_rule = Some(attr.value()),
            BotCommandAttribute::CustomParser => {
                parser = Some(ParserType::parse(&attr.value()))
            }
            BotCommandAttribute::Separator => separator = Some(attr.value()),
        }
    }

    Ok(CommandAttrs {
        prefix,
        description,
        rename: rename_rule,
        parser,
        separator,
    })
}
