extern crate quote;

use quote::quote;
use syn::FieldsUnnamed;

pub fn impl_parse_args_unnamed(data: &FieldsUnnamed) -> quote::__rt::TokenStream {
    let iter = 0..data.unnamed.len();
    quote! {
        (#(FromStr::from_str(args.get(#iter)),)*)
    }
}
