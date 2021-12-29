use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Fields, GenericParam, ItemEnum, Path, Type,
};

pub fn expand(item: ItemEnum) -> Result<TokenStream, syn::Error> {
    let enum_ident = &item.ident;
    let self_params_with_bounds = {
        let params = &item.generics.params;
        if params.len() != 0 {
            quote! { < #params > }
        } else {
            quote! {}
        }
    };
    let self_params = {
        let params = &item.generics.params;
        if params.len() != 0 {
            let mut params = quote! { < };
            item.generics.params.iter().for_each(|param| match param {
                GenericParam::Type(ty) => {
                    let ident = &ty.ident;
                    params.extend(quote! { #ident, });
                }
                GenericParam::Lifetime(li) => {
                    let li = &li.lifetime;
                    params.extend(quote! { #li, })
                }
                GenericParam::Const(_par) => todo!(),
            });
            params.extend(quote! { > });
            params
        } else {
            quote! {}
        }
    };
    let where_clause = match item.generics.where_clause.clone() {
        Some(mut clause) => {
            let predicate = quote! { Self: Clone + Send + Sync + 'static };
            clause.predicates.push(syn::parse2(predicate).unwrap());
            Some(clause)
        }
        x => x,
    };
    let out = parse_out_type(item.ident.span(), &item.attrs)?;

    let mut branches = quote! {};
    for variant in item.variants.iter() {
        let handler = {
            let handler_attr = variant
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident("handler"))
                .ok_or_else(|| {
                    syn::Error::new(
                        variant.span(),
                        "Expected `handler` attribute.",
                    )
                })?;
            handler_attr.parse_args::<HandlerAttr>()?
        };

        branches.extend(match &variant.fields {
            Fields::Named(_) => {
                return Err(syn::Error::new(
                    variant.span(),
                    "Named fields does not allowed",
                ))
            }
            Fields::Unnamed(fields) => match fields.unnamed.len() {
                1 => create_branch_one_field(
                    &enum_ident,
                    &self_params,
                    &variant.ident,
                    &handler.func,
                ),
                len => {
                    return Err(syn::Error::new(
                        fields.span(),
                        format!("Expected 1 field, found {}", len),
                    ));
                }
            },
            Fields::Unit => create_branch_no_fields(
                &enum_ident,
                &self_params,
                &variant.ident,
                &handler.func,
            ),
        });
    }

    Ok(quote! {const _: () = {
        fn assert_clone<T: Clone>() {}

        use teloxide::dptree;
        use teloxide::dispatching2::dialogue::Dialogue;

        impl #self_params_with_bounds teloxide::dispatching2::HandlerFactory for #enum_ident #self_params #where_clause {
        type Out = #out;

        fn handler() -> dptree::Handler<'static, dptree::di::DependencyMap, Self::Out> {
            assert_clone::<#enum_ident #self_params>();

            dptree::entry()
                #branches
        }
    }
    };})
}

fn create_branch_no_fields(
    state: &Ident,
    state_generics: impl ToTokens,
    kind: &Ident,
    handler: &Path,
) -> TokenStream {
    quote! {
        .branch(
            dptree::filter(|state: #state #state_generics| async move {
                match state { #state::#kind => true, _ => false }
            }).endpoint(#handler)
        )
    }
}

fn create_branch_one_field(
    state: &Ident,
    state_generics: impl ToTokens,
    kind: &Ident,
    handler: &Path,
) -> TokenStream {
    quote! {
        .branch(
            dptree::filter_map(|state: #state #state_generics| async move {
                match state { #state::#kind(arg) => Some(arg), _ => None }
            }).endpoint(#handler)
        )
    }
}

fn parse_out_type(
    span: Span,
    attrs: &[syn::Attribute],
) -> Result<Type, syn::Error> {
    let mut out = None;
    for x in attrs {
        if x.path.is_ident("out") {
            out = Some(x.parse_args::<Type>()?);
        }
    }
    if let Some(out) = out {
        return Ok(out);
    }
    Err(syn::Error::new(
        span,
        "You must specify #[out()] argument in which declare output type of \
         handlers. For example, #[out(Result<(), Error>)]",
    ))
}

pub struct HandlerAttr {
    func: Path,
}

impl Parse for HandlerAttr {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self { func: input.parse::<Path>()? })
    }
}
