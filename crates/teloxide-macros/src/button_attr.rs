use crate::{
    attr::{fold_attrs, Attr},
    error::compile_error_at,
    Result,
};

use proc_macro2::Span;
use syn::Attribute;

/// All attributes that can be used for `derive(InlineButtons)`
pub(crate) struct ButtonAttrs {
    pub rename: Option<(String, Span)>,
    pub text: Option<(String, Span)>,
    pub row: Option<(u8, Span)>,
    pub url: Option<(String, Span)>,
    pub login_url: Option<(String, Span)>,
    pub webapp_url: Option<(String, Span)>,
    pub switch_inline_query: Option<(String, Span)>,
    pub switch_inline_query_current_chat: Option<(String, Span)>,
    pub copy_text: Option<(String, Span)>,
    pub game: Option<(bool, Span)>,
    pub pay: Option<(bool, Span)>,
    pub fields_separator: Option<(String, Span)>,
}

/// A single k/v attribute for `InlineButtons` derive macro.
///
/// Similar to [`crate::command_attr::CommandAttr`]
struct ButtonAttr {
    kind: ButtonAttrKind,
    sp: Span,
}

/// Kind of [`ButtonAttr`].
enum ButtonAttrKind {
    Rename(String),
    Text(String),
    Row(u8),
    Url(String),
    LoginUrl(String),
    WebAppUrl(String),
    SwitchInlineQuery(String),
    SwitchInlineQueryCurrentChat(String),
    CopyText(String),
    Game(bool),
    Pay(bool),
    FieldsSeparator(String),
}

impl ButtonAttrs {
    pub fn from_attributes(attributes: &[Attribute]) -> Result<Self> {
        use ButtonAttrKind::*;

        fold_attrs(
            attributes,
            is_button_attribute,
            ButtonAttr::parse,
            Self {
                rename: None,
                text: None,
                row: None,
                url: None,
                login_url: None,
                webapp_url: None,
                switch_inline_query: None,
                switch_inline_query_current_chat: None,
                copy_text: None,
                game: None,
                pay: None,
                fields_separator: None,
            },
            |mut this, attr| {
                fn insert<T>(opt: &mut Option<(T, Span)>, x: T, sp: Span) -> Result<()> {
                    match opt {
                        slot @ None => {
                            *slot = Some((x, sp));
                            Ok(())
                        }
                        Some(_) => Err(compile_error_at("duplicate attribute", sp)),
                    }
                }

                match attr.kind {
                    Rename(r) => insert(&mut this.rename, r, attr.sp),
                    FieldsSeparator(s) => insert(&mut this.fields_separator, s, attr.sp),
                    Text(t) => insert(&mut this.text, t, attr.sp),
                    Row(r) => insert(&mut this.row, r, attr.sp),
                    Url(u) => insert(&mut this.url, u, attr.sp),
                    LoginUrl(l) => insert(&mut this.login_url, l, attr.sp),
                    WebAppUrl(w) => insert(&mut this.webapp_url, w, attr.sp),
                    SwitchInlineQuery(s) => insert(&mut this.switch_inline_query, s, attr.sp),
                    SwitchInlineQueryCurrentChat(s) => {
                        insert(&mut this.switch_inline_query_current_chat, s, attr.sp)
                    }
                    CopyText(c) => insert(&mut this.copy_text, c, attr.sp),
                    Game(g) => insert(&mut this.game, g, attr.sp),
                    Pay(p) => insert(&mut this.pay, p, attr.sp),
                }?;

                Ok(this)
            },
        )
    }
}

impl ButtonAttr {
    fn parse(attr: Attr) -> Result<Self> {
        use ButtonAttrKind::*;

        let sp = attr.span();
        let Attr { mut key, value } = attr;

        let outermost_key = key.pop().unwrap(); // `Attr`'s invariants ensure `key.len() > 0`

        let kind = match &*outermost_key.to_string() {
            "button" => {
                let Some(attr) = key.pop() else {
                    return Err(compile_error_at(
                        "expected an attribute name",
                        outermost_key.span(),
                    ));
                };

                if let Some(unexpected_key) = key.last() {
                    return Err(compile_error_at(
                        &format!("{attr} can't have nested attributes"),
                        unexpected_key.span(),
                    ));
                }

                match &*attr.to_string() {
                    "rename" => Rename(value.expect_string()?),
                    "text" => Text(value.expect_string()?),
                    "row" => Row(value.expect_u8()?),
                    "url" => Url(value.expect_string()?),
                    "login_url" => LoginUrl(value.expect_string()?),
                    "webapp_url" => WebAppUrl(value.expect_string()?),
                    "switch_inline_query" => SwitchInlineQuery(value.expect_string()?),
                    "switch_inline_query_current_chat" => {
                        SwitchInlineQueryCurrentChat(value.expect_string()?)
                    }
                    "copy_text" => CopyText(value.expect_string()?),
                    "game" => Game(value.expect_bool()?),
                    "pay" => Pay(value.expect_bool()?),
                    "fields_separator" => FieldsSeparator(value.expect_string()?),
                    _ => {
                        return Err(compile_error_at(
                            "unexpected attribute name (expected one of `rename`, `text`, `row`, \
                             `url`, `login_url`, `webapp_url`, `switch_inline_query`, \
                             `switch_inline_query_current_chat`, `copy_text`, `game`, `pay` or \
                             `fields_separator`)",
                            attr.span(),
                        ))
                    }
                }
            }

            _ => {
                return Err(compile_error_at(
                    "unexpected attribute (expected `button`)",
                    outermost_key.span(),
                ))
            }
        };

        Ok(Self { kind, sp })
    }
}

fn is_button_attribute(a: &Attribute) -> bool {
    matches!(a.path().get_ident(), Some(ident) if ident == "button")
}
