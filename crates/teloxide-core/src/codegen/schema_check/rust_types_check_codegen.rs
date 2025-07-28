use crate::codegen::schema_check::api_schema::*;
use derive_more::derive::Display;
use serde_json::{Map, Value};

fn convert_reference_string(reference: String, initial_object_name: String) -> String {
    reference.trim_start_matches("#/$defs/").replace("#", &initial_object_name).to_string()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Exception {
    RemoveObjectFromChecking { object: String },
    RenameCheckingObject { api_object_name: String, rust_object_name: String },
    IgnoreFieldName { field_name: String },
    IgnoreObjectField { object: String, field_name: String },
    ExpandTelegramReferece { reference: String },
    IgnoreFieldRequiredName { field_name: String },
    IgnoreFieldRequiredObjectName { object: String, field_name: String },
}

impl Exception {
    fn to_eval_rust(&self) -> String {
        match self {
            Self::RemoveObjectFromChecking { object } => {
                format!("Exception::RemoveObjectFromChecking {{ object: \"{object}\".to_owned() }}")
            }
            Self::RenameCheckingObject { api_object_name, rust_object_name } => {
                format!(
                    "Exception::RenameCheckingObject {{ api_object_name: \
                     \"{api_object_name}\".to_owned(), rust_object_name: \
                     \"{rust_object_name}\".to_owned() }}"
                )
            }
            Self::IgnoreFieldName { field_name } => {
                format!("Exception::IgnoreFieldName {{ field_name: \"{field_name}\".to_owned() }}")
            }
            Self::IgnoreObjectField { object, field_name } => format!(
                "Exception::IgnoreObjectField {{ object: \"{object}\".to_owned(), field_name: \
                 \"{field_name}\".to_owned() }}"
            ),
            Self::ExpandTelegramReferece { reference } => {
                format!(
                    "Exception::ExpandTelegramReferece {{ reference: \"{reference}\".to_owned() }}"
                )
            }
            Self::IgnoreFieldRequiredName { field_name } => format!(
                "Exception::IgnoreFieldRequiredName {{ field_name: \"{field_name}\".to_owned() }}"
            ),
            Self::IgnoreFieldRequiredObjectName { object, field_name } => format!(
                "Exception::IgnoreFieldRequiredObjectName {{ object: \"{object}\".to_owned(), \
                 field_name: \"{field_name}\".to_owned() }}"
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exceptions {
    exceptions: Vec<Exception>,
}

impl Exceptions {
    pub fn new(exceptions: Vec<Exception>) -> Self {
        Self { exceptions }
    }

    fn extend(&mut self, exceptions: Vec<Exception>) {
        self.exceptions.extend(exceptions);
    }

    fn is_field_ignored(&self, field_name: String) -> bool {
        self.exceptions.contains(&Exception::IgnoreFieldName { field_name })
    }

    fn is_object_field_ignored(&self, object: String, field_name: String) -> bool {
        self.exceptions.contains(&Exception::IgnoreObjectField { object, field_name })
    }

    fn is_object_field_required_ignored(&self, object: String, field_name: String) -> bool {
        self.exceptions.contains(&Exception::IgnoreFieldRequiredObjectName { object, field_name })
    }

    fn is_field_required_ignored(&self, field_name: String) -> bool {
        self.exceptions.contains(&Exception::IgnoreFieldRequiredName { field_name })
    }

    fn is_expand_refenence(&self, reference: String) -> bool {
        self.exceptions.contains(&Exception::ExpandTelegramReferece { reference })
    }

    fn is_object_removed_from_checking(&self, object: String) -> bool {
        self.exceptions.contains(&Exception::RemoveObjectFromChecking { object })
    }

    fn get_renamed(&self, api_object: String) -> Option<String> {
        for exception in self.exceptions.iter() {
            if let Exception::RenameCheckingObject { api_object_name, rust_object_name } = exception
            {
                if *api_object_name == api_object {
                    return Some(rust_object_name.to_owned());
                }
            }
        }

        None
    }
}

#[derive(Debug, Display)]
#[allow(clippy::enum_variant_names)] // Other errors might be added in the future
pub enum ApiCheckError {
    #[display("Object `{object}` does not have `{field}` field")]
    FieldDoesNotExist { object: String, field: String },
    #[display("Object `{object}` has required `{field}` field, when it is not required")]
    FieldIsNotRequired { object: String, field: String },
    #[display("Object `{object}` has optional `{field}` field, when it is not optional")]
    FieldIsNotOptional { object: String, field: String },
    #[display(
        "`{field}` field of object `{object}` is of type `{raw_type}`, but the actual type is \
         `{actual_type}`"
    )]
    FieldReferenceTyDoesNotMatch {
        object: String,
        field: String,
        raw_type: String,
        actual_type: String,
    },
    #[display(
        "`{field}` field of object `{object}` is of type [{raw_type:?}], but the actual type is \
         [{actual_type:?}]"
    )]
    FieldTyDoesNotMatch { object: String, field: String, raw_type: Kind, actual_type: Kind },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Integer,
    Boolean,
    Number,
    Array,
    Object,
    Null,
    Reference { reference: String },
    Unknown,
}

fn get_type(prop: &Value, initial_object_name: String) -> (bool, Kind) {
    let mut required = true;
    let mut prop_type: Option<Type> = None;
    let mut item_kind: Option<Kind> = None;

    if let Some(any_of) = prop.get("anyOf") {
        if let Some(array) = any_of.as_array() {
            let mut types_array = vec![];
            for variant in array {
                types_array.push(get_type(variant, initial_object_name.clone()).1);
            }

            // This is most likely Recipient with string/int type
            if types_array.len() == 2
                && types_array.contains(&Kind::Reference { reference: "ChatId".to_owned() })
                && types_array.iter().any(|x| {
                    matches!(x, Kind::String { default: _, min_len: _, max_len: _, enumeration: _ })
                })
            {
                // Further checking is pointless, we already know its Recipient
                return (
                    required,
                    Kind::AnyOf {
                        any_of: vec![
                            KindWrapper(Kind::Integer {
                                default: None,
                                min: None,
                                max: None,
                                enumeration: vec![],
                            }),
                            KindWrapper(Kind::String {
                                default: None,
                                min_len: None,
                                max_len: None,
                                enumeration: vec![],
                            }),
                        ],
                    },
                );
            } else {
                for variant_type in types_array.clone() {
                    match variant_type {
                        Kind::Null => required = false,
                        Kind::Reference { reference } => {
                            prop_type = Some(Type::Reference { reference })
                        }
                        unreachable_type => unreachable!(
                            "Object `{initial_object_name}`: Nothing else should be in anyOf, the \
                             type is {unreachable_type:?}, all types are: {types_array:?}"
                        ),
                    }
                }
            }
        }
    } else if let Some(r) = prop.get("$ref") {
        if let Some(s) = r.as_str() {
            required = true;
            prop_type = Some(Type::Reference {
                reference: convert_reference_string(s.to_owned(), initial_object_name.clone()),
            });
        }
    } else if let Some(ty) = prop.get("type") {
        if ty.is_array() {
            let arr = ty.as_array().unwrap();

            for t in arr {
                if t == "null" {
                    required = false;
                } else {
                    prop_type = Some(parse_type(t));
                }
            }
        } else {
            required = true;
            prop_type = Some(parse_type(ty));
        }
    } else if let Some(one_of) = prop.get("oneOf") {
        // If its oneOf, its probably an enum, probably of String
        if let Some(array) = one_of.as_array() {
            // Just extract the type while checking, if its all the same
            let mut all_variant_type = None;
            for variant in array {
                let variant_type = get_type(variant, initial_object_name.clone()).1;

                if all_variant_type.is_none() {
                    all_variant_type = Some(variant_type)
                } else if all_variant_type.clone().unwrap() != variant_type {
                    panic!(
                        "Object `{initial_object_name}`: oneOf variant has a different type. \
                         Initial type: {all_variant_type:?}, got type: {variant_type:?}"
                    );
                }
            }

            if all_variant_type.is_none() {
                panic!("Object `{initial_object_name}`: oneOf variants has no type");
            }

            return (required, all_variant_type.unwrap());
        }
    } else {
        panic!("Object `{initial_object_name}`: No type source was found, {prop:?}")
    }

    if prop_type == Some(Type::Array) {
        if let Some(item) = prop.get("items") {
            item_kind = Some(get_type(item, initial_object_name.clone()).1);
        } else {
            panic!("Object `{initial_object_name}`: Array without items!");
        }
    }

    let prop_kind = match (prop_type, item_kind) {
        (Some(Type::Integer), None) => {
            Kind::Integer { default: None, min: None, max: None, enumeration: vec![] }
        }
        (Some(Type::String), None) => {
            Kind::String { default: None, min_len: None, max_len: None, enumeration: vec![] }
        }
        (Some(Type::Number), None) => Kind::Float,
        (Some(Type::Null), None) => Kind::Null,
        (Some(Type::Boolean), None) => Kind::Bool { default: None },
        (Some(Type::Reference { reference }), None) => Kind::Reference { reference },
        (Some(Type::Array), Some(array_type)) => {
            Kind::Array { array: Box::new(KindWrapper(array_type)) }
        }
        (prop_type, item_kind) => unreachable!(
            "Object `{initial_object_name}`: Any other doesn't make sense. Prop type: \
             {prop_type:?}; Item kind: {item_kind:?}"
        ),
    };

    (required, prop_kind)
}

fn parse_type(val: &Value) -> Type {
    match val.as_str() {
        Some("string") => Type::String,
        Some("integer") => Type::Integer,
        Some("boolean") => Type::Boolean,
        Some("number") => Type::Number,
        Some("array") => Type::Array,
        Some("object") => Type::Object,
        Some("null") => Type::Null,
        _ => Type::Unknown,
    }
}

fn extract_fields_from_references_array(
    array: Vec<Value>,
    official_types: Vec<String>,
    references: &Value,
    initial_object_name: String,
    most_fields_are_optional: bool,
    all_fields_are_optional: bool, // The difference between them is that if code detects that some
    // field exists in all references, all_fields_are_optional will still make them optional
    exceptions: &Exceptions,
) -> Vec<Property> {
    let mut fields = vec![];

    for value in array.clone() {
        if value.get("type").map(Value::as_str) == Some("object".into()) {
            let x = get_fields_of_rust_object(
                &value,
                official_types.clone(),
                references,
                initial_object_name.clone(),
                false,
                exceptions,
            );
            fields.extend(x);
            continue;
        }

        let (required, field_type) = get_type(&value, initial_object_name.clone());

        if let Kind::Reference { reference } = field_type {
            let reference_object = references.get(reference).unwrap();

            fields.extend(get_fields_of_rust_object(
                reference_object,
                official_types.clone(),
                references,
                initial_object_name.clone(),
                !required,
                exceptions,
            ));
        } else {
            continue; // Its probably an empty field like
                      // ButtonRequest::Location
        }
    }

    let fields_clone = fields.clone();

    // If some field appears as much times are there are references, its in every
    // single reference, and is required
    for field in fields.iter_mut() {
        let fields_count = fields_clone
            .iter()
            .filter(|x| {
                x.name == field.name && x.kind == field.kind && field.required && x.required
            })
            .count();
        if fields_count == array.len() {
            if all_fields_are_optional {
                field.required = false;
            } // Otherwise - do nothing, the field is required
        } else if most_fields_are_optional {
            field.required = false
        }
    }

    fields
}

fn expand_reference(
    references: &Value,
    reference: String,
    field_name: String,
    initial_object_name: String,
) -> Option<Kind> {
    let reference_object = references
        .get(&reference)
        .unwrap_or_else(|| panic!("Reference does not exist, {field_name}, {reference}"));

    // A check if its really a wrapper and not smth like FileMeta
    if reference_object.get("type").and_then(|x| x.as_str()) != Some("object") {
        let (reference_required, reference_kind) =
            get_type(reference_object, initial_object_name.clone());

        assert!(
            reference_required,
            "Required assertion is false: {initial_object_name}, field: {field_name}"
        );

        return Some(reference_kind);
    }

    None
}

fn extract_fields_from_properties_object(
    rust_object: &Map<String, Value>,
    official_types: Vec<String>,
    references: &Value,
    initial_object_name: String,
    all_fields_are_optional: bool,
    exceptions: &Exceptions,
) -> Vec<Property> {
    let mut fields = vec![];

    for (name, prop) in rust_object {
        let (required, mut kind) = get_type(prop, initial_object_name.clone());

        // Here we flatten the types like ChatId and others
        if let Kind::Reference { ref reference } = kind {
            // If its a reference that isnt in official types, its probably a wrapper for a
            // type
            if !official_types.contains(reference)
                || exceptions.is_expand_refenence(reference.to_owned())
            {
                if let Some(reference_kind) = expand_reference(
                    references,
                    reference.to_owned(),
                    name.to_owned(),
                    initial_object_name.clone(),
                ) {
                    kind = reference_kind
                }
            }
        } else if let Kind::Array { ref array } = kind {
            if let Kind::Reference { ref reference } = array.0 {
                if !official_types.contains(reference)
                    || exceptions.is_expand_refenence(reference.to_owned())
                {
                    if let Some(reference_kind) = expand_reference(
                        references,
                        reference.to_owned(),
                        name.to_owned(),
                        initial_object_name.clone(),
                    ) {
                        kind = Kind::Array { array: Box::new(KindWrapper(reference_kind)) }
                    }
                }
            }
        }
        fields.push(Property {
            name: name.to_owned(),
            description: prop
                .get("description")
                .unwrap_or(&Value::String(String::new()))
                .as_str()
                .unwrap_or("")
                .to_owned(),
            required: required && !all_fields_are_optional,
            kind: KindWrapper(kind),
        });
    }

    fields
}

fn extend_fields(fields: &mut Vec<Property>, new_fields: Vec<Property>) {
    for field in new_fields {
        if fields.iter().any(|x| x.name == field.name) {
            continue;
        }

        fields.push(field);
    }
}

fn get_fields_of_rust_object(
    rust_object: &Value,
    official_types: Vec<String>,
    references: &Value,
    initial_object_name: String,
    all_fields_are_optional: bool,
    exceptions: &Exceptions,
) -> Vec<Property> {
    let mut fields = vec![];
    let empty_array = Value::Array(vec![]);
    let empty_obj = Value::Null;

    if let Some(props) = rust_object.get("properties").unwrap_or(&empty_obj).as_object() {
        extend_fields(
            &mut fields,
            extract_fields_from_properties_object(
                props,
                official_types.clone(),
                references,
                initial_object_name.clone(),
                all_fields_are_optional,
                exceptions,
            ),
        );
    }

    if let Some(one_of) = rust_object.get("oneOf").unwrap_or(&empty_array).as_array() {
        extend_fields(
            &mut fields,
            extract_fields_from_references_array(
                one_of.to_vec(),
                official_types.clone(),
                references,
                initial_object_name.clone(),
                true,
                all_fields_are_optional,
                exceptions,
            ),
        );
    }

    if let Some(any_of) = rust_object.get("anyOf").unwrap_or(&empty_array).as_array() {
        extend_fields(
            &mut fields,
            extract_fields_from_references_array(
                any_of.to_vec(),
                official_types.clone(),
                references,
                initial_object_name.clone(),
                true,
                all_fields_are_optional,
                exceptions,
            ),
        );
    }

    if let Some(ref_string) = rust_object.get("$ref").unwrap_or(&empty_obj).as_str() {
        let reference_object = references
            .get(convert_reference_string(ref_string.to_owned(), initial_object_name.clone()))
            .unwrap();
        extend_fields(
            &mut fields,
            get_fields_of_rust_object(
                reference_object,
                official_types.clone(),
                references,
                initial_object_name.clone(),
                all_fields_are_optional,
                exceptions,
            ),
        );
    }

    fields
}

fn check_struct_kind(
    rust_kind: &Kind,
    api_kind: &Kind,
    field_name: String,
    object_name: String,
    errors: &mut Vec<ApiCheckError>,
) {
    if std::mem::discriminant(rust_kind) != std::mem::discriminant(api_kind) {
        errors.push(ApiCheckError::FieldTyDoesNotMatch {
            object: object_name,
            field: field_name,
            raw_type: rust_kind.clone(),
            actual_type: api_kind.clone(),
        });
    } else if let (
        Kind::Reference { reference: rust_reference },
        Kind::Reference { reference: api_reference },
    ) = (rust_kind, api_kind)
    {
        if rust_reference != api_reference {
            errors.push(ApiCheckError::FieldReferenceTyDoesNotMatch {
                object: object_name,
                field: field_name,
                raw_type: rust_reference.to_owned(),
                actual_type: api_reference.to_owned(),
            });
        }
    } else if let (Kind::Array { array: rust_array }, Kind::Array { array: api_array }) =
        (rust_kind, api_kind)
    {
        check_struct_kind(&rust_array.0, &api_array.0, field_name, object_name, errors);
    }
}

fn check_struct_field(
    rust_field: &Property,
    api_field: &Property,
    field_name: String,
    object_name: String,
    errors: &mut Vec<ApiCheckError>,
    exceptions: &Exceptions,
) {
    if exceptions.is_field_ignored(field_name.clone())
        || exceptions.is_object_field_ignored(object_name.clone(), field_name.clone())
    {
        return;
    }

    let rust_kind = rust_field.kind.0.clone();
    let api_kind = api_field.kind.0.clone();

    check_struct_kind(&rust_kind, &api_kind, field_name.clone(), object_name.clone(), errors);

    if exceptions.is_field_required_ignored(field_name.clone())
        || exceptions.is_object_field_required_ignored(object_name.clone(), field_name.clone())
    {
        return;
    }

    // We have a lot skip_serializing_if for bools
    if !matches!(rust_kind, Kind::Bool { default: _ }) {
        if rust_field.required && !api_field.required {
            errors
                .push(ApiCheckError::FieldIsNotRequired { object: object_name, field: field_name });
        } else if !rust_field.required && api_field.required {
            errors
                .push(ApiCheckError::FieldIsNotOptional { object: object_name, field: field_name });
        }
    }
}

fn get_fields_of_api_object(api_schema: ApiSchema, api_object: Object) -> Vec<Property> {
    let mut fields = vec![];

    match api_object.data.clone() {
        ObjectData::Properties { properties } => fields.extend(properties),
        ObjectData::AnyOf { any_of } => {
            let mut reference_fields_arrays = vec![];

            // First, we convert an array of references into an array of their fields
            for value in any_of {
                if let Kind::Reference { reference } = value.0 {
                    let reference_object =
                        api_schema.objects.iter().find(|x| x.name == reference).unwrap();
                    let reference_fields =
                        get_fields_of_api_object(api_schema.clone(), reference_object.clone());

                    reference_fields_arrays.push(reference_fields);
                } else {
                    unreachable!("anyOf has not a reference");
                }
            }

            let reference_fields_arrays_clone = reference_fields_arrays.clone();

            // Second, we check them and add them to `fields`
            for reference_fields_array in reference_fields_arrays.iter_mut() {
                for reference_field in reference_fields_array.iter_mut() {
                    // This basically checks if all references have the same field. If yes, it is
                    // required. If not - set the required to false
                    if !(reference_fields_arrays_clone.iter().all(|x| {
                        x.iter().any(|y| {
                            y.name == reference_field.name
                                && y.kind == reference_field.kind
                                && y.required
                        })
                    })) {
                        reference_field.required = false;
                    }
                }
                extend_fields(&mut fields, reference_fields_array.to_vec());
            }
        }
        ObjectData::Unknown => {
            // Its an empty object
            fields = vec![];
        }
    }

    fields
}

pub fn check_object(
    api_schema: ApiSchema,
    rust_object: schemars::Schema,
    api_object_name: String,
    errors: &mut Vec<ApiCheckError>,
    exceptions: &Exceptions,
) {
    let official_types: Vec<String> = api_schema.objects.iter().map(|x| x.name.clone()).collect();
    let initial_object_name = rust_object.get("title").unwrap().as_str().unwrap().to_owned();

    let binding = api_schema.objects.clone();
    let api_object = binding.iter().find(|x| x.name == api_object_name).unwrap();

    let empty_object = Value::Object(serde_json::Map::new());
    let references = rust_object.get("$defs").unwrap_or(&empty_object);

    let rust_fields = get_fields_of_rust_object(
        rust_object.as_value(),
        official_types,
        references,
        initial_object_name.clone(),
        false,
        exceptions,
    );

    let api_fields = get_fields_of_api_object(api_schema, api_object.clone());

    for api_field in api_fields {
        if let Some(rust_field) = rust_fields.iter().find(|x| x.name == api_field.name) {
            check_struct_field(
                rust_field,
                &api_field,
                api_field.name.clone(),
                initial_object_name.clone(),
                errors,
                exceptions,
            );
        } else if !exceptions.is_field_ignored(api_field.name.clone())
            && !exceptions
                .is_object_field_ignored(initial_object_name.clone(), api_field.name.clone())
        {
            errors.push(ApiCheckError::FieldDoesNotExist {
                object: initial_object_name.clone(),
                field: api_field.name.clone(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{codegen::reformat, types::BusinessMessagesDeleted};
    use schemars::schema_for;

    #[test]
    fn test_rust_object_types() {
        let api_schema = get_api_schema();

        // Fields that are usually problematic are better to skip completely
        let mut exceptions = Exceptions::new(vec![
            // The `type` fields is usually a serde tag, and its messy
            Exception::IgnoreFieldName { field_name: "type".to_owned() },
            Exception::IgnoreFieldName { field_name: "transaction_type".to_owned() },
            Exception::IgnoreFieldName { field_name: "sticker_type".to_owned() },
            Exception::IgnoreFieldName { field_name: "source".to_owned() },
        ]);

        // Fields that usually have something different with Option<>
        exceptions.extend(vec![
            // file_size has a fallback
            Exception::IgnoreFieldRequiredName { field_name: "file_size".to_owned() },
        ]);

        // Some TBA types exist, but don't actually do anything
        exceptions
            .extend(vec![Exception::ExpandTelegramReferece { reference: "InputFile".to_owned() }]);

        let mut errors = vec![];

        check_object(
            api_schema.clone(),
            schema_for!(BusinessMessagesDeleted),
            "BusinessMessagesDeleted".to_owned(),
            &mut errors,
            &exceptions,
        );

        if !errors.is_empty() {
            let mut errors_string = String::new();
            for (i, error) in errors.iter().enumerate() {
                errors_string = format!("{errors_string}\n\n{}. {error}", i + 1);
            }
            panic!("schema.ron does not match the rust types. The errors are:\n\n{errors_string}",);
        }
    }

    #[test]
    #[allow(clippy::print_stdout)]
    fn codegen_types_checking() {
        let generator = "codegen_types_checking";

        // These are exceptions that apply to the types and objects
        // Fields that are usually problematic are better to skip completely
        let mut exceptions = Exceptions::new(vec![
            // The `type` fields is usually a serde tag, and its messy
            Exception::IgnoreFieldName { field_name: "type".to_owned() },
            Exception::IgnoreFieldName { field_name: "transaction_type".to_owned() },
            Exception::IgnoreFieldName { field_name: "sticker_type".to_owned() },
            Exception::IgnoreFieldName { field_name: "source".to_owned() },
        ]);

        // Fields that usually have something different with Option<>
        exceptions.extend(vec![
            // file_size has a fallback
            Exception::IgnoreFieldRequiredName { field_name: "file_size".to_owned() },
            // It doesn't exist in InaccessibleMessage because its always 0
            Exception::IgnoreFieldRequiredObjectName {
                object: "MaybeInaccessibleMessage".to_owned(),
                field_name: "date".to_owned(),
            },
            // They have a default
            Exception::IgnoreFieldRequiredObjectName {
                object: "PreCheckoutQuery".to_owned(),
                field_name: "order_info".to_owned(),
            },
            Exception::IgnoreFieldRequiredObjectName {
                object: "SuccessfulPayment".to_owned(),
                field_name: "order_info".to_owned(),
            },
            Exception::IgnoreFieldRequiredObjectName {
                object: "KeyboardButtonRequestUsers".to_owned(),
                field_name: "max_quantity".to_owned(),
            },
            Exception::IgnoreFieldRequiredObjectName {
                object: "KeyboardMarkup".to_owned(),
                field_name: "input_field_placeholder".to_owned(),
            },
            Exception::IgnoreFieldRequiredObjectName {
                object: "TextQuote".to_owned(),
                field_name: "entities".to_owned(),
            },
            Exception::IgnoreFieldRequiredObjectName {
                object: "InputSticker".to_owned(),
                field_name: "keywords".to_owned(),
            },
            Exception::IgnoreFieldRequiredObjectName {
                object: "File".to_owned(),
                field_name: "file_path".to_owned(),
            },
        ]);

        // Some fields in objects need to be ignored
        exceptions.extend(vec![
            // `User` has a separation with `Me`
            Exception::IgnoreObjectField {
                object: "User".to_owned(),
                field_name: "can_join_groups".to_owned(),
            },
            Exception::IgnoreObjectField {
                object: "User".to_owned(),
                field_name: "can_read_all_group_messages".to_owned(),
            },
            Exception::IgnoreObjectField {
                object: "User".to_owned(),
                field_name: "can_connect_to_business".to_owned(),
            },
            Exception::IgnoreObjectField {
                object: "User".to_owned(),
                field_name: "has_main_web_app".to_owned(),
            },
            Exception::IgnoreObjectField {
                object: "User".to_owned(),
                field_name: "supports_inline_queries".to_owned(),
            },
            // Date is always 0, so we omit it
            Exception::IgnoreObjectField {
                object: "InaccessibleMessage".to_owned(),
                field_name: "date".to_owned(),
            },
            // It has custom deser
            Exception::IgnoreObjectField {
                object: "PollAnswer".to_owned(),
                field_name: "voter_chat".to_owned(),
            },
            Exception::IgnoreObjectField {
                object: "PollAnswer".to_owned(),
                field_name: "user".to_owned(),
            },
            // Also a custom deser
            Exception::IgnoreObjectField {
                object: "MessageReactionUpdated".to_owned(),
                field_name: "actor_chat".to_owned(),
            },
            Exception::IgnoreObjectField {
                object: "MessageReactionUpdated".to_owned(),
                field_name: "user".to_owned(),
            },
            // Its a tag
            Exception::IgnoreObjectField {
                object: "ChatMember".to_owned(),
                field_name: "status".to_owned(),
            },
        ]);

        // Some TBA types exist, but don't actually do anything
        exceptions
            .extend(vec![Exception::ExpandTelegramReferece { reference: "InputFile".to_owned() }]);

        // These are exceptions that apply to codegen
        let mut objects_exceptions = Exceptions::new(vec![
            // Update has custom serialization, and it probably won't be wrong any time soon
            Exception::RemoveObjectFromChecking { object: "Update".to_owned() },
            // They are in a single struct
            Exception::RemoveObjectFromChecking { object: "MessageOriginUser".to_owned() },
            Exception::RemoveObjectFromChecking { object: "MessageOriginHiddenUser".to_owned() },
            Exception::RemoveObjectFromChecking { object: "MessageOriginChat".to_owned() },
            Exception::RemoveObjectFromChecking { object: "MessageOriginChannel".to_owned() },
            // They are checked in ChatMember
            Exception::RemoveObjectFromChecking { object: "ChatMemberOwner".to_owned() },
            Exception::RemoveObjectFromChecking { object: "ChatMemberAdministrator".to_owned() },
            Exception::RemoveObjectFromChecking { object: "ChatMemberMember".to_owned() },
            Exception::RemoveObjectFromChecking { object: "ChatMemberRestricted".to_owned() },
            Exception::RemoveObjectFromChecking { object: "ChatMemberLeft".to_owned() },
            Exception::RemoveObjectFromChecking { object: "ChatMemberBanned".to_owned() },
            // They are a part of ReactionType
            Exception::RemoveObjectFromChecking { object: "ReactionTypeEmoji".to_owned() },
            Exception::RemoveObjectFromChecking { object: "ReactionTypeCustomEmoji".to_owned() },
            Exception::RemoveObjectFromChecking { object: "ReactionTypePaid".to_owned() },
            // They are a part of BotCommandScope
            Exception::RemoveObjectFromChecking { object: "BotCommandScopeDefault".to_owned() },
            Exception::RemoveObjectFromChecking {
                object: "BotCommandScopeAllPrivateChats".to_owned(),
            },
            Exception::RemoveObjectFromChecking {
                object: "BotCommandScopeAllGroupChats".to_owned(),
            },
            Exception::RemoveObjectFromChecking {
                object: "BotCommandScopeAllChatAdministrators".to_owned(),
            },
            Exception::RemoveObjectFromChecking { object: "BotCommandScopeChat".to_owned() },
            Exception::RemoveObjectFromChecking {
                object: "BotCommandScopeChatAdministrators".to_owned(),
            },
            Exception::RemoveObjectFromChecking { object: "BotCommandScopeChatMember".to_owned() },
            // They are a part of MenuButton
            Exception::RemoveObjectFromChecking { object: "MenuButtonCommands".to_owned() },
            Exception::RemoveObjectFromChecking { object: "MenuButtonWebApp".to_owned() },
            Exception::RemoveObjectFromChecking { object: "MenuButtonDefault".to_owned() },
            // They are a part of RevenueWithdrawalState
            Exception::RemoveObjectFromChecking {
                object: "RevenueWithdrawalStatePending".to_owned(),
            },
            Exception::RemoveObjectFromChecking {
                object: "RevenueWithdrawalStateFailed".to_owned(),
            },
            // They are a part of TransactionPartner
            Exception::RemoveObjectFromChecking {
                object: "TransactionPartnerTelegramAds".to_owned(),
            },
            Exception::RemoveObjectFromChecking { object: "TransactionPartnerOther".to_owned() },
            // They are checked with PassportElementError, and they don't have a `message` field,
            // which PassportElementError has
            Exception::RemoveObjectFromChecking {
                object: "PassportElementErrorDataField".to_owned(),
            },
            Exception::RemoveObjectFromChecking {
                object: "PassportElementErrorFrontSide".to_owned(),
            },
            Exception::RemoveObjectFromChecking {
                object: "PassportElementErrorReverseSide".to_owned(),
            },
            Exception::RemoveObjectFromChecking { object: "PassportElementErrorSelfie".to_owned() },
            Exception::RemoveObjectFromChecking { object: "PassportElementErrorFile".to_owned() },
            Exception::RemoveObjectFromChecking { object: "PassportElementErrorFiles".to_owned() },
            Exception::RemoveObjectFromChecking {
                object: "PassportElementErrorTranslationFile".to_owned(),
            },
            Exception::RemoveObjectFromChecking {
                object: "PassportElementErrorTranslationFiles".to_owned(),
            },
            Exception::RemoveObjectFromChecking {
                object: "PassportElementErrorUnspecified".to_owned(),
            },
        ]);

        objects_exceptions.extend(vec![
            Exception::RenameCheckingObject {
                api_object_name: "ReplyKeyboardMarkup".to_owned(),
                rust_object_name: "KeyboardMarkup".to_owned(),
            },
            Exception::RenameCheckingObject {
                api_object_name: "ReplyKeyboardRemove".to_owned(),
                rust_object_name: "KeyboardRemove".to_owned(),
            },
            Exception::RenameCheckingObject {
                api_object_name: "InputTextMessageContent".to_owned(),
                rust_object_name: "InputMessageContentText".to_owned(),
            },
            Exception::RenameCheckingObject {
                api_object_name: "InputLocationMessageContent".to_owned(),
                rust_object_name: "InputMessageContentLocation".to_owned(),
            },
            Exception::RenameCheckingObject {
                api_object_name: "InputVenueMessageContent".to_owned(),
                rust_object_name: "InputMessageContentVenue".to_owned(),
            },
            Exception::RenameCheckingObject {
                api_object_name: "InputContactMessageContent".to_owned(),
                rust_object_name: "InputMessageContentContact".to_owned(),
            },
            Exception::RenameCheckingObject {
                api_object_name: "InputInvoiceMessageContent".to_owned(),
                rust_object_name: "InputMessageContentInvoice".to_owned(),
            },
        ]);

        let mut string_exceptions = String::new();

        for exception in exceptions.exceptions {
            string_exceptions = format!("{}, {string_exceptions}", exception.to_eval_rust());
        }

        let api_schema = get_api_schema();

        let mut string_check_objects = String::new();
        for object in api_schema.objects {
            if objects_exceptions.is_object_removed_from_checking(object.name.clone()) {
                continue;
            }

            let mut rust_object_name = object.name.clone();

            if let Some(renamed) = objects_exceptions.get_renamed(object.name.clone()) {
                rust_object_name = renamed
            }
            string_check_objects = format!(
                "{string_check_objects}\n    check_object(api_schema.clone(), \
                 schema_for!({rust_object_name}), \"{}\".to_owned(), &mut errors, &exceptions);",
                object.name
            )
        }

        let mut contents = format!(
            "
//! Generated by `{generator}`, do not edit by hand.

use crate::types::*;
use crate::codegen::schema_check::rust_types_check_codegen::*;
use crate::codegen::schema_check::api_schema::*;
use schemars::schema_for;

#[test]
fn test_rust_objects() {{
    let api_schema = get_api_schema();
    
    let exceptions = Exceptions::new(vec![{string_exceptions}]);
    
    let mut errors = vec![];
    
    {string_check_objects}

    if !errors.is_empty() {{
        let mut errors_string = String::new();
        for (i, error) in errors.iter().enumerate() {{
            errors_string = format!(\"{{errors_string}}\\n\\n{{}}. {{error}}\", i + 1);
        }}
        panic!(\"schema.ron does not match the rust types. The errors \
             are:\\n\\n{{errors_string}}\",);
    }}
}}
"
        );

        contents = reformat(contents);
        println!("{contents}");
    }
}
