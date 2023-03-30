use quote::quote;
use syn::{Fields, FieldsNamed, FieldsUnnamed, Type};

use crate::{attr::AttrValue, error::Result};

#[derive(Clone)]
pub(crate) enum ParserType {
    Default,
    Split { separator: Option<String> },
    Custom(syn::Path),
}

impl ParserType {
    pub fn parse(value: AttrValue) -> Result<Self> {
        value.expect(r#""default", "split", or a path to a custom parser function"#, |v| match v {
            AttrValue::Path(p) => Ok(ParserType::Custom(p)),
            AttrValue::Lit(syn::Lit::Str(ref l)) => match &*l.value() {
                "default" => Ok(ParserType::Default),
                "split" => Ok(ParserType::Split { separator: None }),
                _ => Err(v),
            },
            _ => Err(v),
        })
    }
}

pub(crate) fn impl_parse_args(
    fields: &Fields,
    self_variant: proc_macro2::TokenStream,
    parser: &ParserType,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unit => self_variant,
        Fields::Unnamed(fields) => impl_parse_args_unnamed(fields, self_variant, parser),
        Fields::Named(named) => impl_parse_args_named(named, self_variant, parser),
    }
}

pub(crate) fn impl_parse_args_unnamed(
    data: &FieldsUnnamed,
    variant: proc_macro2::TokenStream,
    parser_type: &ParserType,
) -> proc_macro2::TokenStream {
    let get_arguments = create_parser(parser_type, data.unnamed.iter().map(|f| &f.ty));
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
    variant: proc_macro2::TokenStream,
    parser_type: &ParserType,
) -> proc_macro2::TokenStream {
    let get_arguments = create_parser(parser_type, data.named.iter().map(|f| &f.ty));
    let i = (0..).map(syn::Index::from);
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
    mut types: impl ExactSizeIterator<Item = &'a Type>,
) -> proc_macro2::TokenStream {
    let function_to_parse = match parser_type {
        ParserType::Default => match types.len() {
            1 => {
                let ty = types.next().unwrap();
                quote! {
                    (
                        |s: ::std::string::String| {
                            let res = <#ty>::from_str(&s)
                                .map_err(|e| teloxide::utils::command::ParseError::IncorrectFormat(e.into()))?;

                            ::std::result::Result::Ok((res,))
                        }
                    )
                }
            }
            _ => {
                quote! { ::std::compile_error!("Default parser works only with exactly 1 field") }
            }
        },
        ParserType::Split { separator } => {
            parser_with_separator(&separator.clone().unwrap_or_else(|| " ".to_owned()), types)
        }
        ParserType::Custom(path) => quote! { #path },
    };

    quote! {
        let arguments = #function_to_parse(args)?;
    }
}

fn parser_with_separator<'a>(
    separator: &str,
    types: impl ExactSizeIterator<Item = &'a Type>,
) -> proc_macro2::TokenStream {
    let expected = types.len();
    let res = {
        let found = 0usize..;
        quote! {
            (
                #(
                    {
                        let s = splitted.next().ok_or(teloxide::utils::command::ParseError::TooFewArguments {
                            expected: #expected,
                            found: #found,
                            message: format!("Expected but not found arg number {}", #found + 1),
                        })?;

                        <#types>::from_str(s).map_err(|e| teloxide::utils::command::ParseError::IncorrectFormat(e.into()))?
                    },
                )*
            )
        }
    };

    let res = quote! {
        (
            |s: ::std::string::String| {
                let mut splitted = s.split(#separator);

                let res = #res;

                match splitted.next() {
                    Some(d) if !s.is_empty() => ::std::result::Result::Err(teloxide::utils::command::ParseError::TooManyArguments {
                        expected: #expected,
                        found: #expected + 1 + splitted.count(),
                        message: format!("Excess argument: {}", d),
                    }),
                    _ => ::std::result::Result::Ok(res)
                }
            }
        )
    };

    res
}
