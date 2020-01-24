mod attr;
mod command;

extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Token};
use syn::parse::{Parse, ParseStream};
use crate::command::Command;
use std::convert::TryFrom;
use crate::attr::{Attr, VecAttrs};

#[proc_macro_derive(TelegramBotCommand, attributes(command))]
pub fn derive_telegram_command_enum(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let variants: Vec<&syn::Variant> = match &input.data {
        syn::Data::Enum(data) => data.variants.iter().map(|attr| attr).collect(),
        _ => return TokenStream::from(quote! { compile_error!("TelegramBotCommand allowed only for enums") })
    };

    let mut variant_infos = vec![];
    for variant in variants.iter() {
        let mut attrs = Vec::new();
        for attr in &variant.attrs {
            match attr.parse_args::<VecAttrs>() {
                Ok(mut attrs_) => {
                    attrs.append(attrs_.data.as_mut());
                },
                Err(e) => {
                    let e = e.to_compile_error();
                    return TokenStream::from(quote! { compile_error!(#e) });
                },
            }
        }
        match Command::try_from(attrs.as_slice()) {
            Ok(command) => variant_infos.push(command),
            Err(e) => return TokenStream::from(quote! { compile_error!(#e) }),
        }
    }

    let variant_ident = variants.iter().map(|variant| &variant.ident);
    let variant_name = variants.iter().map(|variant| variant.ident.to_string().to_lowercase());
    let variant_prefixes = variant_infos.iter().map(|info| &info.prefix);
    let variant_str1 = variant_prefixes.zip(variant_name).map(|(prefix, command)| prefix.to_string() + command.as_str());
    let variant_str2 = variant_str1.clone();
    let variant_description = variant_infos.iter().map(|info| info.description.as_ref().map(String::as_str).unwrap_or(""));

    let ident = input.ident;

    let expanded = quote! {
        impl TelegramBotCommand for #ident {
            fn try_from(value: &str) -> Option<Self> {
                match value {
                    #(
                        #variant_str1 => Some(Self::#variant_ident),
                    )*
                    _ => None
                }
            }
            fn descriptions() -> String {
                std::concat!(#(#variant_str2, " - ", #variant_description, '\n'),*).to_string()
            }
        }
    };
    //for debug
    //println!("{}", &expanded.to_string());
    let tokens = TokenStream::from(expanded);
    tokens
}