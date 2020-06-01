extern crate quote;

use quote::__private::Span;
use quote::{quote, ToTokens};
use syn::{FieldsUnnamed, FieldsNamed};

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
    let get_arguments = create_parser(parser_type, data.unnamed.len());
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
    let get_arguments = create_parser(parser_type, data.named.len());
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


fn create_parser(parser_type: &ParserType, count_args: usize) -> quote::__private::TokenStream {
    let function_to_parse = match parser_type {
        ParserType::Default => {
            match count_args {
                1 => {
                    quote! { (|s: String| Ok((FromStr::from_str(&s).map_err(|_|ParseError::UncorrectFormat)?,)) ) }
                }
                _ => quote! { compile_error!("Expected 1 argument") },
            }
        }
        ParserType::Split { separator } => parser_with_separator(
            &separator.clone().unwrap_or(" ".to_owned()),
            count_args,
        ),
        ParserType::Custom(s) => {
            let ident = syn::Ident::new(&s, Span::call_site());
            quote! { #ident }
        }
    };
    quote! { let arguments = #function_to_parse(args)?; }
}

fn parser_with_separator(separator: &str, count_args: usize) -> quote::__private::TokenStream {
    let inner = quote! { let splited = s.split(#separator).collect::<Vec<_>>(); };
    let mut inner2 = quote! {};
    for i in 0..count_args {
        inner2.extend(
            quote! { FromStr::from_str(splited[#i]).map_err(|_|ParseError::UncorrectFormat)?, },
        )
    }
    let res = quote! {
        (|s: String| {
            #inner
            Ok((#inner2))
        })
    };
    res
}
