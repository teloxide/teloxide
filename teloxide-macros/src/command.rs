use crate::attr::{Attr, BotCommandAttribute};
use std::convert::TryFrom;

pub struct Command {
    pub prefix: String,
    pub description: Option<String>,
}

impl Command {
    fn from_attrs(prefix: Option<&Attr>, description: Option<&Attr>) -> Self {
        let prefix = match prefix {
            Some(attr) => attr.value(),
            None => "/".to_string(),
        };
        let description = description.map(|attr| attr.value());

        Self {
            prefix,
            description
        }
    }
}

impl TryFrom<&[Attr]> for Command {
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
