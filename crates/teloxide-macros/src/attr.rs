use crate::{error::compile_error_at, Result};

use proc_macro2::{Delimiter, Group, Span};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Parser},
    spanned::Spanned,
    Attribute, Ident, Lit, Path, Token,
};

pub(crate) fn fold_attrs<A, R>(
    attrs: &[Attribute],
    filter: fn(&Attribute) -> bool,
    parse: impl Fn(Attr) -> Result<R>,
    init: A,
    f: impl Fn(A, R) -> Result<A>,
) -> Result<A> {
    attrs
        .iter()
        .filter(|&a| filter(a))
        .flat_map(|attribute| {
            let Some(key) = attribute.path().get_ident().cloned() else {
                return vec![Err(compile_error_at("expected an ident", attribute.path().span()))];
            };

            // TODO: rewrite `Attrs` parser to syn 2 fully
            //
            // syn 2 has many breaking changes compared to syn 1.
            // At this place, code was adapted to support syn 2, ensuring that the same
            // tokens are passed to `Attrs::parse_with_key(input, key)`.
            //
            // The internal logic remains unchanged, similar to syn 1.
            // In the future, parsing should be rewritten to take advantage of syn 2’s
            // improvements for code deduplication and maintainability.

            let args = match attribute.parse_args() {
                Ok(v) => Group::new(Delimiter::Parenthesis, v).to_token_stream(),
                Err(_) => {
                    let value = match attribute.meta.require_name_value() {
                        Ok(v) => v.value.to_token_stream(),
                        Err(err) => return vec![Err(err.into())],
                    };

                    quote::quote! { = #value }
                }
            };

            match (|input: ParseStream<'_>| Attrs::parse_with_key(input, key)).parse2(args) {
                Ok(ok) => ok.0.into_iter().map(&parse).collect(),
                Err(err) => vec![Err(err.into())],
            }
        })
        .try_fold(init, |acc, r| f(acc, r?))
}

/// A helper to parse a set of attributes.
///
/// For example:
/// ```text
///   #[blahblah(key = "puff", value = 12, nope, inner(what = some::path))]
/// ```
///
/// The code above will produce
/// ```test
/// [
///     Attr { key: [key, blahblah], value: "puff" },
///     Attr { key: [value, blahblah], value: 12 },
///     Attr { key: [nope, blahblah], value: none },
///     Attr { key: [what, inner, blahblah], value: some::path },
/// ]
/// ```
#[derive(Default, Debug)]
struct Attrs(Vec<Attr>);

/// An attribute key-value pair.
///
/// For example:
/// ```text
///   #[blahblah(key = "puff", value = 12, nope)]
///              ^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^
/// ```
#[derive(Debug)]
pub(crate) struct Attr {
    /// The key captures the full "path" in the reverse order, for example here:
    ///
    /// ```text
    ///   #[blahblah(key = "puff")]
    ///              ^^^^^^^^^^^^
    /// ```
    ///
    /// The `key` will be `[key, blahblah]`. See [Attrs] for more examples.
    pub key: Vec<Ident>,
    pub value: AttrValue,
}

/// Value of an attribute.
///
/// For example:
/// ```text
///   #[blahblah(key = "puff", value = 12, nope)]
///                    ^^^^^^          ^^     ^-- (None pseudo-value)
/// ```
#[derive(Debug)]
pub(crate) enum AttrValue {
    Path(Path),
    Lit(Lit),
    Array(Vec<AttrValue>, Span),
    None(Span),
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> syn::Result<Attrs> {
        let key = input.parse::<Ident>()?;

        Attrs::parse_with_key(input, key)
    }
}

impl Attrs {
    fn parse_with_key(input: ParseStream, key: Ident) -> syn::Result<Attrs> {
        // Parse an attribute group
        let attrs = input.step(|cursor| {
            if let Some((group, _sp, next_cursor)) = cursor.group(Delimiter::Parenthesis) {
                if !next_cursor.eof() {
                    return Err(syn::Error::new(next_cursor.span(), "unexpected tokens"));
                }

                let mut attrs =
                    (|input: ParseStream<'_>| input.parse_terminated(Attrs::parse, Token![,]))
                        .parse(group.token_stream().into())?
                        .into_iter()
                        .reduce(|mut l, r| {
                            l.0.extend(r.0);
                            l
                        })
                        .unwrap_or_default();

                attrs.0.iter_mut().for_each(|attr| attr.key.push(key.clone()));

                Ok((Some(attrs), next_cursor))
            } else {
                Ok((None, *cursor))
            }
        })?;

        if let Some(attrs) = attrs {
            return Ok(attrs);
        }

        // Parse a single attribute
        let value = match input.peek(Token![=]) {
            true => {
                input.parse::<Token![=]>()?;
                input.parse::<AttrValue>()?
            }
            false => AttrValue::None(input.span()),
        };

        Ok(Attrs(vec![Attr { key: vec![key], value }]))
    }
}

impl Attr {
    pub(crate) fn span(&self) -> Span {
        self.key().span().join(self.value.span()).unwrap_or_else(|| self.key().span())
    }

    fn key(&self) -> &Ident {
        // It's an invariant of the type that `self.key` is non-empty
        self.key.first().unwrap()
    }
}

impl AttrValue {
    /// Unwraps this value if it's a string literal.
    pub fn expect_string(self) -> Result<String> {
        self.expect("a string", |this| match this {
            AttrValue::Lit(Lit::Str(s)) => Ok(s.value()),
            _ => Err(this),
        })
    }

    /// Unwraps this value if it's a nothing.
    pub fn expect_none(self, option_name: &str) -> Result<()> {
        match self {
            AttrValue::None(_) => Ok(()),
            _ => Err(compile_error_at(
                &format!("The {option_name} option should not have a value, remove it"),
                self.span(),
            )),
        }
    }

    /// Unwraps this value if it's a vector of `T`.
    /// ## Example
    /// ```text
    ///  #[command(some = [1, 2, 3])]
    ///                   ^^^^^^^^^
    ///              this value will be parsed as a vector of integers
    /// ```
    pub fn expect_array(self) -> Result<Vec<Self>> {
        self.expect("an array", |this| match this {
            AttrValue::Array(a, _) => Ok(a),
            _ => Err(this),
        })
    }

    // /// Unwraps this value if it's a path.
    // pub fn expect_path(self) -> Result<Path> {
    //     self.expect("a path", |this| match this {
    //         AttrValue::Path(p) => Ok(p),
    //         _ => Err(this),
    //     })
    // }

    pub fn expect<T>(self, expected: &str, f: impl FnOnce(Self) -> Result<T, Self>) -> Result<T> {
        f(self).map_err(|this| {
            compile_error_at(&format!("expected {expected}, found {}", this.descr()), this.span())
        })
    }

    fn descr(&self) -> &'static str {
        use Lit::*;

        match self {
            Self::None(_) => "nothing",
            Self::Lit(l) => match l {
                Str(_) | ByteStr(_) | CStr(_) => "a string",
                Char(_) => "a character",
                Byte(_) | Int(_) => "an integer",
                Float(_) => "a floating point integer",
                Bool(_) => "a boolean",
                Verbatim(_) => ":shrug:",
                _ => ":mag:",
            },
            Self::Array(_, _) => "an array",
            Self::Path(_) => "a path",
        }
    }

    /// Returns span of the value
    ///
    /// ```text
    ///   #[blahblah(key = "puff", value = 12, nope )]
    ///                    ^^^^^^          ^^      ^
    /// ```
    pub fn span(&self) -> Span {
        match self {
            Self::Path(p) => p.span(),
            Self::Lit(l) => l.span(),
            Self::None(sp) => *sp,
            Self::Array(_, sp) => *sp,
        }
    }
}

impl Parse for AttrValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Lit) {
            input.parse::<Lit>().map(AttrValue::Lit)
        } else if input.peek(syn::token::Bracket) {
            let content;
            let array_span = syn::bracketed!(content in input).span;
            let array = content.parse_terminated(AttrValue::parse, Token![,])?;
            Ok(AttrValue::Array(array.into_iter().collect(), array_span.span()))
        } else {
            Ok(AttrValue::Path(
                input
                    .parse::<Path>()
                    .map_err(|_| syn::Error::new(input.span(), "Unexpected token"))?,
            ))
        }
    }
}
