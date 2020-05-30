extern crate quote;

use quote::__private::Span;
use quote::{quote, ToTokens};
use syn::FieldsUnnamed;

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
    let function_to_parse = match parser_type {
        ParserType::Default => {
            match data.unnamed.len() {
                1 => {
                    quote! { (|s: String| Ok((FromStr::from_str(&s).map_err(|_|ParseError::UncorrectFormat)?,)) ) }
                }
                _ => quote! { compile_error!("Expected 1 argument") },
            }
        }
        ParserType::Split { separator } => parser_with_separator(
            &separator.clone().unwrap_or(" ".to_owned()),
            data.unnamed.len(),
        ),
        ParserType::Custom(s) => {
            let ident = syn::Ident::new(&s, Span::call_site());
            quote! { #ident }
        }
    };
    let get_arguments = quote! { let arguments = #function_to_parse(args)?; };
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
