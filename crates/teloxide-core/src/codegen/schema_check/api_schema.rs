//! Credit to https://github.com/ark0f/tg-bot-api and https://github.com/ENCRYPTEDFOREVER/tg-bot-api/tree/bot_api_9_0
use crate::codegen::project_root;
use serde::Deserialize;
use std::fs;

fn get_api_schema() -> ApiSchema {
    let path = project_root().join("custom_v2.json");
    let text = fs::read_to_string(path).unwrap();
    let schema: ApiSchema = serde_json::from_str(&text).unwrap();

    schema
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct ApiSchema {
    version: Version,
    recent_changes: Date,
    methods: Vec<ApiMethod>,
    objects: Vec<Object>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Date {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
enum Kind {
    Integer {
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<i64>,
        enumeration: Vec<i64>,
    },
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min_len: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_len: Option<u64>,
        enumeration: Vec<String>,
    },
    Bool {
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<bool>,
    },
    Float,
    AnyOf {
        any_of: Vec<KindWrapper>,
    },
    Reference {
        reference: String,
    },
    Array {
        array: Box<KindWrapper>,
    },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
struct KindWrapper(Kind);

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ApiMethod {
    name: String,
    description: String,
    arguments: Vec<Argument>,
    maybe_multipart: bool,
    return_type: KindWrapper,
    documentation_link: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Argument {
    name: String,
    description: String,
    required: bool,
    #[serde(rename = "type_info")]
    kind: KindWrapper,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Object {
    name: String,
    description: String,
    #[serde(flatten)]
    data: ObjectData,
    documentation_link: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
enum ObjectData {
    Properties { properties: Vec<Property> },
    AnyOf { any_of: Vec<KindWrapper> },
    Unknown,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Property {
    name: String,
    description: String,
    required: bool,
    #[serde(rename = "type_info")]
    kind: KindWrapper,
}

#[cfg(test)]
mod tests {
    use derive_more::derive::Display;

    use super::*;
    use crate::codegen::{patch::escape_kw, schema};

    #[derive(Debug, Display)]
    enum ApiCheckError {
        #[display("Method {method} does not exist")]
        MethodDoesNotExist { method: String },
        #[display("Method {method} does not have {param} parameter")]
        ParamDoesNotExist { method: String, param: String },
        #[display(
            "Method {method} and sibling {sibling} have something different in {param} param."
        )]
        SiblingParamsDontMatch { method: String, sibling: String, param: String },
        #[display("Method {method} has required {param} parameter, when it is not required")]
        ParamIsNotRequired { method: String, param: String },
        #[display("Method {method} has optional {param} parameter, when it is not optional")]
        ParamIsNotOptional { method: String, param: String },
        #[display(
            "{param} parameter of method {method} has a number type that can't fit all the \
             possible values. The limit is {param_limit}, but values go up to {actual_limit}"
        )]
        ParamIsTooRestrictive { method: String, param: String, param_limit: i64, actual_limit: i64 }, /* Limit can't be higher than i64 ever */
        #[display(
            "{param} parameter of method {method} is of type {raw_type}, but the actual type is \
             {actual_type}"
        )]
        ParamRawTyDoesNotMatch {
            method: String,
            param: String,
            raw_type: String,
            actual_type: String,
        },
        #[display(
            "{param} parameter of method {method} is of type [{raw_type:?}], but the actual type \
             is [{actual_type:?}]"
        )]
        ParamTyDoesNotMatch {
            method: String,
            param: String,
            raw_type: schema::Type,
            actual_type: Kind,
        },
        #[display(
            "Method {method} has a link to TBA {doc_link}, but the actual link is \
             {actual_doc_link}"
        )]
        MethodDocLinkDoesNotMatch { method: String, doc_link: String, actual_doc_link: String },
    }

    #[derive(Debug, Clone, PartialEq)]
    enum Exception {
        MethodField { method: String, param: String },
        FieldType { ron_raw_type: String, actual_type: String },
        SiblingParam { param: String },
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Exceptions {
        exceptions: Vec<Exception>,
    }

    impl Exceptions {
        fn new(exceptions: Vec<Exception>) -> Self {
            Self { exceptions }
        }

        fn is_method_field_exception(&self, method: String, param: String) -> bool {
            self.exceptions.contains(&Exception::MethodField { method, param })
        }

        fn is_field_type_exception(&self, ron_raw_type: String, actual_type: String) -> bool {
            self.exceptions.contains(&Exception::FieldType { ron_raw_type, actual_type })
        }

        fn is_sibling_param_exception(&self, param: String) -> bool {
            self.exceptions.contains(&Exception::SiblingParam { param })
        }
    }

    fn find_ron_method_by_name(name: &str, schema: &schema::Schema) -> Option<schema::Method> {
        schema.methods.iter().find(|x| x.names.0 == name).cloned()
    }

    fn find_ron_param_by_name(name: &str, method: &schema::Method) -> Option<schema::Param> {
        method.params.iter().find(|x| x.name == name).cloned()
    }

    fn check_ron_siblings(
        ron_method: &schema::Method,
        ron_sibling_method: &schema::Method,
        method: &ApiMethod,
        errors: &mut Vec<ApiCheckError>,
        missing_field_exceptions: &Exceptions,
    ) {
        // If checker finds that one sibling has that field name, while the other
        // doesn't, it will consider it normal. To add a one-time exception,
        // refer to `missing_field_exceptions` argument.
        let acceptable_sibling_params = Exceptions::new(vec![
            Exception::SiblingParam { param: "inline_message_id".to_owned() },
            Exception::SiblingParam { param: "user_id".to_owned() },
            Exception::SiblingParam { param: "chat_id".to_owned() },
            Exception::SiblingParam { param: "message_id".to_owned() },
        ]);

        check_ron_method_meta(ron_method, method, errors);
        check_ron_method_meta(ron_sibling_method, method, errors);

        for param in &method.arguments {
            let mut param_name = param.name.clone();
            escape_kw(&mut param_name); // Converts type to type_. Will think field is missing
                                        // otherwise.

            let maybe_ron_param = find_ron_param_by_name(&param_name, ron_method);
            let maybe_ron_sibling_param = find_ron_param_by_name(&param_name, ron_sibling_method);

            match (maybe_ron_param.clone(), maybe_ron_sibling_param.clone()) {
                (Some(ron_param), Some(ron_sibling_param)) => check_ron_siblings_field(
                    ron_method,
                    ron_sibling_method,
                    &ron_param,
                    &ron_sibling_param,
                    param,
                    errors,
                ),
                (Some(ron_param), None) => check_ron_siblings_only_one_field(
                    &ron_param,
                    ron_method.names.0.clone(),
                    ron_sibling_method.names.0.clone(),
                    param,
                    errors,
                    &acceptable_sibling_params,
                    missing_field_exceptions,
                ),
                (None, Some(ron_sibling_param)) => check_ron_siblings_only_one_field(
                    &ron_sibling_param,
                    ron_sibling_method.names.0.clone(),
                    ron_method.names.0.clone(),
                    param,
                    errors,
                    &acceptable_sibling_params,
                    missing_field_exceptions,
                ),
                (None, None) => {
                    if !missing_field_exceptions
                        .is_method_field_exception(method.name.clone(), param_name)
                    {
                        errors.push(ApiCheckError::ParamDoesNotExist {
                            method: method.name.clone(),
                            param: param.name.clone(),
                        });
                    }
                }
            }
        }
    }

    fn check_ron_siblings_field(
        ron_method: &schema::Method,
        ron_sibling_method: &schema::Method,
        ron_param: &schema::Param,
        ron_sibling_param: &schema::Param,
        param: &Argument,
        errors: &mut Vec<ApiCheckError>,
    ) {
        if ron_param != ron_sibling_param {
            errors.push(ApiCheckError::SiblingParamsDontMatch {
                method: ron_method.names.0.clone(),
                sibling: ron_sibling_method.names.0.clone(),
                param: param.name.clone(),
            });
        } else {
            check_param(ron_param, param, ron_method.names.0.clone(), false, errors);
        }
    }

    fn check_ron_siblings_only_one_field(
        ron_param: &schema::Param,
        ron_method_name: String,
        ron_method_without_param_name: String,
        param: &Argument,
        errors: &mut Vec<ApiCheckError>,
        acceptable_sibling_params: &Exceptions,
        missing_field_exceptions: &Exceptions,
    ) {
        let mut param_name = param.name.clone();
        escape_kw(&mut param_name);

        // Check if its in some exceptions
        if (!acceptable_sibling_params.is_sibling_param_exception(param_name.clone()))
            && (!missing_field_exceptions
                .is_method_field_exception(ron_method_without_param_name.clone(), param_name))
        {
            errors.push(ApiCheckError::ParamDoesNotExist {
                method: ron_method_without_param_name,
                param: param.name.clone(),
            });
        } else {
            // It is in some exceptions. We don't care about optional fields, since its an
            // exception.
            check_param(ron_param, param, ron_method_name, true, errors);
        }
    }

    fn check_ron_params(
        ron_method: &schema::Method,
        method: &ApiMethod,
        errors: &mut Vec<ApiCheckError>,
        missing_field_exceptions: &Exceptions,
    ) {
        for param in &method.arguments {
            let mut param_name = param.name.clone();
            escape_kw(&mut param_name);

            if let Some(ron_param) = find_ron_param_by_name(&param_name, ron_method) {
                check_param(&ron_param, param, method.name.clone(), false, errors);
            } else if !missing_field_exceptions
                .is_method_field_exception(method.name.clone(), param_name)
            {
                errors.push(ApiCheckError::ParamDoesNotExist {
                    method: method.name.clone(),
                    param: param.name.clone(),
                });
            }
        }
    }

    // Checks everything about the method that is not params
    fn check_ron_method_meta(
        ron_method: &schema::Method,
        method: &ApiMethod,
        errors: &mut Vec<ApiCheckError>,
    ) {
        check_type(
            &ron_method.return_ty,
            &method.return_type.0,
            "return_ty".to_owned(),
            method.name.clone(),
            errors,
        );

        // Some docs are for some reason like api/#something, not api#something.
        if ron_method.tg_doc.replace("/#", "#") != method.documentation_link.replace("/#", "#") {
            errors.push(ApiCheckError::MethodDocLinkDoesNotMatch {
                method: method.name.clone(),
                doc_link: ron_method.tg_doc.clone(),
                actual_doc_link: method.documentation_link.clone(),
            });
        }
    }

    fn check_ron_method(
        ron_method: &schema::Method,
        method: &ApiMethod,
        errors: &mut Vec<ApiCheckError>,
        missing_field_exceptions: &Exceptions,
    ) {
        check_ron_method_meta(ron_method, method, errors);
        check_ron_params(ron_method, method, errors, missing_field_exceptions);
    }

    fn check_param(
        ron_param: &schema::Param,
        param: &Argument,
        method_name: String,
        // If it is a method with a sibling, some optional fields could be required in our
        // schema
        ignore_optional: bool,
        errors: &mut Vec<ApiCheckError>,
    ) {
        let mut ron_param = ron_param.clone();

        if let schema::Type::Option(ron_param_type) = &ron_param.ty {
            if param.required {
                errors.push(ApiCheckError::ParamIsNotOptional {
                    method: method_name.clone(),
                    param: param.name.clone(),
                });
            }
            ron_param.ty = *ron_param_type.clone()
        } else if !param.required && !ignore_optional {
            errors.push(ApiCheckError::ParamIsNotRequired {
                method: method_name.clone(),
                param: param.name.clone(),
            });
        }

        check_type(&ron_param.ty, &param.kind.0, param.name.clone(), method_name.clone(), errors);
    }

    fn check_type(
        ron_type: &schema::Type,
        api_type: &Kind,
        param_name: String,
        method_name: String,
        errors: &mut Vec<ApiCheckError>,
    ) {
        // Assumes that the ron_type is not Option, since api_type has `required` field
        // instead
        assert!(!matches!(ron_type, &schema::Type::Option(_)), "ron_type can't be Option");

        // Some types can be more narrow in teloxide, e.g. `Me` is a subset of `User`
        let acceptable_different_raw_type = Exceptions::new(vec![
            Exception::FieldType {
                ron_raw_type: "ReplyMarkup".to_owned(),
                actual_type: "InlineKeyboardMarkup".to_owned(),
            },
            Exception::FieldType { ron_raw_type: "Me".to_owned(), actual_type: "User".to_owned() },
        ]);

        // If it matches, do nothing
        match (ron_type, api_type) {
            (schema::Type::bool, Kind::Bool { default: _ }) => {}
            (
                schema::Type::u8
                | schema::Type::u16
                | schema::Type::u32
                | schema::Type::u64
                | schema::Type::i32
                | schema::Type::i64
                | schema::Type::DateTime, // DateTime is always an int timestamp in the TBA
                Kind::Integer { default: _, min, max, enumeration: _ },
            ) => {
                let (ron_min, ron_max) = match ron_type {
                    schema::Type::u8 => (u8::MIN as i64, u8::MAX as i64),
                    schema::Type::u16 => (u16::MIN as i64, u16::MAX as i64),
                    schema::Type::u32 => (u32::MIN as i64, u32::MAX as i64),
                    schema::Type::u64 => (u64::MIN as i64, i64::MAX), // u64::MAX will
                    // overflow. And the TBA will never return something bigger than i64.
                    schema::Type::i32 => (i32::MIN as i64, i32::MAX as i64),
                    schema::Type::i64 => (i64::MIN, i64::MAX),
                    schema::Type::DateTime => (i64::MIN, i64::MAX),
                    _ => unreachable!("Other types are not in the match statement"),
                };

                if min.is_some() && ron_min > min.unwrap() {
                    errors.push(ApiCheckError::ParamIsTooRestrictive {
                        method: method_name.clone(),
                        param: param_name.clone(),
                        param_limit: ron_min,
                        actual_limit: min.unwrap(),
                    })
                }
                if max.is_some() && ron_max < max.unwrap() {
                    errors.push(ApiCheckError::ParamIsTooRestrictive {
                        method: method_name.clone(),
                        param: param_name.clone(),
                        param_limit: ron_max,
                        actual_limit: max.unwrap(),
                    })
                }
            }
            (
                schema::Type::String | schema::Type::Url,
                Kind::String { default: _, min_len: _, max_len: _, enumeration: _ },
            ) => {}
            (schema::Type::f64, Kind::Float) => {}
            (schema::Type::True, Kind::Bool { default: Some(true) }) => {}
            (schema::Type::ArrayOf(ron_type), Kind::Array { array: api_type }) => {
                check_type(ron_type, &api_type.0, param_name, method_name, errors);
            }
            (schema::Type::RawTy(ron_raw_type), Kind::Reference { reference: raw_type }) => {
                if ron_raw_type != raw_type
                    && !acceptable_different_raw_type
                        .is_field_type_exception(ron_raw_type.to_owned(), raw_type.to_owned())
                {
                    errors.push(ApiCheckError::ParamRawTyDoesNotMatch {
                        method: method_name.clone(),
                        param: param_name.clone(),
                        raw_type: ron_raw_type.to_owned(),
                        actual_type: raw_type.to_owned(),
                    })
                }
            }
            (schema::Type::RawTy(_), Kind::AnyOf { any_of: _ }) => {} // If it's AnyOf, we have to
            // have our own type like `Recipient`
            (schema::Type::True, Kind::AnyOf { any_of: _ }) => {} // Or with AnyOf there could be
            // either True or Message, like with editMessageMedia
            (schema::Type::RawTy(_), _) => {} // Any other is fine, we can't check if
            // our type like PollId is actually String or Integer
            _ => errors.push(ApiCheckError::ParamTyDoesNotMatch {
                method: method_name.clone(),
                param: param_name.clone(),
                raw_type: ron_type.clone(),
                actual_type: api_type.clone(),
            }),
        }
    }

    #[test]
    fn check_ron_schema() {
        let ron_schema = schema::get();
        let api_schema = get_api_schema();
        let mut errors = vec![];

        // Here you can set exceptions for fields that don't exist in our schema for
        // some reason
        let missing_field_exceptions = Exceptions::new(vec![
            // The Inline methods can't set these values
            Exception::MethodField {
                method: "editMessageTextInline".to_owned(),
                param: "link_preview_options".to_owned(),
            },
            Exception::MethodField {
                method: "editMessageCaptionInline".to_owned(),
                param: "link_preview_options".to_owned(),
            },
            Exception::MethodField {
                method: "editMessageCaptionInline".to_owned(),
                param: "show_caption_above_media".to_owned(),
            },
            Exception::MethodField {
                method: "editMessageLiveLocationInline".to_owned(),
                param: "live_period".to_owned(),
            },
            // getGameHighScores has `target` field for that
            Exception::MethodField {
                method: "getGameHighScores".to_owned(),
                param: "chat_id".to_owned(),
            },
            Exception::MethodField {
                method: "getGameHighScores".to_owned(),
                param: "message_id".to_owned(),
            },
            Exception::MethodField {
                method: "getGameHighScores".to_owned(),
                param: "inline_message_id".to_owned(),
            },
        ]);

        let api_version = format!("{}.{}", api_schema.version.major, api_schema.version.minor);
        let ron_version = ron_schema.api_version.ver.clone();
        if api_version != ron_version {
            panic!(
                "schema.ron is of api version {ron_version}, while the checking schema is \
                 {api_version}. Please update the checking schema."
            );
        }

        for method in api_schema.methods {
            if let Some(ron_method) = find_ron_method_by_name(&method.name, &ron_schema) {
                if let Some(ref sibling) = ron_method.sibling {
                    let ron_sibling_method = find_ron_method_by_name(sibling, &ron_schema)
                        .unwrap_or_else(|| {
                            panic!(
                                "{}",
                                format!("Sibling method of {} does not exist", &method.name)
                            )
                        });

                    check_ron_siblings(
                        &ron_method,
                        &ron_sibling_method,
                        &method,
                        &mut errors,
                        &missing_field_exceptions,
                    );
                } else {
                    check_ron_method(&ron_method, &method, &mut errors, &missing_field_exceptions);
                }
            } else {
                errors.push(ApiCheckError::MethodDoesNotExist { method: method.name });
            };
        }

        if !errors.is_empty() {
            let mut errors_string = String::new();
            for (i, error) in errors.iter().enumerate() {
                errors_string = format!("{errors_string}\n\n{}. {error}", i + 1);
            }
            panic!(
                "schema.ron does not match the check schema. The errors are:\n\n{errors_string}",
            );
        }
    }
}
