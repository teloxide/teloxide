use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr, Token,
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

pub(crate) struct CommandAttrs(Punctuated<CommandAttr, Token![,]>);

impl Parse for CommandAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse_terminated(CommandAttr::parse).map(Self)
    }
}

impl IntoIterator for CommandAttrs {
    type Item = CommandAttr;

    type IntoIter = syn::punctuated::IntoIter<CommandAttr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
