extern crate proc_macro;

mod attr;
mod bot_commands;
mod command;
mod command_attr;
mod command_enum;
mod error;
mod fields_parse;
mod rename_rules;
mod unzip;

pub(crate) use error::{compile_error, Result};
use syn::{parse_macro_input, DeriveInput};

use crate::bot_commands::bot_commands_impl;
use proc_macro::TokenStream;

#[proc_macro_derive(BotCommands, attributes(command))]
pub fn bot_commands_derive(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    bot_commands_impl(input).unwrap_or_else(<_>::into).into()
}
