use crate::codegen::schema_check::api_schema::*;
use derive_more::derive::Display;
use serde_json::{Map, Value};

fn convert_reference_string(reference: String, initial_object_name: String) -> String {
    reference.trim_start_matches("#/$defs/").replace("#", &initial_object_name).to_string()
}

#[derive(Debug, Clone, PartialEq)]
enum Exception {
    IgnoreObject { object: String },
    IgnoreFieldName { field_name: String },
    IgnoreFieldRequiredName { field_name: String },
}

#[derive(Debug, Clone, PartialEq)]
struct Exceptions {
    exceptions: Vec<Exception>,
}

impl Exceptions {
    fn new(exceptions: Vec<Exception>) -> Self {
        Self { exceptions }
    }

    fn is_object_ignored(&self, object: String) -> bool {
        self.exceptions.contains(&Exception::IgnoreObject { object })
    }

    fn is_field_ignored(&self, field_name: String) -> bool {
        self.exceptions.contains(&Exception::IgnoreFieldName { field_name })
    }

    fn is_field_required_ignored(&self, field_name: String) -> bool {
        self.exceptions.contains(&Exception::IgnoreFieldRequiredName { field_name })
    }
}

#[derive(Debug, Display)]
enum ApiCheckError {
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
            for variant in array {
                match get_type(variant, initial_object_name.clone()) {
                    (_, Kind::Null) => required = false,
                    (_, Kind::Reference { reference }) => {
                        prop_type = Some(Type::Reference { reference })
                    }
                    _ => unreachable!("Nothing else should be in anyOf"),
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
    } else {
        panic!("No type source was found, {prop:?}")
    }

    if prop_type == Some(Type::Array) {
        if let Some(item) = prop.get("items") {
            item_kind = Some(get_type(item, initial_object_name.clone()).1);
        } else {
            panic!("Array without items!");
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
        _ => unreachable!("Any other doesn't make sense"),
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
    all_fields_are_optional: bool,
) -> Vec<Property> {
    let mut fields = vec![];

    for value in array {
        if value.get("type").map(Value::as_str) == Some("object".into()) {
            let x = get_fields_of_rust_object(
                &value,
                official_types.clone(),
                references,
                initial_object_name.clone(),
                all_fields_are_optional,
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
                (!required) || all_fields_are_optional,
            ));
        } else {
            unreachable!("Array has not a reference")
        }
    }

    fields
}

fn extract_fields_from_properties_object(
    rust_object: &Map<String, Value>,
    official_types: Vec<String>,
    references: &Value,
    initial_object_name: String,
    all_fields_are_optional: bool,
) -> Vec<Property> {
    let mut fields = vec![];

    for (name, prop) in rust_object {
        let (required, mut kind) = get_type(prop, initial_object_name.clone());
        if let Kind::Reference { ref reference } = kind {
            // If its a reference that isnt in official types, its probably a wrapper for a
            // type
            if !official_types.contains(reference) {
                let reference_object = references
                    .get(reference)
                    .unwrap_or_else(|| panic!("{name}, {kind:?}, {reference:?}"));

                // A check if its really a wrapper and not smth like FileMeta
                if reference_object.get("type").unwrap().as_str() != Some("object") {
                    let (reference_required, reference_kind) =
                        get_type(reference_object, initial_object_name.clone());
                    assert!(reference_required);

                    kind = reference_kind;
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
    ignore_field_names: &Exceptions,
    errors: &mut Vec<ApiCheckError>,
) {
    // Fields that usually have something different with Option<>
    let ignore_fields_required = Exceptions::new(vec![
        // file_size has a fallback
        Exception::IgnoreFieldRequiredName { field_name: "file_size".to_owned() },
    ]);

    if ignore_field_names.is_field_ignored(field_name.clone())
        || ignore_fields_required.is_field_required_ignored(field_name.clone())
    {
        return;
    }

    let rust_kind = rust_field.kind.0.clone();
    let api_kind = api_field.kind.0.clone();

    check_struct_kind(&rust_kind, &api_kind, field_name.clone(), object_name.clone(), errors);
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
            for value in any_of {
                if let Kind::Reference { reference } = value.0 {
                    let reference_object =
                        api_schema.objects.iter().find(|x| x.name == reference).unwrap();
                    let mut reference_fields =
                        get_fields_of_api_object(api_schema.clone(), reference_object.clone());

                    for reference_field in reference_fields.iter_mut() {
                        reference_field.required = false;
                    }
                    extend_fields(&mut fields, reference_fields);
                } else {
                    unreachable!("anyOf has not a reference");
                }
            }
        }
        ObjectData::Unknown => unreachable!("Unknown object data"),
    }

    fields
}

fn check_object(
    api_schema: ApiSchema,
    rust_object: schemars::Schema,
    errors: &mut Vec<ApiCheckError>,
) {
    // Fields that are usually problematic are better to skip completely
    let ignore_field_names = Exceptions::new(vec![
        // The `type` fields is usually a serde tag, and its messy
        Exception::IgnoreFieldName { field_name: "type".to_owned() },
        Exception::IgnoreFieldName { field_name: "transaction_type".to_owned() },
    ]);

    let official_types: Vec<String> = api_schema.objects.iter().map(|x| x.name.clone()).collect();
    let initial_object_name = rust_object.get("title").unwrap().as_str().unwrap().to_owned();

    let binding = api_schema.objects.clone();
    let api_object = binding.iter().find(|x| x.name == initial_object_name).unwrap();

    let empty_object = Value::Object(serde_json::Map::new());
    let references = rust_object.get("$defs").unwrap_or(&empty_object);

    let rust_fields = get_fields_of_rust_object(
        rust_object.as_value(),
        official_types,
        references,
        initial_object_name.clone(),
        false,
    );

    let api_fields = get_fields_of_api_object(api_schema, api_object.clone());

    for api_field in api_fields {
        if let Some(rust_field) = rust_fields.iter().find(|x| x.name == api_field.name) {
            check_struct_field(
                rust_field,
                &api_field,
                api_field.name.clone(),
                initial_object_name.clone(),
                &ignore_field_names,
                errors,
            );
        } else {
            if !ignore_field_names.is_field_ignored(api_field.name.clone()) {
                errors.push(ApiCheckError::FieldDoesNotExist {
                    object: initial_object_name.clone(),
                    field: api_field.name.clone(),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message;
    use schemars::schema_for;

    #[test]
    fn test_rust_object() {
        let api_schema = get_api_schema();
        let ignore_objects =
            Exceptions::new(vec![Exception::IgnoreObject { object: "Update".to_string() }]);

        let mut errors = vec![];

        check_object(api_schema.clone(), schema_for!(Message), &mut errors);

        if !errors.is_empty() {
            let mut errors_string = String::new();
            for (i, error) in errors.iter().enumerate() {
                errors_string = format!("{errors_string}\n\n{}. {error}", i + 1);
            }
            panic!("schema.ron does not match the rust types. The errors are:\n\n{errors_string}",);
        }
    }
}
