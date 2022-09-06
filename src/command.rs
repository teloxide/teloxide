use crate::{
    command_attr::CommandAttrs, command_enum::CommandEnum,
    fields_parse::ParserType, rename_rules::RenameRule, Result,
};

pub(crate) struct Command {
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub parser: Option<ParserType>,
    pub name: String,
}

impl Command {
    pub fn try_from(attrs: CommandAttrs, name: &str) -> Result<Self> {
        let CommandAttrs {
            prefix,
            description,
            rename_rule,
            parser,
            separator: _,
        } = attrs;

        let name = rename_rule.unwrap_or(RenameRule::Identity).apply(name);

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

    pub(crate) fn description_is_enabled(&self) -> bool {
        self.description != Some("off".to_owned())
    }
}
