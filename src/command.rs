use crate::{
    attr::{Attr, BotCommandAttribute},
    command_enum::CommandEnum,
    fields_parse::ParserType,
    rename_rules::RenameRule,
    Result,
};

pub(crate) struct Command {
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub parser: Option<ParserType>,
    pub name: String,
}

impl Command {
    pub fn try_from(attrs: &[Attr], name: &str) -> Result<Self> {
        let attrs = parse_attrs(attrs)?;
        let CommandAttrs {
            prefix,
            description,
            rename_rule,
            parser,
            separator: _,
        } = attrs;

        let name = rename_rule.apply(name);

        Ok(Self { prefix, description, parser, name })
    }

    pub fn get_matched_value(&self, global_parameters: &CommandEnum) -> String {
        let prefix = if let Some(prefix) = &self.prefix {
            prefix
        } else if let Some(prefix) = &global_parameters.prefix {
            prefix
        } else {
            "/"
        };

        String::from(prefix) + &global_parameters.rename_rule.apply(&self.name)
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

        (String::from(prefix), global_parameters.rename_rule.apply(&self.name))
    }
}

pub(crate) struct CommandAttrs {
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub rename_rule: RenameRule,
    pub parser: Option<ParserType>,
    pub separator: Option<String>,
}

pub(crate) fn parse_attrs(attrs: &[Attr]) -> Result<CommandAttrs> {
    let mut prefix = None;
    let mut description = None;
    let mut rename_rule = RenameRule::Identity;
    let mut parser = None;
    let mut separator = None;

    for attr in attrs {
        match attr.name() {
            BotCommandAttribute::Prefix => prefix = Some(attr.value()),
            BotCommandAttribute::Description => {
                description = Some(attr.value())
            }
            BotCommandAttribute::RenameRule => {
                rename_rule = RenameRule::parse(&attr.value())?
            }
            BotCommandAttribute::CustomParser => {
                parser = Some(ParserType::parse(&attr.value()))
            }
            BotCommandAttribute::Separator => separator = Some(attr.value()),
        }
    }

    Ok(CommandAttrs { prefix, description, rename_rule, parser, separator })
}
