// TODO: refactor this shit.

mod attr;
mod command;
mod command_enum;
mod fields_parse;
mod rename_rules;

extern crate proc_macro;
extern crate quote;
extern crate syn;
use crate::{
    attr::{Attr, VecAttrs},
    command::Command,
    command_enum::CommandEnum,
    fields_parse::{impl_parse_args_named, impl_parse_args_unnamed},
};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, DeriveInput, Fields, FnArg, ItemEnum, ItemFn,
    ReturnType, Type,
};

use std::fmt::Write;

/// The docs is below.
///
/// The only accepted form at the current moment is `#[teloxide(subtransition)]`
/// on an asynchronous function. Either this:
///
/// ```no_compile
/// #[teloxide(subtransition)]
/// async fn my_transition(state: MyState, cx: TransitionIn, ans: T) -> TransitionOut<MyDialogue> {
///     todo!()
/// }
/// ```
///
/// Or this:
///
/// ```no_compile
/// #[teloxide(subtransition)]
/// async fn my_transition(state: MyState, cx: TransitionIn) -> TransitionOut<MyDialogue> {
///     todo!()
/// }
/// ```
///
/// Notice the presence/absence of `ans: T`. In the first case, it generates
/// `impl SubTransition for MyState { type Aux = T; type Dialogue = MyDialogue;
/// ... }`. In the second case, the `Aux` type defaults to `()`.
#[proc_macro_attribute]
pub fn teloxide(attr: TokenStream, item: TokenStream) -> TokenStream {
    match attr.to_string().as_ref() {
        "subtransition" => {
            let item_cloned = item.clone();
            let input = parse_macro_input!(item as ItemFn);
            let params = input.sig.inputs.iter().collect::<Vec<&FnArg>>();

            if params.len() != 2 && params.len() != 3 {
                panic!(
                    "An transition function must accept two/three parameters: \
                     a state type, TransitionIn, and an optional data."
                );
            }

            // This is actually used inside the quite! { ... } below.
            #[allow(unused_variables)]
            let state_type = match params[0] {
                FnArg::Typed(pat_type) => &pat_type.ty,
                _ => unreachable!(),
            };
            let fn_name = input.sig.ident;
            let fn_return_type = match input.sig.output {
                ReturnType::Type(_arrow, _type) => _type,
                _ => panic!(
                    "A subtransition must return TransitionOut<your dialogue \
                     type>"
                ),
            };
            let aux_param_type = match params.get(2) {
                Some(data_param_type) => match *data_param_type {
                    FnArg::Typed(typed) => typed.ty.clone(),
                    _ => unreachable!(),
                },
                None => {
                    let unit_type = proc_macro::TokenStream::from(quote! {()});
                    Box::new(parse_macro_input!(unit_type as Type))
                }
            };
            let call_fn = match params.get(2) {
                Some(_) => {
                    quote! {  #fn_name(self, cx, aux) }
                }
                None => quote! { #fn_name(self, cx) },
            };

            let item = proc_macro2::TokenStream::from(item_cloned);

            let impl_transition = quote! {
                impl teloxide::dispatching::dialogue::Subtransition for #state_type {
                    type Aux = #aux_param_type;
                    type Dialogue = <#fn_return_type as teloxide::dispatching::dialogue::SubtransitionOutputType>::Output;

                    fn react(self, cx: teloxide::dispatching::dialogue::TransitionIn, aux: #aux_param_type)
                        -> futures::future::BoxFuture<'static, #fn_return_type> {
                                #item
                                futures::future::FutureExt::boxed(#call_fn)
                            }
                }
            };

            impl_transition.into()
        }
        _ => {
            panic!("Unrecognised attribute '{}'", attr);
        }
    }
}

/// The docs is below.
///
/// All the variants must be of the form `VariantName(MyStateType)`, and
/// `MyStateType` must implement `Subtransition`. All `MyStateType`s must have
/// the same `Subtransition::Aux`, which will be also used in the generated
/// implementation.
#[proc_macro_derive(Transition)]
pub fn derive_transition(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    let mut dispatch_fn = "".to_owned();

    let enum_name = input.ident;
    let field_type_of_first_variant =
        match &input.variants.iter().next().unwrap().fields {
            Fields::Unnamed(fields) => {
                fields
                    .unnamed
                    .iter()
                    .next()
                    // .unwrap() because empty enumerations are not yet allowed
                    // in stable Rust.
                    .unwrap()
                    .ty
                    .to_token_stream()
                    .to_string()
            }
            _ => panic!("Only one unnamed field per variant is allowed"),
        };

    write!(
        dispatch_fn,
        "impl teloxide::dispatching::dialogue::Transition for {1} {{type Aux \
         = <{0} as teloxide::dispatching::dialogue::Subtransition>::Aux;fn \
         react(self, cx: teloxide::dispatching::dialogue::TransitionIn, aux: \
         Self::Aux) -> futures::future::BoxFuture<'static, \
         teloxide::dispatching::dialogue::TransitionOut<Self>> {{ \
         futures::future::FutureExt::boxed(async move {{ match self {{",
        field_type_of_first_variant, enum_name
    )
    .unwrap();

    for variant in input.variants.iter() {
        write!(
            dispatch_fn,
            "{}::{}(state) => \
             teloxide::dispatching::dialogue::Subtransition::react(state, cx, \
             aux).await,",
            enum_name, variant.ident
        )
        .unwrap();
    }

    write!(dispatch_fn, "}} }}) }} }}").unwrap();
    dispatch_fn.parse().unwrap()
}

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

    let variants: Vec<&syn::Variant> = data_enum.variants.iter().collect();

    let mut variant_infos = vec![];
    for variant in variants.iter() {
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

    let mut vec_impl_create = vec![];
    for (variant, info) in variants.iter().zip(variant_infos.iter()) {
        let var = &variant.ident;
        let variantt = quote! { Self::#var };
        match &variant.fields {
            Fields::Unnamed(fields) => {
                let parser =
                    info.parser.as_ref().unwrap_or(&command_enum.parser_type);
                vec_impl_create
                    .push(impl_parse_args_unnamed(fields, variantt, parser));
            }
            Fields::Unit => {
                vec_impl_create.push(variantt);
            }
            Fields::Named(named) => {
                let parser =
                    info.parser.as_ref().unwrap_or(&command_enum.parser_type);
                vec_impl_create
                    .push(impl_parse_args_named(named, variantt, parser));
            }
        }
    }

    let ident = &input.ident;

    let fn_descriptions = impl_descriptions(&variant_infos, &command_enum);
    let fn_parse = impl_parse(&variant_infos, &command_enum, &vec_impl_create);

    let trait_impl = quote! {
        impl BotCommand for #ident {
            #fn_descriptions
            #fn_parse
        }
    };

    TokenStream::from(trait_impl)
}

fn impl_descriptions(
    infos: &[Command],
    global: &CommandEnum,
) -> quote::__private::TokenStream {
    let global_description = if let Some(s) = &global.description {
        quote! { #s, "\n", }
    } else {
        quote! {}
    };
    let command = infos.iter().map(|c| c.get_matched_value(global));
    let description =
        infos.iter().map(|info| {
            info.description
                .as_deref()
                .map(|e| {
                    if e != "off" {
                        format!(" - {}", e)
                    } else {
                        e.to_string()
                    }
                })
                .unwrap_or_default()
        });
    let result_iter = command.zip(description).map(|(c, d)| {
        if &d == "off" {
            quote! {}
        } else {
            quote! { #c, #d, '\n', }
        }
    });

    quote! {
        fn descriptions() -> String {
            std::concat!(#global_description #(#result_iter)*).to_string()
        }
    }
}

fn impl_parse(
    infos: &[Command],
    global: &CommandEnum,
    variants_initialization: &[quote::__private::TokenStream],
) -> quote::__private::TokenStream {
    let matching_values = infos.iter().map(|c| c.get_matched_value(global));

    quote! {
         fn parse<N>(s: &str, bot_name: N) -> Result<Self, teloxide::utils::command::ParseError>
         where
              N: Into<String>
         {
              use std::str::FromStr;
              use teloxide::utils::command::ParseError;

              let mut words = s.splitn(2, ' ');
              let mut splited = words.next().expect("First item will be always.").split('@');
              let command_raw = splited.next().expect("First item will be always.");
              let bot = splited.next();
              let bot_name = bot_name.into();
              match bot {
                  Some(name) if name == bot_name => {}
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

fn get_enum_data(input: &DeriveInput) -> Result<&syn::DataEnum, TokenStream> {
    match &input.data {
        syn::Data::Enum(data) => Ok(data),
        _ => Err(compile_error("TelegramBotCommand allowed only for enums")),
    }
}

fn parse_attributes(
    input: &[syn::Attribute],
) -> Result<Vec<Attr>, TokenStream> {
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
