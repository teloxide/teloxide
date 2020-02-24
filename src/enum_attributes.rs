use crate::attr::{Attr, BotCommandAttribute};

pub struct CommandEnum {
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub rename_rule: Option<String>,
}

impl CommandEnum {
    pub fn try_from(attrs: &[Attr]) -> Result<Self, String> {
        let attrs = parse_attrs(attrs)?;

        let prefix = attrs.prefix;
        let description = attrs.description;
        let rename = attrs.rename;
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
            BotCommandAttribute::Description => description = Some(attr.value()),
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
