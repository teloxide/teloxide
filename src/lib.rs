// TODO: refactor this shit.

mod attr;
mod command;
mod command_enum;
mod error;
mod fields_parse;
mod rename_rules;

extern crate proc_macro;
extern crate quote;
extern crate syn;
use crate::{
    attr::CommandAttrs,
    command::Command,
    command_enum::CommandEnum,
    fields_parse::{impl_parse_args_named, impl_parse_args_unnamed},
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields};

pub(crate) use error::{compile_error, Error, Result};

#[proc_macro_derive(BotCommands, attributes(command))]
pub fn bot_commands_derive(tokens: TokenStream) -> TokenStream {
    bot_commands_impl(tokens).unwrap_or_else(Error::into)
}

fn bot_commands_impl(tokens: TokenStream) -> Result<TokenStream, Error> {
    let input = syn::parse_macro_input::parse::<DeriveInput>(tokens)?;

    let data_enum: &syn::DataEnum = get_enum_data(&input)?;
    let enum_attrs = CommandAttrs::from_attributes(&input.attrs)?;
    let command_enum = CommandEnum::try_from(enum_attrs)?;

    let variant_infos = data_enum
        .variants
        .iter()
        .map(|variant| {
            let attrs = CommandAttrs::from_attributes(&variant.attrs)?;
            let command = Command::try_from(attrs, &variant.ident.to_string())?;

            Ok(command)
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let mut vec_impl_create = vec![];

    for (variant, info) in data_enum.variants.iter().zip(variant_infos.iter()) {
        let var = &variant.ident;
        let variant_ = quote! { Self::#var };
        match &variant.fields {
            Fields::Unnamed(fields) => {
                let parser =
                    info.parser.as_ref().unwrap_or(&command_enum.parser_type);
                vec_impl_create
                    .push(impl_parse_args_unnamed(fields, variant_, parser));
            }
            Fields::Unit => {
                vec_impl_create.push(variant_);
            }
            Fields::Named(named) => {
                let parser =
                    info.parser.as_ref().unwrap_or(&command_enum.parser_type);
                vec_impl_create
                    .push(impl_parse_args_named(named, variant_, parser));
            }
        }
    }

    let ident = &input.ident;

    let fn_descriptions = impl_descriptions(&variant_infos, &command_enum);
    let fn_parse = impl_parse(&variant_infos, &command_enum, &vec_impl_create);
    let fn_commands = impl_commands(&variant_infos, &command_enum);

    let trait_impl = quote! {
        impl BotCommands for #ident {
            #fn_descriptions
            #fn_parse
            #fn_commands
        }
    };

    Ok(TokenStream::from(trait_impl))
}

fn impl_commands(
    infos: &[Command],
    global: &CommandEnum,
) -> proc_macro2::TokenStream {
    let commands_to_list = infos.iter().filter_map(|command| {
        if command.description == Some("off".into()) {
            None
        } else {
            let c = command.get_matched_value(global);
            let d = command.description.as_deref().unwrap_or_default();
            Some(quote! { BotCommand::new(#c,#d) })
        }
    });
    quote! {
        fn bot_commands() -> Vec<teloxide::types::BotCommand> {
            use teloxide::types::BotCommand;
            vec![#(#commands_to_list),*]
        }
    }
}

fn impl_descriptions(
    infos: &[Command],
    global: &CommandEnum,
) -> proc_macro2::TokenStream {
    let command_descriptions = infos.iter().filter_map(|c| {
        let (prefix, command) = c.get_matched_value2(global);
        let description = c.description.clone().unwrap_or_default();
        (description != "off").then(|| quote! { CommandDescription { prefix: #prefix, command: #command, description: #description } })
    });

    let global_description = match global.description.as_deref() {
        Some(gd) => quote! { .global_description(#gd) },
        None => quote! {},
    };

    quote! {
        fn descriptions() -> teloxide::utils::command::CommandDescriptions<'static> {
            use teloxide::utils::command::{CommandDescriptions, CommandDescription};
            use std::borrow::Cow;

            CommandDescriptions::new(&[
                #(#command_descriptions),*
            ])
            #global_description
        }
    }
}

fn impl_parse(
    infos: &[Command],
    global: &CommandEnum,
    variants_initialization: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    let matching_values = infos.iter().map(|c| c.get_matched_value(global));

    quote! {
         fn parse<N>(s: &str, bot_name: N) -> Result<Self, teloxide::utils::command::ParseError>
         where
              N: Into<String>
         {
              use std::str::FromStr;
              use teloxide::utils::command::ParseError;

              let mut words = s.splitn(2, ' ');
              let mut splitted = words.next().expect("First item will be always.").split('@');
              let command_raw = splitted.next().expect("First item will be always.");
              let bot = splitted.next();
              let bot_name = bot_name.into();
              match bot {
                  Some(name) if name.eq_ignore_ascii_case(&bot_name) => {}
                  None => {}
                  Some(n) => return Err(ParseError::WrongBotName(n.to_string())),
              }
              let mut args = words.next().unwrap_or("").to_string();
              match command_raw {
                   #(
                        #matching_values => Ok(#variants_initialization),
                   )*
                   _ => Err(ParseError::UnknownCommand(command_raw.to_string())),
              }
         }
    }
}

fn get_enum_data(input: &DeriveInput) -> Result<&syn::DataEnum> {
    match &input.data {
        syn::Data::Enum(data) => Ok(data),
        _ => Err(compile_error("TelegramBotCommand allowed only for enums")),
    }
}
