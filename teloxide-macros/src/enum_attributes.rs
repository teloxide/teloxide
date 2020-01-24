use crate::attr::{Attr, BotCommandAttribute};
use std::convert::TryFrom;

pub struct CommandEnum {
    pub prefix: Option<String>,
    pub description: Option<String>,
}

impl CommandEnum {
    fn from_attrs(prefix: Option<&Attr>, description: Option<&Attr>) -> Self {
        let prefix = prefix.map(|attr| attr.value());
        let description = description.map(|attr| attr.value());

        Self {
            prefix,
            description
        }
    }
}

impl TryFrom<&[Attr]> for CommandEnum {
    type Error = String;

    fn try_from(attrs: &[Attr]) -> Result<Self, Self::Error> {
        let mut prefix = None;
        let mut description = None;

        for attr in attrs {
            match attr.name() {
                BotCommandAttribute::Prefix => prefix = Some(attr),
                BotCommandAttribute::Description => description = Some(attr),
                _ => return Err(format!("unexpected attribute")),
            }
        }

        Ok(Self::from_attrs(prefix, description))
    }
}
