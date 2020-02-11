use crate::{
    attr::{Attr, BotCommandAttribute},
    rename_rules::rename_by_rule,
};

pub struct Command {
    pub prefix: Option<String>,
    pub description: Option<String>,
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
        if let Some(rename_rule) = rename {
            new_name = rename_by_rule(name, &rename_rule);
            renamed = true;
        }
        Ok(Self {
            prefix,
            description,
            name: new_name,
            renamed,
        })
    }
}

struct CommandAttrs {
    prefix: Option<String>,
    description: Option<String>,
    rename: Option<String>,
}

fn parse_attrs(attrs: &[Attr]) -> Result<CommandAttrs, String> {
    let mut prefix = None;
    let mut description = None;
    let mut rename_rule = None;

    for attr in attrs {
        match attr.name() {
            BotCommandAttribute::Prefix => prefix = Some(attr.value()),
            BotCommandAttribute::Description => {
                description = Some(attr.value())
            }
            BotCommandAttribute::RenameRule => rename_rule = Some(attr.value()),
            #[allow(unreachable_patterns)]
            _ => return Err("unexpected attribute".to_owned()),
        }
    }

    Ok(CommandAttrs {
        prefix,
        description,
        rename: rename_rule,
    })
}
