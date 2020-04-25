mod attr;
mod command;
mod enum_attributes;
mod fields_parse;
mod rename_rules;

extern crate proc_macro;
extern crate quote;
extern crate syn;
use crate::fields_parse::impl_parse_args_unnamed;
use crate::{
    attr::{Attr, VecAttrs},
    command::Command,
    enum_attributes::CommandEnum,
};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Fields, Variant};

macro_rules! get_or_return {
    ($($some:tt)*) => {
        match $($some)* {
            Ok(elem) => elem,
            Err(e) => return e
        };
    }
}

#[proc_macro_derive(BotCommand, attributes(command))]
pub fn derive_telegram_command_enum(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let data_enum: &syn::DataEnum = get_or_return!(get_enum_data(&input));

    let enum_attrs: Vec<Attr> = get_or_return!(parse_attributes(&input.attrs));

    let command_enum = match CommandEnum::try_from(enum_attrs.as_slice()) {
        Ok(command_enum) => command_enum,
        Err(e) => return compile_error(e),
    };

    let variants: Vec<&syn::Variant> = data_enum.variants.iter().map(|variant| variant).collect();

    let mut vec_impl_create = vec![];
    for variant in &variants {
        match &variant.fields {
            Fields::Unnamed(fields) => {
                vec_impl_create.push(impl_parse_args_unnamed(fields));
            }
            Fields::Unit => {
                vec_impl_create.push(quote! {});
            }
            _ => panic!("only unnamed fields"), // TODO: named fields
        }
    }

    let mut variant_infos = vec![];
    for variant in &variants {
        let mut attrs = Vec::new();
        for attr in &variant.attrs {
            match attr.parse_args::<VecAttrs>() {
                Ok(mut attrs_) => {
                    attrs.append(attrs_.data.as_mut());
                }
                Err(e) => {
                    return compile_error(e.to_compile_error());
                }
            }
        }
        match Command::try_from(attrs.as_slice(), &variant.ident.to_string()) {
            Ok(command) => variant_infos.push(command),
            Err(e) => return compile_error(e),
        }
    }

    let ident = &input.ident;

    let fn_descriptions = impl_descriptions(&variant_infos, &command_enum);
    let fn_parse = impl_parse(&variants, &variant_infos, &command_enum, &vec_impl_create);

    let trait_impl = quote! {
        impl BotCommand for #ident {
            #fn_descriptions
            #fn_parse
        }
    };

    TokenStream::from(trait_impl)
}

fn impl_descriptions(infos: &[Command], global: &CommandEnum) -> quote::__private::TokenStream {
    let global_description = if let Some(s) = &global.description {
        quote! { #s, "\n", }
    } else {
        quote! {}
    };
    let command = infos.iter().map(|c| c.get_matched_value(global));
    let description = infos.iter().map(|info| {
        info.description
            .as_deref()
            .map(|e| format!(" - {}", e))
            .unwrap_or_default()
    });

    quote! {
        fn descriptions() -> String {
            std::concat!(#global_description #(#command, #description, '\n'),*).to_string()
        }
    }
}

fn impl_parse(
    variants: &[&Variant],
    infos: &[Command],
    global: &CommandEnum,
    variants_initialization: &[quote::__private::TokenStream],
) -> quote::__private::TokenStream {
    let matching_values = infos.iter().map(|c| c.get_matched_value(global));
    let variant_ident = variants.iter().map(|variant| &variant.ident);

    quote! {
         fn parse<N>(s: &str, bot_name: N) -> Option<Self>
         where
              N: Into<String>
         {
              let mut words = s.splitn(2, ' ');
              let mut splited = words.next()?.split('@');
              let command_raw = splited.next()?;
              let bot = splited.next();
              let bot_name = bot_name.into();
              match bot {
                  Some(name) if name == bot_name => {}
                  None => {}
                  _ => return None,
              }
              let mut args = words.next().unwrap_or("").to_string();
              match command_raw {
                   #(
                        #matching_values => Some(Self::#variant_ident #variants_initialization),
                   )*
                   _ => None,
              }
         }
    }
}

fn get_enum_data(input: &DeriveInput) -> Result<&syn::DataEnum, TokenStream> {
    match &input.data {
        syn::Data::Enum(data) => Ok(data),
        _ => Err(compile_error("TelegramBotCommand allowed only for enums")),
    }
}

fn parse_attributes(input: &[syn::Attribute]) -> Result<Vec<Attr>, TokenStream> {
    let mut enum_attrs = Vec::new();
    for attr in input.iter() {
        match attr.parse_args::<VecAttrs>() {
            Ok(mut attrs_) => {
                enum_attrs.append(attrs_.data.as_mut());
            }
            Err(e) => {
                return Err(compile_error(e.to_compile_error()));
            }
        }
    }
    Ok(enum_attrs)
}

fn compile_error<T>(data: T) -> TokenStream
where
    T: ToTokens,
{
    TokenStream::from(quote! { compile_error!(#data) })
}
