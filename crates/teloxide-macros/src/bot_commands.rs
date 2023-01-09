use crate::{
    command::Command, command_enum::CommandEnum, compile_error, fields_parse::impl_parse_args,
    unzip::Unzip, Result,
};

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn bot_commands_impl(input: DeriveInput) -> Result<TokenStream> {
    let data_enum = get_enum_data(&input)?;
    let command_enum = CommandEnum::from_attributes(&input.attrs)?;

    let Unzip(var_init, var_info) = data_enum
        .variants
        .iter()
        .map(|variant| {
            let command = Command::new(&variant.ident.to_string(), &variant.attrs, &command_enum)?;

            let variant_name = &variant.ident;
            let self_variant = quote! { Self::#variant_name };

            let parse = impl_parse_args(&variant.fields, self_variant, &command.parser);

            Ok((parse, command))
        })
        .collect::<Result<Unzip<Vec<_>, Vec<_>>>>()?;

    let type_name = &input.ident;
    let fn_descriptions = impl_descriptions(&var_info, &command_enum);
    let fn_parse = impl_parse(&var_info, &var_init);
    let fn_commands = impl_commands(&var_info);

    let trait_impl = quote! {
        impl teloxide::utils::command::BotCommands for #type_name {
            #fn_descriptions
            #fn_parse
            #fn_commands
        }
    };

    Ok(trait_impl)
}

fn impl_commands(infos: &[Command]) -> proc_macro2::TokenStream {
    let commands = infos.iter().filter(|command| command.description_is_enabled()).map(|command| {
        let c = command.get_prefixed_command();
        let d = command.description.as_deref().unwrap_or_default();
        quote! { BotCommand::new(#c,#d) }
    });

    quote! {
        fn bot_commands() -> ::std::vec::Vec<teloxide::types::BotCommand> {
            use teloxide::types::BotCommand;
            ::std::vec![#(#commands),*]
        }
    }
}

fn impl_descriptions(infos: &[Command], global: &CommandEnum) -> proc_macro2::TokenStream {
    let command_descriptions = infos
        .iter()
        .filter(|command| command.description_is_enabled())
        .map(|Command { prefix, name, description, ..}| {
            let description = description.clone().unwrap_or_default();
            quote! { CommandDescription { prefix: #prefix, command: #name, description: #description } }
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
    variants_initialization: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    let matching_values = infos.iter().map(|c| c.get_prefixed_command());

    quote! {
         fn parse(s: &str, bot_name: &str) -> ::std::result::Result<Self, teloxide::utils::command::ParseError> {
              // FIXME: we should probably just call a helper function from `teloxide`, instead of parsing command syntax ourselves
              use std::str::FromStr;
              use teloxide::utils::command::ParseError;

              // 2 is used to only split once (=> in two parts),
              // we only need to split the command and the rest of arguments.
              let mut words = s.splitn(2, ' ');

              // Unwrap: split iterators always have at least one item
              let mut full_command = words.next().unwrap().split('@');
              let command = full_command.next().unwrap();

              let bot_username = full_command.next();
              match bot_username {
                  ::std::option::Option::None => {}
                  ::std::option::Option::Some(username) if username.eq_ignore_ascii_case(bot_name) => {}
                  ::std::option::Option::Some(n) => return ::std::result::Result::Err(ParseError::WrongBotName(n.to_owned())),
              }

              let args = words.next().unwrap_or("").to_owned();
              match command {
                   #(
                        #matching_values => Ok(#variants_initialization),
                   )*
                   _ => ::std::result::Result::Err(ParseError::UnknownCommand(command.to_owned())),
              }
         }
    }
}

fn get_enum_data(input: &DeriveInput) -> Result<&syn::DataEnum> {
    match &input.data {
        syn::Data::Enum(data) => Ok(data),
        _ => Err(compile_error("`BotCommands` is only allowed for enums")),
    }
}
