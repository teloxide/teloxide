// TODO: refactor this shit.

mod attr;
mod command;
mod command_enum;
mod error;
mod fields_parse;
mod rename_rules;
mod unzip;

extern crate proc_macro;
extern crate quote;
extern crate syn;
use crate::{
    attr::CommandAttrs, command::Command, command_enum::CommandEnum,
    fields_parse::impl_parse_args, unzip::Unzip,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) use error::{compile_error, Error, Result};

#[proc_macro_derive(BotCommands, attributes(command))]
pub fn bot_commands_derive(tokens: TokenStream) -> TokenStream {
    bot_commands_impl(tokens).unwrap_or_else(Error::into)
}

fn bot_commands_impl(tokens: TokenStream) -> Result<TokenStream, Error> {
    let input = syn::parse_macro_input::parse::<DeriveInput>(tokens)?;

    let data_enum = get_enum_data(&input)?;
    let enum_attrs = CommandAttrs::from_attributes(&input.attrs)?;
    let command_enum = CommandEnum::try_from(enum_attrs)?;

    let Unzip(var_init, var_info) = data_enum
        .variants
        .iter()
        .map(|variant| {
            let attrs = CommandAttrs::from_attributes(&variant.attrs)?;
            let command = Command::try_from(attrs, &variant.ident.to_string())?;

            let variant_name = &variant.ident;
            let self_variant = quote! { Self::#variant_name };

            let parser =
                command.parser.as_ref().unwrap_or(&command_enum.parser_type);
            let parse = impl_parse_args(&variant.fields, self_variant, parser);

            Ok((parse, command))
        })
        .collect::<Result<Unzip<Vec<_>, Vec<_>>, Error>>()?;

    let type_name = &input.ident;
    let fn_descriptions = impl_descriptions(&var_info, &command_enum);
    let fn_parse = impl_parse(&var_info, &command_enum, &var_init);
    let fn_commands = impl_commands(&var_info, &command_enum);

    let trait_impl = quote! {
        impl BotCommands for #type_name {
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
    let commands = infos
        .iter()
        .filter(|command| command.description_is_enabled())
        .map(|command| {
            let c = command.get_matched_value(global);
            let d = command.description.as_deref().unwrap_or_default();
            quote! { BotCommand::new(#c,#d) }
        });

    quote! {
        fn bot_commands() -> Vec<teloxide::types::BotCommand> {
            use teloxide::types::BotCommand;
            vec![#(#commands),*]
        }
    }
}

fn impl_descriptions(
    infos: &[Command],
    global: &CommandEnum,
) -> proc_macro2::TokenStream {
    let command_descriptions = infos
        .iter()
        .filter(|command| command.description_is_enabled())
        .map(|c| {
            let (prefix, command) = c.get_matched_value2(global);
            let description = c.description.clone().unwrap_or_default();
            quote! { CommandDescription { prefix: #prefix, command: #command, description: #description } }
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
