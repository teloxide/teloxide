extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(TelegramCommandEnum)]
pub fn derive_telegram_command_enum(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let (variant, variant_str) = match &input.data {
        syn::Data::Enum(data) => {
            (data.variants.iter(),
             data.variants.iter().map(|variant| {
                variant.ident.to_string().to_lowercase()
            }))
        }
        _ => panic!("TelegramCommandEnum allowed only for enums")
    };

    let ident = input.ident;

    let expanded = quote! {
        impl TelegramCommandEnum for #ident {
            fn try_from(value: &str) -> Option<Self> {
                match value {
                    #(
                        #variant_str => Some(Self::#variant),
                    )*
                    _ => None
                }
            }
        }
    };
    let tokens = TokenStream::from(expanded);
    tokens
}