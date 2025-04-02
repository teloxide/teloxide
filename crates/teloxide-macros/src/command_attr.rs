use crate::{
    attr::{fold_attrs, Attr, AttrValue},
    error::compile_error_at,
    fields_parse::ParserType,
    rename_rules::RenameRule,
    Result,
};

use proc_macro2::Span;
use syn::Attribute;

/// All attributes that can be used for `derive(BotCommands)`
pub(crate) struct CommandAttrs {
    pub prefix: Option<(String, Span)>,
    /// The bool is true if the description contains a doc comment
    pub description: Option<(String, bool, Span)>,
    pub rename_rule: Option<(RenameRule, Span)>,
    pub rename: Option<(String, Span)>,
    pub aliases: Option<(Vec<String>, Span)>,
    pub parser: Option<(ParserType, Span)>,
    pub separator: Option<(String, Span)>,
    pub command_separator: Option<(String, Span)>,
    pub hide: Option<((), Span)>,
    pub hide_aliases: Option<((), Span)>,
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
    /// Description of the command. and if its doc comment or not
    Description(String, bool),
    RenameRule(RenameRule),
    Rename(String),
    Aliases(Vec<String>),
    ParseWith(ParserType),
    Separator(String),
    CommandSeparator(String),
    Hide,
    HideAliases,
}

impl CommandAttrs {
    pub fn from_attributes(attributes: &[Attribute]) -> Result<Self> {
        use CommandAttrKind::*;

        fold_attrs(
            attributes,
            |attr| is_command_attribute(attr) || is_doc_comment(attr),
            CommandAttr::parse,
            Self {
                prefix: None,
                description: None,
                rename_rule: None,
                rename: None,
                aliases: None,
                parser: None,
                separator: None,
                command_separator: None,
                hide: None,
                hide_aliases: None,
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

                fn join_string(opt: &mut Option<(String, bool, Span)>, new_str: &str, sp: Span) {
                    match opt {
                        slot @ None => {
                            *slot = Some((new_str.to_owned(), false, sp));
                        }
                        Some((old_str, ..)) => {
                            *old_str = format!("{old_str}\n{new_str}");
                        }
                    }
                }

                match attr.kind {
                    Prefix(p) => insert(&mut this.prefix, p, attr.sp),
                    Description(d, is_doc) => {
                        join_string(
                            &mut this.description,
                            // Sometimes doc comments include a space before them, this removes it
                            d.strip_prefix(' ').unwrap_or(&d),
                            attr.sp,
                        );
                        if is_doc {
                            if let Some((_, is_doc, _)) = &mut this.description {
                                *is_doc = true;
                            }
                        }
                        Ok(())
                    }
                    RenameRule(r) => insert(&mut this.rename_rule, r, attr.sp),
                    Rename(r) => insert(&mut this.rename, r, attr.sp),
                    Aliases(a) => insert(&mut this.aliases, a, attr.sp),
                    ParseWith(p) => insert(&mut this.parser, p, attr.sp),
                    Separator(s) => insert(&mut this.separator, s, attr.sp),
                    CommandSeparator(s) => insert(&mut this.command_separator, s, attr.sp),
                    Hide => insert(&mut this.hide, (), attr.sp),
                    HideAliases => insert(&mut this.hide_aliases, (), attr.sp),
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
        let Attr { mut key, value } = attr;

        let outermost_key = key.pop().unwrap(); // `Attr`'s invariants ensure `key.len() > 0`

        let kind = match &*outermost_key.to_string() {
            "doc" => {
                if let Some(unexpected_key) = key.last() {
                    return Err(compile_error_at(
                        "`doc` can't have nested attributes",
                        unexpected_key.span(),
                    ));
                }

                Description(value.expect_string()?, true)
            }

            "command" => {
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
                    "prefix" => Prefix(value.expect_string()?),
                    "description" => Description(value.expect_string()?, false),
                    "rename_rule" => {
                        RenameRule(value.expect_string().and_then(|r| self::RenameRule::parse(&r))?)
                    }
                    "rename" => Rename(value.expect_string()?),
                    "parse_with" => ParseWith(ParserType::parse(value)?),
                    "separator" => Separator(value.expect_string()?),
                    "command_separator" => CommandSeparator(value.expect_string()?),
                    "hide" => value.expect_none("hide").map(|_| Hide)?,
                    "hide_aliases" => value.expect_none("hide_aliases").map(|_| HideAliases)?,
                    "alias" => Aliases(vec![value.expect_string()?]),
                    "aliases" => Aliases(
                        value
                            .expect_array()?
                            .into_iter()
                            .map(AttrValue::expect_string)
                            .collect::<Result<_>>()?,
                    ),
                    _ => {
                        return Err(compile_error_at(
                            "unexpected attribute name (expected one of `prefix`, `description`, \
                             `rename`, `parse_with`, `separator`, `hide`, `alias` and `aliases`",
                            attr.span(),
                        ))
                    }
                }
            }

            _ => {
                return Err(compile_error_at(
                    "unexpected attribute (expected `command` or `doc`)",
                    outermost_key.span(),
                ))
            }
        };

        Ok(Self { kind, sp })
    }
}

fn is_command_attribute(a: &Attribute) -> bool {
    matches!(a.path().get_ident(), Some(ident) if ident == "command")
}

fn is_doc_comment(a: &Attribute) -> bool {
    matches!(
        a.path().get_ident(),
        Some(ident) if ident == "doc" && a.meta.require_name_value().is_ok()
    )
}
