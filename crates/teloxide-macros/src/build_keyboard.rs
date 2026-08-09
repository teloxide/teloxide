use crate::{button::Button, Result};
use heck::ToSnakeCase;
use quote::quote;
use syn::{spanned::Spanned, Fields, FieldsNamed, FieldsUnnamed};

pub(crate) fn impl_keyboard_args(
    fields: &Fields,
    self_string_variant: String,
    self_variant: proc_macro2::TokenStream,
    button: &Button,
) -> Result<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)> {
    let button_text = &button.text;
    match fields {
        Fields::Unit => impl_keyboard_args_unit(button_text, self_variant, button),
        Fields::Unnamed(fields) => {
            impl_keyboard_args_unnamed(fields, self_string_variant, self_variant, button_text)
        }
        Fields::Named(named) => {
            impl_keyboard_args_named(named, self_string_variant, self_variant, button_text)
        }
    }
}

pub(crate) fn impl_keyboard_args_unit(
    button_text: &String,
    variant: proc_macro2::TokenStream,
    button: &Button,
) -> Result<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)> {
    if let Some(url) = &button.url {
        Ok((
            // url is checked in button.rs
            quote! { teloxide::types::InlineKeyboardButton::url(#button_text, ::reqwest::Url::parse(#url).unwrap()) },
            None,
        ))
    } else if let Some(login_url) = &button.login_url {
        Ok((
            // url is checked in button.rs
            quote! {
                teloxide::types::InlineKeyboardButton::login(
                    #button_text,
                    teloxide::types::LoginUrl {
                        url: ::reqwest::Url::parse(#login_url).unwrap(),
                        forward_text: None,
                        bot_username: None,
                        request_write_access: None
                    }
                )
            },
            None,
        ))
    } else if let Some(webapp_url) = &button.webapp_url {
        Ok((
            // url is checked in button.rs
            quote! {
                teloxide::types::InlineKeyboardButton::web_app(
                    #button_text,
                    teloxide::types::WebAppInfo {
                        url: ::reqwest::Url::parse(#webapp_url).unwrap(),
                    }
                )
            },
            None,
        ))
    } else if let Some(switch_inline_query) = &button.switch_inline_query {
        Ok((
            quote! {
                teloxide::types::InlineKeyboardButton::switch_inline_query(
                    #button_text,
                    #switch_inline_query
                )
            },
            None,
        ))
    } else if let Some(switch_inline_query_current_chat) = &button.switch_inline_query_current_chat
    {
        Ok((
            quote! {
                teloxide::types::InlineKeyboardButton::switch_inline_query_current_chat(
                    #button_text,
                    #switch_inline_query_current_chat
                )
            },
            None,
        ))
    } else if let Some(copy_text) = &button.copy_text {
        Ok((
            quote! {
                teloxide::types::InlineKeyboardButton::copy_text_button(
                    #button_text,
                    teloxide::types::CopyTextButton {
                        text: #copy_text.to_owned()
                    }
                )
            },
            None,
        ))
    } else if button.game.is_some() && button.game.unwrap() {
        Ok((
            quote! {
                teloxide::types::InlineKeyboardButton::callback_game(
                    #button_text,
                    teloxide::types::CallbackGame {}
                )
            },
            None,
        ))
    } else if button.pay.is_some() && button.pay.unwrap() {
        Ok((
            quote! {
                teloxide::types::InlineKeyboardButton::pay(
                    #button_text,
                )
            },
            None,
        ))
    } else {
        Ok((quote! { #variant.build_button(#button_text)? }, None))
    }
}

pub(crate) fn impl_keyboard_args_unnamed(
    data: &FieldsUnnamed,
    string_variant: String,
    variant: proc_macro2::TokenStream,
    button_text: &String,
) -> Result<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)> {
    let names = (0..data.unnamed.len()).map(|i| {
        // This is needed to avoid the scenario where multiple variants are unnamed
        syn::Ident::new(&format!("{}_{i}", string_variant.to_snake_case()), variant.span())
    });
    let types = data.unnamed.iter().map(|f| &f.ty);

    let all_names = quote! { #(#names),* };
    let all_types = quote! { #(#types),* };

    let construct_variant = quote! { #variant( #all_names ).build_button(#button_text)? };
    let parameter = quote! { (#all_names): (#all_types) };

    Ok((construct_variant, Some(parameter)))
}

pub(crate) fn impl_keyboard_args_named(
    data: &FieldsNamed,
    string_variant: String,
    variant: proc_macro2::TokenStream,
    button_text: &String,
) -> Result<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)> {
    let names = data.named.iter().map(|f| {
        (
            f.ident.as_ref().unwrap(),
            syn::Ident::new(
                &format!("{}_{}", string_variant.to_snake_case(), f.ident.as_ref().unwrap()),
                variant.span(),
            ),
        )
    });
    let types = data.named.iter().map(|f| &f.ty);

    let mut all_names = vec![];
    let mut all_names_construct_variant = vec![];
    for (original_name, new_name) in names {
        // This is needed to avoid the scenario where multiple variants have the same
        // variable
        all_names_construct_variant.push(quote! { #original_name : #new_name });
        all_names.push(quote! { #new_name });
    }

    let all_names = quote! {#(#all_names),*};
    let all_names_construct_variant = quote! {#(#all_names_construct_variant),*};

    let all_types = quote! { #(#types),* };

    let construct_variant =
        quote! { #variant { #all_names_construct_variant }.build_button(#button_text)? };
    let parameter = quote! { (#all_names): (#all_types) };

    Ok((construct_variant, Some(parameter)))
}
