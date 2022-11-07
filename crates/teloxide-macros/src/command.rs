use crate::{
    command_attr::CommandAttrs, command_enum::CommandEnum, error::compile_error_at,
    fields_parse::ParserType, Result,
};

pub(crate) struct Command {
    /// Prefix of this command, for example "/".
    pub prefix: String,
    /// Description for the command.
    pub description: Option<String>,
    /// Name of the command, with all renames already applied.
    pub name: String,
    /// Parser for arguments of this command.
    pub parser: ParserType,
}

impl Command {
    pub fn new(
        name: &str,
        attributes: &[syn::Attribute],
        global_options: &CommandEnum,
    ) -> Result<Self> {
        let attrs = CommandAttrs::from_attributes(attributes)?;
        let CommandAttrs {
            prefix,
            description,
            rename_rule,
            rename,
            parser,
            // FIXME: error on/do not ignore separator
            separator: _,
        } = attrs;

        let name = match (rename, rename_rule) {
            (Some((rename, _)), None) => rename,
            (Some(_), Some((_, sp))) => {
                return Err(compile_error_at(
                    "`rename_rule` can't be applied to `rename`-d variant",
                    sp,
                ))
            }
            (None, Some((rule, _))) => rule.apply(name),
            (None, None) => global_options.rename_rule.apply(name),
        };

        let prefix = prefix.map(|(p, _)| p).unwrap_or_else(|| global_options.prefix.clone());
        let description = description.map(|(d, _)| d);
        let parser = parser.map(|(p, _)| p).unwrap_or_else(|| global_options.parser_type.clone());

        Ok(Self { prefix, description, parser, name })
    }

    pub fn get_prefixed_command(&self) -> String {
        let Self { prefix, name, .. } = self;
        format!("{prefix}{name}")
    }

    pub(crate) fn description_is_enabled(&self) -> bool {
        self.description != Some("off".to_owned())
    }
}
