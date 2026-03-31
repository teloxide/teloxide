use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

use crate::{
    build_keyboard::impl_keyboard_args, button::Button, button_enum::ButtonEnum, compile_error,
    error::compile_error_at, fields_parse::impl_parse_args, fields_stringify::impl_stringify_args,
    unzip::Unzip5, Result,
};

pub(crate) fn inline_buttons_impl(input: DeriveInput) -> Result<TokenStream> {
    let data_enum = get_enum_data(&input)?;
    let button_enum = ButtonEnum::from_attributes(&input.attrs)?;
    let type_name = &input.ident;
    let mut current_row = 1;

    let Unzip5(var_init, var_info, var_stringify, var_construct_variant, var_parameter) = data_enum
        .variants
        .iter()
        .map(|variant| {
            let button = Button::new(
                &variant.ident.to_string(),
                current_row,
                &variant.attrs,
                &button_enum,
                &variant.fields,
            )?;

            if button.row > current_row {
                return Err(compile_error_at(
                    &format!(
                        "Entered row {} is bigger than the current max row {current_row}",
                        button.row
                    ),
                    variant.span(),
                ));
            }
            if button.row == current_row {
                // This doesnt interfere if user wants to make some
                // buttons with a row value and others without
                current_row += 1;
            }

            if button.data_name.len() > 64 {
                return Err(compile_error_at(
                    // Limit of the TBA
                    //
                    // "64 chars" and not "64 bytes" because enum variants usually don't contain
                    // non-ascii characters, and it's more understandable.
                    "Enum variant is too long (64 chars max), please consider using \
                     `#[button(rename=\"...\")` to reduce the size",
                    variant.span(),
                ));
            }

            let variant_name = &variant.ident;
            let self_variant = quote! { Self::#variant_name };
            let button_data_name = button.data_name.clone();
            let self_string_variant = button_data_name.to_owned();
            let self_string_name = type_name.to_string();

            let parse = impl_parse_args(&variant.fields, self_variant.clone(), &button.parser);
            let stringify = impl_stringify_args(
                &variant.fields,
                self_variant.clone(),
                self_string_name,
                self_string_variant.clone(),
            );

            let (construct_variant, parameter) =
                impl_keyboard_args(&variant.fields, self_string_variant, self_variant, &button)?;

            Ok((parse, button, stringify, construct_variant, parameter))
        })
        .collect::<Result<Unzip5<Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>>>>()?;

    let fn_parse = impl_parse(&var_info, &var_init, &button_enum.fields_separator);
    let fn_stringify = impl_stringify(&var_stringify, &button_enum.fields_separator);
    let fn_build_keyboard = impl_build_keyboard(&var_info, &var_construct_variant, &var_parameter);

    let trait_impl = quote! {
        impl teloxide::utils::button::InlineButtons for #type_name {
            #fn_parse
            #fn_stringify
        }

        impl #type_name {
            #fn_build_keyboard
        }
    };

    Ok(trait_impl)
}

fn impl_parse(
    infos: &[Button],
    variants_initialization: &[proc_macro2::TokenStream],
    fields_separator: &str,
) -> proc_macro2::TokenStream {
    let matching_values = infos.iter().map(|c| c.data_name.clone());

    quote! {
         fn parse(s: &str) -> ::std::result::Result<Self, teloxide::utils::command::ParseError> {
              use std::str::FromStr;
              use teloxide::utils::command::ParseError;

              // 2 is used to only split once (=> in two parts),
              // we only need to split the enum variant and the rest of arguments.
              let mut words = s.splitn(2, #fields_separator);

              let enum_variant = words.next().unwrap();

              let args = words.next().unwrap_or("").to_owned();
              match enum_variant {
                   #(
                        #matching_values => Ok(#variants_initialization),
                   )*
                   _ => ::std::result::Result::Err(ParseError::UnknownCallbackDataVariant(enum_variant.to_owned())),
              }
         }
    }
}

fn impl_stringify(
    stringify_return: &[proc_macro2::TokenStream],
    fields_separator: &str,
) -> proc_macro2::TokenStream {
    quote! {
        fn stringify(self) -> ::std::result::Result<::std::string::String, teloxide::utils::button::StringifyError> {
            use std::string::ToString;
            use teloxide::utils::button::StringifyError;

            let fields_separator = #fields_separator;

            match self {
                #(#stringify_return)*
            }
        }
    }
}

fn impl_build_keyboard(
    infos: &[Button],
    construct_variants: &[proc_macro2::TokenStream],
    parameters: &[Option<proc_macro2::TokenStream>],
) -> proc_macro2::TokenStream {
    let mut rows = vec![];

    // First collect everything (user may have put rows out of order)
    for (button, construct_button) in infos.iter().zip(construct_variants) {
        if button.row as usize > rows.len() {
            rows.push(vec![])
        }
        rows[(button.row - 1) as usize].push(construct_button)
    }

    let mut construct_rows = vec![];

    for row in rows {
        construct_rows.push(quote! { vec![#(#row),*] });
    }

    let construct_keyboard =
        quote! {teloxide::types::InlineKeyboardMarkup::new(vec![#(#construct_rows),*])};

    let only_parameters: Vec<proc_macro2::TokenStream> =
        parameters.iter().filter_map(|p| p.clone()).collect();
    quote! {
        fn build_keyboard( #(#only_parameters),* ) -> ::std::result::Result<teloxide::types::InlineKeyboardMarkup, teloxide::utils::button::StringifyError> {
            Ok(#construct_keyboard)
        }
    }
}

fn get_enum_data(input: &DeriveInput) -> Result<&syn::DataEnum> {
    match &input.data {
        syn::Data::Enum(data) => Ok(data),
        _ => Err(compile_error("`InlineButtons` is only allowed for enums")),
    }
}
