extern crate quote;

use quote::{quote, ToTokens};
use syn::{FieldsNamed, FieldsUnnamed, Type};

#[derive(Debug)]
pub(crate) enum ParserType {
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

pub(crate) fn impl_parse_args_unnamed(
    data: &FieldsUnnamed,
    variant: impl ToTokens,
    parser_type: &ParserType,
) -> proc_macro2::TokenStream {
    let get_arguments = create_parser(
        parser_type,
        data.unnamed.iter().map(|f| &f.ty),
        data.unnamed.len(),
    );
    let iter = (0..data.unnamed.len()).map(syn::Index::from);
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

pub(crate) fn impl_parse_args_named(
    data: &FieldsNamed,
    variant: impl ToTokens,
    parser_type: &ParserType,
) -> proc_macro2::TokenStream {
    let get_arguments = create_parser(
        parser_type,
        data.named.iter().map(|f| &f.ty),
        data.named.len(),
    );
    let i = (0..data.named.len()).map(syn::Index::from);
    let name = data.named.iter().map(|f| f.ident.as_ref().unwrap());
    let res = quote! {
        {
            #get_arguments
            #variant { #(#name: arguments.#i),* }
        }
    };
    res
}

fn create_parser<'a>(
    parser_type: &ParserType,
    mut types: impl Iterator<Item = &'a Type>,
    count_args: usize,
) -> proc_macro2::TokenStream {
    let function_to_parse = match parser_type {
        ParserType::Default => match count_args {
            1 => {
                let ty = types.next().expect("count_args != types.len()");
                quote! { (|s: String| {
                    let res = <#ty>::from_str(&s)
                        .map_err(|e|ParseError::IncorrectFormat({ let e: Box<dyn std::error::Error + Send + Sync + 'static> = e.into(); e }))?;
                    Ok((res, ))
                 })
                }
            }
            _ => quote! { compile_error!("Expected exactly 1 argument") },
        },
        ParserType::Split { separator } => parser_with_separator(
            &separator.clone().unwrap_or_else(|| " ".to_owned()),
            types,
            count_args,
        ),
        ParserType::Custom(s) => {
            let path = syn::parse_str::<syn::Path>(s).unwrap_or_else(|_| {
                panic!("Failed to parse a custom command parser, {}", s)
            });
            quote! { #path }
        }
    };
    quote! {
        let arguments = #function_to_parse(args)?;
    }
}

fn parser_with_separator<'a>(
    separator: &str,
    types: impl Iterator<Item = &'a Type>,
    count_args: usize,
) -> proc_macro2::TokenStream {
    let inner = quote! { let mut splited = s.split(#separator); };
    let i = 0..count_args;
    let inner2 = quote! {
        #(<#types>::from_str(splited.next().ok_or(ParseError::TooFewArguments {
            expected: #count_args,
            found: #i,
            message: format!("Expected but not found arg number {}", #i + 1),
        })?).map_err(|e|ParseError::IncorrectFormat({ let e: Box<dyn std::error::Error + Send + Sync + 'static> = e.into(); e }))?,)*
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
