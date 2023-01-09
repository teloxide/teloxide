use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub(crate) struct Error(TokenStream);

pub(crate) fn compile_error<T>(data: T) -> Error
where
    T: ToTokens,
{
    Error(quote! { ::std::compile_error! { #data } })
}

pub(crate) fn compile_error_at(msg: &str, sp: Span) -> Error {
    use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenTree};
    // compile_error! { $msg }
    let ts = TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new("compile_error", sp)),
        TokenTree::Punct({
            let mut punct = Punct::new('!', Spacing::Alone);
            punct.set_span(sp);
            punct
        }),
        TokenTree::Group({
            let mut group = Group::new(Delimiter::Brace, {
                TokenStream::from_iter(vec![TokenTree::Literal({
                    let mut string = Literal::string(msg);
                    string.set_span(sp);
                    string
                })])
            });
            group.set_span(sp);
            group
        }),
    ]);

    Error(ts)
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
