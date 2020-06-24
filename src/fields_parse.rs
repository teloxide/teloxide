extern crate quote;

use quote::__private::Span;
use quote::{quote, ToTokens};
use syn::{FieldsNamed, FieldsUnnamed, Type};

#[derive(Debug)]
pub enum ParserType {
    Default,
    Split { separator: Option<String> },
    Custom(String),
}

impl ParserType {
    pub fn parse(data: &str) -> Self {
        match data {
            "default" => ParserType::Default,
            "split" => ParserType::Split { separator: None },
            s => ParserType::Custom(s.to_owned()),
        }
    }
}

pub fn impl_parse_args_unnamed(
    data: &FieldsUnnamed,
    variant: impl ToTokens,
    parser_type: &ParserType,
) -> quote::__private::TokenStream {
    let get_arguments = create_parser(parser_type, data.unnamed.iter().map(|f| &f.ty), data.unnamed.len());
    let iter = 0..data.unnamed.len();
    let mut initialization = quote! {};
    for i in iter {
        initialization.extend(quote! { arguments.#i, })
    }
    let res = quote! {
        {
            #get_arguments
            #variant(#initialization)
        }
    };
    res
}

pub fn impl_parse_args_named(
    data: &FieldsNamed,
    variant: impl ToTokens,
    parser_type: &ParserType,
) -> quote::__private::TokenStream {
    let get_arguments = create_parser(parser_type, data.named.iter().map(|f| &f.ty), data.named.len());
    let i = 0..data.named.len();
    let name = data.named.iter().map(|f| f.ident.as_ref().unwrap());
    let res = quote! {
        {
            #get_arguments
            #variant { #(#name: arguments.#i),* }
        }
    };
    res
}

fn create_parser<'a>(parser_type: &ParserType, types: impl Iterator<Item = &'a Type>, count_args: usize) -> quote::__private::TokenStream {
    let function_to_parse = match parser_type {
        ParserType::Default => match count_args {
            1 => {
                quote! { (|s: String| Ok((s,))) }
            }
            _ => quote! { compile_error!("Expected 1 argument") },
        },
        ParserType::Split { separator } => {
            parser_with_separator(&separator.clone().unwrap_or(" ".to_owned()), types, count_args)
        }
        ParserType::Custom(s) => {
            let ident = syn::Ident::new(&s, Span::call_site());
            quote! { #ident }
        }
    };
    quote! {
        let arguments = #function_to_parse(args)?;
    }
}

fn parser_with_separator<'a>(separator: &str, types: impl Iterator<Item = &'a Type>, count_args: usize) -> quote::__private::TokenStream {
    let inner = quote! { let mut splited = s.split(#separator); };
    let i = 0..count_args;
    let inner2 =
        quote! { 
                #(#types::from_str(splited.next().ok_or(ParseError::TooFewArguments {
                    expected: #count_args,
                    found: #i,
                    message: format!("Expected but not found arg number {}", #i + 1),
                })?).map_err(|e|ParseError::IncorrectFormat({ let e: Box<dyn std::error::Error> = e.into(); e }))?,)*
            };
    let res = quote! {
        (|s: String| {
            #inner
            let res = (#inner2);
            match splited.next() {
                Some(d) => Err(ParseError::TooManyArguments {
                    expected: #count_args,
                    found: #count_args + 1,
                    message: format!("Excess argument: {}", d),
                }),
                None => Ok(res)
            }
        })
    };
    res
}
