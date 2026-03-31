use crate::{error::compile_error_at, Result};
use heck::ToSnakeCase;
use proc_macro2::Span;
use quote::quote;
use syn::{spanned::Spanned, Fields, FieldsNamed, FieldsUnnamed};

pub(crate) fn impl_keyboard_args(
    fields: &Fields,
    variant_span: Span,
    self_string_variant: String,
    self_variant: proc_macro2::TokenStream,
    button_text: String,
    button_url: Option<String>,
) -> Result<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)> {
    match fields {
        Fields::Unit => {
            if let Some(url) = button_url {
                Ok((
                    // url is checked in button.rs
                    quote! { Ok(teloxide::types::InlineKeyboardButton::url(#button_text, ::reqwest::Url::parse(#url).unwrap())) },
                    None,
                ))
            } else {
                Ok((quote! { #self_variant.build_button(#button_text) }, None))
            }
        }
        Fields::Unnamed(fields) => impl_keyboard_args_unnamed(
            fields,
            variant_span,
            self_string_variant,
            self_variant,
            button_text,
            button_url,
        ),
        Fields::Named(named) => impl_keyboard_args_named(
            named,
            variant_span,
            self_string_variant,
            self_variant,
            button_text,
            button_url,
        ),
    }
}

pub(crate) fn impl_keyboard_args_unnamed(
    data: &FieldsUnnamed,
    variant_span: Span,
    string_variant: String,
    variant: proc_macro2::TokenStream,
    button_text: String,
    button_url: Option<String>,
) -> Result<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)> {
    if button_url.is_some() {
        return Err(compile_error_at(
            "`url` can only exist on unit enum variants, please remove all fields.",
            variant_span,
        ));
    }
    let names = (0..data.unnamed.len()).map(|i| {
        // This is needed to avoid the scenario where multiple variants are unnamed
        syn::Ident::new(&format!("{}_{i}", string_variant.to_snake_case()), variant.span())
    });
    let types = data.unnamed.iter().map(|f| &f.ty);

    let all_names = quote! { #(#names),* };
    let all_types = quote! { #(#types),* };

    let construct_variant = quote! { #variant( #all_names ).build_button(#button_text) };
    let parameter = quote! { (#all_names): (#all_types) };

    Ok((construct_variant, Some(parameter)))
}

pub(crate) fn impl_keyboard_args_named(
    data: &FieldsNamed,
    variant_span: Span,
    string_variant: String,
    variant: proc_macro2::TokenStream,
    button_text: String,
    button_url: Option<String>,
) -> Result<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)> {
    if button_url.is_some() {
        return Err(compile_error_at(
            "`url` can only exist on unit enum variants, please remove all fields.",
            variant_span,
        ));
    }
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
        quote! { #variant { #all_names_construct_variant }.build_button(#button_text) };
    let parameter = quote! { (#all_names): (#all_types) };

    Ok((construct_variant, Some(parameter)))
}
