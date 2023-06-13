use proc_macro2::Span;

use crate::{
    command_attr::CommandAttrs, command_enum::CommandEnum, error::compile_error_at,
    fields_parse::ParserType, Result,
};

pub(crate) struct Command {
    /// Prefix of this command, for example "/".
    pub prefix: String,
    /// Description for the command.
    pub description: Option<(String, Span)>,
    /// Name of the command, with all renames already applied.
    pub name: String,
    /// Parser for arguments of this command.
    pub parser: ParserType,
    /// Whether the command is hidden from the help message.
    pub hidden: bool,
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
            hide,
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
        let parser = parser.map(|(p, _)| p).unwrap_or_else(|| global_options.parser_type.clone());
        let hidden = hide.is_some();

        Ok(Self { prefix, description, parser, name, hidden })
    }

    pub fn get_prefixed_command(&self) -> String {
        let Self { prefix, name, .. } = self;
        format!("{prefix}{name}")
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|(d, _span)| &**d)
    }

    pub(crate) fn description_is_enabled(&self) -> bool {
        // FIXME: remove the first, `== "off"`, check eventually
        self.description() != Some("off") && !self.hidden
    }

    pub(crate) fn deprecated_description_off_span(&self) -> Option<Span> {
        self.description.as_ref().filter(|(d, _)| d == "off").map(|&(_, span)| span)
    }
}
