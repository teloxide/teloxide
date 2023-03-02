use crate::{
    attr::{fold_attrs, Attr},
    error::compile_error_at,
    fields_parse::ParserType,
    rename_rules::RenameRule,
    Result,
};

use proc_macro2::Span;
use quote::quote_spanned;
use syn::{parse::Parser, spanned::Spanned, Attribute};

/// All attributes that can be used for `derive(BotCommands)`
pub(crate) struct CommandAttrs {
    pub prefix: Option<(String, Span)>,
    pub description: Option<(String, Span)>,
    pub rename_rule: Option<(RenameRule, Span)>,
    pub rename: Option<(String, Span)>,
    pub parser: Option<(ParserType, Span)>,
    pub separator: Option<(String, Span)>,
    pub hide: Option<((), Span)>,
}

/// A single k/v attribute for `BotCommands` derive macro.
///
/// For example:
/// ```text
///   #[command(prefix = "!", rename_rule = "snake_case")]
///            /^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^---- CommandAttr { kind: RenameRule(SnakeCase) }
///            |
///            CommandAttr { kind: Prefix("!") }
/// ```
struct CommandAttr {
    kind: CommandAttrKind,
    sp: Span,
}

/// Kind of [`CommandAttr`].
enum CommandAttrKind {
    Prefix(String),
    Description(String),
    RenameRule(RenameRule),
    Rename(String),
    ParseWith(ParserType),
    Separator(String),
    Hide,
}

impl CommandAttrs {
    pub fn from_attributes(attributes: &[Attribute]) -> Result<Self> {
        use CommandAttrKind::*;
        // Convert the `doc` attribute into `command(description = "...")`
        let attributes = attributes.iter().map(|attr| {
            if attr.path.is_ident("doc") {
                // Extract the token literal from the doc attribute.
                let description = attr
                    .tokens
                    .clone()
                    .into_iter()
                    .nth(1)
                    .map(|t| {
                        // remove first and last quotes only, is expected to be a string literal
                        let mut s = t.to_string();
                        s.remove(0);
                        s.pop();
                        s.trim().replace(r"\\n", "\n")
                    })
                    .unwrap_or_default();
                // Convert the doc attribute into a command description attribute.
                let sp = attr.span();
                let attr = Attribute::parse_outer
                    .parse2(quote_spanned!(sp => #[command(description = #description)]))
                    .unwrap();
                attr.into_iter().next().unwrap()
            } else {
                attr.clone()
            }
        });

        fold_attrs(
            attributes,
            is_command_attribute,
            CommandAttr::parse,
            Self {
                prefix: None,
                description: None,
                rename_rule: None,
                rename: None,
                parser: None,
                separator: None,
                hide: None,
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
                    Prefix(p) => insert(&mut this.prefix, p, attr.sp),
                    Description(d) => insert(&mut this.description, d, attr.sp),
                    RenameRule(r) => insert(&mut this.rename_rule, r, attr.sp),
                    Rename(r) => insert(&mut this.rename, r, attr.sp),
                    ParseWith(p) => insert(&mut this.parser, p, attr.sp),
                    Separator(s) => insert(&mut this.separator, s, attr.sp),
                    Hide => insert(&mut this.hide, (), attr.sp),
                }?;

                Ok(this)
            },
        )
    }
}

impl CommandAttr {
    fn parse(attr: Attr) -> Result<Self> {
        use CommandAttrKind::*;

        let sp = attr.span();
        let Attr { key, value } = attr;
        let kind = match &*key.to_string() {
            "prefix" => Prefix(value.expect_string()?),
            "description" => Description(value.expect_string()?),
            "rename_rule" => {
                RenameRule(value.expect_string().and_then(|r| self::RenameRule::parse(&r))?)
            }
            "rename" => Rename(value.expect_string()?),
            "parse_with" => ParseWith(ParserType::parse(value)?),
            "separator" => Separator(value.expect_string()?),
            "hide" => value.expect_none("hide").map(|_| Hide)?,
            _ => {
                return Err(compile_error_at(
                    "unexpected attribute name (expected one of `prefix`, `description`, \
                     `rename`, `parse_with`, `separator` and `hide`",
                    key.span(),
                ))
            }
        };

        Ok(Self { kind, sp })
    }
}

fn is_command_attribute(a: &Attribute) -> bool {
    match a.path.get_ident() {
        Some(ident) => ident == "command",
        _ => false,
    }
}
