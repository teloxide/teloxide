use crate::Result;

use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    Attribute, LitStr, Token,
};

pub(crate) enum CommandAttrName {
    Prefix,
    Description,
    Rename,
    ParseWith,
    Separator,
}

impl Parse for CommandAttrName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name_arg: syn::Ident = input.parse()?;

        match name_arg.to_string().as_str() {
            "prefix" => Ok(CommandAttrName::Prefix),
            "description" => Ok(CommandAttrName::Description),
            "rename" => Ok(CommandAttrName::Rename),
            "parse_with" => Ok(CommandAttrName::ParseWith),
            "separator" => Ok(CommandAttrName::Separator),
            _ => Err(syn::Error::new(
                name_arg.span(),
                "unexpected attribute name (expected one of `prefix`, \
                 `description`, `rename`, `parse_with`, `separator`",
            )),
        }
    }
}

pub(crate) struct CommandAttr {
    pub name: CommandAttrName,
    pub value: String,
}

impl Parse for CommandAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<CommandAttrName>()?;

        // FIXME: this should support value-less attrs, as well as
        //        non-string-literal values
        input.parse::<Token![=]>()?;
        let value = input.parse::<LitStr>()?.value();

        Ok(Self { name, value })
    }
}

pub(crate) struct CommandAttrs(Vec<CommandAttr>);

impl CommandAttrs {
    pub fn from_attributes(attributes: &[Attribute]) -> Result<Self> {
        let mut attrs = Vec::new();

        for attribute in attributes.iter().filter(is_command_attribute) {
            let attrs_ = attribute.parse_args_with(|input: &ParseBuffer| {
                input.parse_terminated::<_, Token![,]>(CommandAttr::parse)
            })?;

            attrs.extend(attrs_);
        }

        Ok(Self(attrs))
    }
}

impl<'a> IntoIterator for &'a CommandAttrs {
    type Item = &'a CommandAttr;

    type IntoIter = std::slice::Iter<'a, CommandAttr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for CommandAttrs {
    type Item = CommandAttr;

    type IntoIter = std::vec::IntoIter<CommandAttr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn is_command_attribute(a: &&Attribute) -> bool {
    match a.path.get_ident() {
        Some(ident) => ident == "command",
        _ => false,
    }
}
