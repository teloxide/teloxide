use crate::{error::compile_error_at, Result};

use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseBuffer, ParseStream},
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
            // FIXME: don't allocate here
            let attrs = match attribute.parse_args_with(|input: &ParseBuffer| {
                input.parse_terminated::<_, Token![,]>(Attr::parse)
            }) {
                Ok(ok) => ok,
                Err(err) => return vec![Err(err.into())],
            };

            attrs.into_iter().map(&parse).collect()
        })
        .try_fold(init, |acc, r| r.and_then(|r| f(acc, r)))
}

/// An attribute key-value pair.
///
/// For example:
/// ```text
///   #[blahblah(key = "puff", value = 12, nope)]
///              ^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^
/// ```
pub(crate) struct Attr {
    pub key: Ident,
    pub value: AttrValue,
}

/// Value of an attribute.
///
/// For example:
/// ```text
///   #[blahblah(key = "puff", value = 12, nope)]
///                    ^^^^^^          ^^     ^-- (None pseudo-value)
/// ```
pub(crate) enum AttrValue {
    Path(Path),
    Lit(Lit),
    None(Span),
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse::<Ident>()?;

        let value = match input.peek(Token![=]) {
            true => {
                input.parse::<Token![=]>()?;
                input.parse::<AttrValue>()?
            }
            false => AttrValue::None(input.span()),
        };

        Ok(Self { key, value })
    }
}

impl Attr {
    pub(crate) fn span(&self) -> Span {
        self.key.span().join(self.value.span()).unwrap_or_else(|| self.key.span())
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
                Str(_) | ByteStr(_) => "a string",
                Char(_) => "a character",
                Byte(_) | Int(_) => "an integer",
                Float(_) => "a floating point integer",
                Bool(_) => "a boolean",
                Verbatim(_) => ":shrug:",
            },
            Self::Path(_) => "a path",
        }
    }

    /// Returns span of the value
    ///
    /// ```text
    ///   #[blahblah(key = "puff", value = 12, nope )]
    ///                    ^^^^^^          ^^      ^
    /// ```
    fn span(&self) -> Span {
        match self {
            Self::Path(p) => p.span(),
            Self::Lit(l) => l.span(),
            Self::None(sp) => *sp,
        }
    }
}

impl Parse for AttrValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let this = match input.peek(Lit) {
            true => Self::Lit(input.parse()?),
            false => Self::Path(input.parse()?),
        };

        Ok(this)
    }
}
