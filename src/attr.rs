use syn::{
    parse::{Parse, ParseStream},
    LitStr, Token,
};

pub enum BotCommandAttribute {
    Prefix,
    Description,
    RenameRule,
    CustomParser,
    Separator,
}

impl Parse for BotCommandAttribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let name_arg: syn::Ident = input.parse()?;
        match name_arg.to_string().as_str() {
            "prefix" => Ok(BotCommandAttribute::Prefix),
            "description" => Ok(BotCommandAttribute::Description),
            "rename" => Ok(BotCommandAttribute::RenameRule),
            "parse_with" => Ok(BotCommandAttribute::CustomParser),
            "separator" => Ok(BotCommandAttribute::Separator),
            _ => Err(syn::Error::new(name_arg.span(), "unexpected argument")),
        }
    }
}

pub struct Attr {
    name: BotCommandAttribute,
    value: String,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let name = input.parse::<BotCommandAttribute>()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<LitStr>()?.value();

        Ok(Self { name, value })
    }
}

impl Attr {
    pub fn name(&self) -> &BotCommandAttribute {
        &self.name
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

pub struct VecAttrs {
    pub data: Vec<Attr>,
}

impl Parse for VecAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut data = vec![];
        while !input.is_empty() {
            data.push(input.parse()?);
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self { data })
    }
}
