use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Lit, Token};


pub enum BotCommandAttribute {
    Prefix,
    Description
}

impl Parse for BotCommandAttribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let name_arg: syn::Ident = input.parse()?;
        match name_arg.to_string().as_str() {
            "prefix" => Ok(BotCommandAttribute::Prefix),
            "description" => Ok(BotCommandAttribute::Description),
            _ => Err(syn::Error::new(name_arg.span(), "unexpected argument"))
        }
    }
}

pub struct Attr {
    name: BotCommandAttribute,
    value: String
}

impl Parse for Attr
{
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let name = input.parse::<BotCommandAttribute>()?;
        input.parse::<Token![=]>()?;
        let value = match input.parse::<Lit>()? {
            Lit::Str(s) => s.value(),
            Lit::ByteStr(lit) => return Err(syn::Error::new(lit.span(), "expected string literal")),
            Lit::Byte(lit) => return Err(syn::Error::new(lit.span(), "expected string literal")),
            Lit::Char(lit) => return Err(syn::Error::new(lit.span(), "expected string literal")),
            Lit::Int(lit) => return Err(syn::Error::new(lit.span(), "expected string literal")),
            Lit::Float(lit) => return Err(syn::Error::new(lit.span(), "expected string literal")),
            Lit::Bool(lit) => return Err(syn::Error::new(lit.span, "expected string literal")),
            Lit::Verbatim(lit)  => return Err(syn::Error::new(lit.span(), "expected string literal")),
        };

        Ok(Self {
            name,
            value
        })
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
