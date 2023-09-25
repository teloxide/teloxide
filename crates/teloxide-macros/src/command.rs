use proc_macro2::Span;

use crate::{
    command_attr::CommandAttrs, command_enum::CommandEnum, error::compile_error_at,
    fields_parse::ParserType, Result,
};

pub(crate) struct Command {
    /// Prefix of this command, for example "/".
    pub prefix: String,
    /// Description for the command.
    /// The bool is true if the description contains a doc comment.
    pub description: Option<(String, bool, Span)>,
    /// Name of the command, with all renames already applied.
    pub name: String,
    /// The aliases of the command.
    pub aliases: Option<(Vec<String>, Span)>,
    /// Parser for arguments of this command.
    pub parser: ParserType,
    /// Whether the command is hidden from the help message.
    pub hidden: bool,
    /// Whether the aliases of the command are hidden from the help message.
    pub hidden_aliases: bool,
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
            aliases,
            parser,
            // FIXME: error on/do not ignore separator
            separator: _,
            // FIXME: error on/do not ignore command separator
            command_separator: _,
            hide,
            hide_aliases,
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
        let hidden_aliases = hide_aliases.is_some();

        Ok(Self { prefix, description, parser, name, aliases, hidden, hidden_aliases })
    }

    pub fn get_prefixed_command(&self) -> String {
        let Self { prefix, name, .. } = self;
        format!("{prefix}{name}")
    }

    pub fn get_prefixed_aliases(&self) -> Option<Vec<String>> {
        let Self { prefix, aliases, .. } = self;
        aliases
            .as_ref()
            .map(|(aliases, _)| aliases.iter().map(|alias| format!("{prefix}{alias}")).collect())
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|(d, ..)| &**d)
    }

    pub fn contains_doc_comment(&self) -> bool {
        self.description.as_ref().map(|(_, is_doc, ..)| *is_doc).unwrap_or(false)
    }

    pub(crate) fn description_is_enabled(&self) -> bool {
        // FIXME: remove the first, `== "off"`, check eventually
        !((self.description() == Some("off") && !self.contains_doc_comment()) || self.hidden)
    }

    pub(crate) fn deprecated_description_off_span(&self) -> Option<Span> {
        self.description
            .as_ref()
            .filter(|(d, ..)| d == "off" && !self.contains_doc_comment())
            .map(|&(.., span)| span)
    }
}
