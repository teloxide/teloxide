extern crate quote;

use quote::{quote};
use syn::FieldsUnnamed;

pub fn impl_parse_args_unnamed(data: &FieldsUnnamed) -> quote::__private::TokenStream {
    let iter = 0..data.unnamed.len();
    let mut tokens = quote! {};
    for _ in iter {
        tokens.extend(quote! { CommandArgument::parse(&mut args)?, });
    }
    quote! { (#tokens) }
}
