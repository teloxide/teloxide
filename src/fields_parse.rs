use quote::quote;
use syn::{Fields, FieldsNamed, FieldsUnnamed, Type};

#[derive(Debug)]
pub(crate) enum ParserType {
    Default,
    Split { separator: Option<String> },
    Custom(String),
}

impl ParserType {
    // FIXME: use path for custom
    pub fn parse(data: &str) -> Self {
        match data {
            "default" => ParserType::Default,
            "split" => ParserType::Split { separator: None },
            s => ParserType::Custom(s.to_owned()),
        }
    }
}

pub(crate) fn impl_parse_args(
    fields: &Fields,
    self_variant: proc_macro2::TokenStream,
    parser: &ParserType,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unit => self_variant,
        Fields::Unnamed(fields) => {
            impl_parse_args_unnamed(fields, self_variant, parser)
        }
        Fields::Named(named) => {
            impl_parse_args_named(named, self_variant, parser)
        }
    }
}

pub(crate) fn impl_parse_args_unnamed(
    data: &FieldsUnnamed,
    variant: proc_macro2::TokenStream,
    parser_type: &ParserType,
) -> proc_macro2::TokenStream {
    let get_arguments =
        create_parser(parser_type, data.unnamed.iter().map(|f| &f.ty));
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
    let get_arguments =
        create_parser(parser_type, data.named.iter().map(|f| &f.ty));
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
                        |s: String| {
                            let res = <#ty>::from_str(&s)
                                .map_err(|e| ParseError::IncorrectFormat(e.into()))?;

                            Ok((res,))
                        }
                    )
                }
            }
            _ => {
                quote! { compile_error!("Default parser works only with exactly 1 field") }
            }
        },
        ParserType::Split { separator } => parser_with_separator(
            &separator.clone().unwrap_or_else(|| " ".to_owned()),
            types,
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
    types: impl ExactSizeIterator<Item = &'a Type>,
) -> proc_macro2::TokenStream {
    let expected = types.len();
    let res = {
        let found = 0usize..;
        quote! {
            (
                #(
                    {
                        let s = splitted.next().ok_or(ParseError::TooFewArguments {
                            expected: #expected,
                            found: #found,
                            message: format!("Expected but not found arg number {}", #found + 1),
                        })?;

                        <#types>::from_str(s).map_err(|e| ParseError::IncorrectFormat(e.into()))?
                    }
                ),*
            )
        }
    };

    let res = quote! {
        (
            |s: String| {
                let mut splitted = s.split(#separator);

                let res = #res;

                match splitted.next() {
                    Some(d) => Err(ParseError::TooManyArguments {
                        expected: #expected,
                        found: #expected + 1,
                        message: format!("Excess argument: {}", d),
                    }),
                    None => Ok(res)
                }
            }
        )
    };

    res
}
