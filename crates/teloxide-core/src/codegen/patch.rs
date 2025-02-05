use crate::codegen::schema::{Doc, Schema, Type};

pub fn patch_schema(mut schema: Schema) -> Schema {
    fn check(l: &Option<&str>, r: &str) -> bool {
        l.map(|m| r == m).unwrap_or(true)
    }

    schema.methods.iter_mut().for_each(|method| {
        method.params.iter_mut().map(|p| &mut p.name).for_each(escape_kw);

        DOC_PATCHES.iter().for_each(|(key, patch)| match key {
            Target::Method(m) => {
                if check(m, &method.names.0) {
                    method.doc.patch(patch, *key);
                }
            }
            Target::Field { method_name: m, field_name: f } => {
                if check(m, &method.names.0) {
                    method
                        .params
                        .iter_mut()
                        .filter(|p| check(f, &p.name))
                        .for_each(|p| p.descr.patch(patch, *key))
                }
            }
            Target::Any { method_name: m } => {
                if check(m, &method.names.0) {
                    method.doc.patch(patch, *key);

                    method.params.iter_mut().for_each(|p| p.descr.patch(patch, *key))
                }
            }
        });
    });

    schema
}

static DOC_PATCHES: &[(Target, Patch)] = &[
    (
        Target::Any { method_name: None },
        Patch::ReplaceLink {
            name: "More info on Sending Files »",
            value: "crate::types::InputFile",
        },
    ),
    (
        Target::Field {
            method_name: Some("sendChatAction"),
            field_name: Some("action"),
        },
        Patch::ReplaceLink {
            name: "text messages",
            value: "crate::payloads::SendMessage",
        },
    ),
    (
        Target::Field {
            method_name: Some("sendChatAction"),
            field_name: Some("action"),
        },
        Patch::ReplaceLink {
            name: "photos",
            value: "crate::payloads::SendPhoto",
        },
    ),
    (
        Target::Field {
            method_name: Some("sendChatAction"),
            field_name: Some("action"),
        },
        Patch::ReplaceLink {
            name: "videos",
            value: "crate::payloads::SendVideo",
        },
    ),
    (
        Target::Field {
            method_name: Some("sendChatAction"),
            field_name: Some("action"),
        },
        Patch::ReplaceLink {
            name: "audio files",
            value: "crate::payloads::SendAudio",
        },
    ),
    (
        Target::Field {
            method_name: Some("sendChatAction"),
            field_name: Some("action"),
        },
        Patch::ReplaceLink {
            name: "general files",
            value: "crate::payloads::SendDocument",
        },
    ),
    (
        Target::Field {
            method_name: Some("sendChatAction"),
            field_name: Some("action"),
        },
        Patch::ReplaceLink {
            name: "location data",
            value: "crate::payloads::SendLocation",
        },
    ),
    (
        Target::Field {
            method_name: Some("sendChatAction"),
            field_name: Some("action"),
        },
        Patch::ReplaceLink {
            name: "video notes",
            value: "crate::payloads::SendVideoNote",
        },
    ),
    (
        Target::Field {
            method_name: Some("sendChatAction"),
            field_name: Some("action"),
        },
        Patch::ReplaceLink {
            name: "stickers",
            value: "crate::payloads::SendSticker",
        },
    ),
    (
        Target::Any { method_name: None },
        Patch::Custom(intra_links),
    ),
    (
        Target::Method(Some("addStickerToSet")),
        Patch::Replace {
            text: "You **must** use exactly one of the fields _png\\_sticker_ or _tgs\\_sticker_. ",
            with: "",
        },
    ),
    (
        Target::Method(Some("GetFile")),
        Patch::Replace {
            text: "The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [`GetFile`] again.",
            with: "The file can then be downloaded via the method [`Bot::download_file(file_path, dst)`], where `file_path` is taken from the response. It is guaranteed that the path from [`GetFile`] will be valid for at least 1 hour. When the path expires, a new one can be requested by calling [`GetFile`].",
        },
    ),
    (
        Target::Method(Some("GetFile")),
        Patch::AddLink {
            name: "`Bot::download_file(file_path, dst)`",
            value: "crate::net::Download::download_file",
        },
    ),
    // FIXME RETUNRS
];

#[derive(Debug, Clone, Copy)]
enum Target<'a> {
    Any { method_name: Option<&'a str> },
    Method(Option<&'a str>),
    Field { method_name: Option<&'a str>, field_name: Option<&'a str> },
}

impl Target<'_> {
    fn is_exact(&self) -> bool {
        match self {
            Target::Method(m) => m.is_some(),
            Target::Field { method_name, field_name } => {
                method_name.is_some() && field_name.is_some()
            }
            Target::Any { method_name: _ } => false,
        }
    }
}

enum Patch<'a> {
    ReplaceLink { name: &'a str, value: &'a str },
    AddLink { name: &'a str, value: &'a str },
    // RemoveLink { name: &'a str },
    // FullReplace { text: &'a str, with: &'a str },
    Replace { text: &'a str, with: &'a str },
    Custom(fn(&mut Doc)),
}

impl Doc {
    fn patch(&mut self, patch: &Patch, key: Target) {
        match patch {
            Patch::ReplaceLink { name, value } => {
                if let Some(link) = self.md_links.get_mut(*name) {
                    link.clear();
                    *link += *value;
                } else if key.is_exact() {
                    panic!("Patch error: {key:?} doesn't have link {name}");
                }
            }
            Patch::AddLink { name, value } => {
                self.md_links.insert((*name).to_owned(), (*value).to_owned());
            }
            // Patch::RemoveLink { name } => drop(self.md_links.remove(*name)),
            // Patch::FullReplace { text, with } => {
            //     assert_eq!(self.md.as_str(), *text);

            //     self.md.clear();
            //     self.md += with;
            // }
            Patch::Replace { text, with } => self.md = self.md.replace(*text, with),
            Patch::Custom(f) => f(self),
        }
    }
}

fn intra_links(doc: &mut Doc) {
    let mut repls_t = Vec::new();
    let mut repls_m = Vec::new();

    doc.md_links
        .iter_mut()
        .filter(|(k, v)| {
            v.starts_with("https://core.telegram.org/bots/api#")
                && !k.contains(&['-', '_', '.', ' '][..])
        })
        .for_each(|(k, v)| {
            if let Some(c) = k.chars().next() {
                match () {
                    _ if k == "games" => {}
                    _ if k == "unbanned" => *v = String::from("crate::payloads::UnbanChatMember"),
                    _ if c.is_lowercase() && !["update"].contains(&&**k) => {
                        repls_m.push(k.clone());
                        *v = format!("crate::payloads::{}", to_uppercase(k));
                    }
                    _ => {
                        repls_t.push(k.clone());
                        *v = format!("crate::types::{k}");
                    }
                }
            }
        });

    for repl in repls_t {
        if let Some(value) = doc.md_links.swap_remove(repl.as_str()) {
            doc.md = doc.md.replace(format!("[{repl}]").as_str(), &format!("[`{repl}`]"));
            doc.md_links.insert(format!("`{repl}`"), value);
        }
    }

    for repl in repls_m {
        if let Some(value) = doc.md_links.swap_remove(repl.as_str()) {
            let repln = to_uppercase(&repl);
            doc.md = doc.md.replace(format!("[{repl}]").as_str(), &format!("[`{repln}`]"));
            doc.md_links.insert(format!("`{repln}`"), value);
        }
    }
}

fn escape_kw(s: &mut String) {
    if ["type"].contains(&s.as_str()) {
        *s = format!("{s}_");
    }
}

fn to_uppercase(s: &str) -> String {
    let mut chars = s.chars();
    format!("{}{}", chars.next().unwrap().to_uppercase(), chars.as_str())
}

pub(crate) fn patch_ty(mut schema: Schema) -> Schema {
    // URLs
    patch_types(&mut schema, Type::String, Type::Url, &[("set_webhook", "url")]);

    patch_types(
        &mut schema,
        Type::Option(Box::new(Type::String)),
        Type::Option(Box::new(Type::Url)),
        &[("answer_callback_query", "url"), ("send_invoice", "photo_url")],
    );

    // Dates
    patch_types(
        &mut schema,
        Type::Option(Box::new(Type::u64)),
        Type::Option(Box::new(Type::DateTime)),
        &[
            ("send_poll", "close_date"),
            ("ban_chat_member", "until_date"),
            ("kick_chat_member", "until_date"),
            ("restrict_chat_member", "until_date"),
        ],
    );
    patch_types(
        &mut schema,
        Type::Option(Box::new(Type::i64)),
        Type::Option(Box::new(Type::DateTime)),
        &[("create_chat_invite_link", "expire_date"), ("edit_chat_invite_link", "expire_date")],
    );

    schema
}

fn patch_types(schema: &mut Schema, from: Type, to: Type, list: &[(&str, &str)]) {
    // URLs
    for &(method, param) in list {
        let m = schema
            .methods
            .iter_mut()
            .find(|m| m.names.2 == method)
            .expect("Couldn't find method for patching");

        let p = m
            .params
            .iter_mut()
            .find(|p| p.name == param)
            .expect("Couldn't find parameter for patching");

        assert_eq!(p.ty, from, "{method}::{param}");
        p.ty = to.clone();
    }
}
