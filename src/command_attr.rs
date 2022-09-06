use crate::{
    attr::{fold_attrs, Attr},
    error::compile_error_at,
    fields_parse::ParserType,
    rename_rules::RenameRule,
    Result,
};

use proc_macro2::Span;
use syn::Attribute;

/// Attributes for `BotCommands` derive macro.
pub(crate) struct CommandAttrs {
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub rename_rule: Option<RenameRule>,
    pub parser: Option<ParserType>,
    pub separator: Option<String>,
}

/// An attribute for `BotCommands` derive macro.
pub(crate) struct CommandAttr {
    kind: CommandAttrKind,
    sp: Span,
}

pub(crate) enum CommandAttrKind {
    Prefix(String),
    Description(String),
    Rename(RenameRule),
    ParseWith(ParserType),
    Separator(String),
}

impl CommandAttrs {
    pub fn from_attributes(attributes: &[Attribute]) -> Result<Self> {
        use CommandAttrKind::*;

        fold_attrs(
            attributes,
            is_command_attribute,
            CommandAttr::parse,
            Self {
                prefix: None,
                description: None,
                rename_rule: None,
                parser: None,
                separator: None,
            },
            |mut this, attr| {
                fn insert<T>(
                    opt: &mut Option<T>,
                    x: T,
                    sp: Span,
                ) -> Result<()> {
                    match opt {
                        slot @ None => {
                            *slot = Some(x);
                            Ok(())
                        }
                        Some(_) => {
                            Err(compile_error_at("duplicate attribute", sp))
                        }
                    }
                }

                match attr.kind {
                    Prefix(p) => insert(&mut this.prefix, p, attr.sp),
                    Description(d) => insert(&mut this.description, d, attr.sp),
                    Rename(r) => insert(&mut this.rename_rule, r, attr.sp),
                    ParseWith(p) => insert(&mut this.parser, p, attr.sp),
                    Separator(s) => insert(&mut this.separator, s, attr.sp),
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
            "rename" => Rename(
                value.expect_string().and_then(|r| RenameRule::parse(&r))?,
            ),
            "parse_with" => {
                ParseWith(value.expect_string().map(|p| ParserType::parse(&p))?)
            }
            "separator" => Separator(value.expect_string()?),
            _ => {
                return Err(compile_error_at(
                    "unexpected attribute name (expected one of `prefix`, \
                     `description`, `rename`, `parse_with` and `separator`",
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
