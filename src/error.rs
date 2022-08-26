use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub(crate) struct Error(TokenStream);

pub(crate) fn compile_error<T>(data: T) -> Error
where
    T: ToTokens,
{
    Error(quote! { compile_error! { #data } })
}

impl From<Error> for proc_macro2::TokenStream {
    fn from(Error(e): Error) -> Self {
        e
    }
}

impl From<syn::Error> for Error {
    fn from(e: syn::Error) -> Self {
        Self(e.to_compile_error())
    }
}
