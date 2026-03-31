//! Credit to https://github.com/ark0f/tg-bot-api and https://github.com/ENCRYPTEDFOREVER/tg-bot-api/tree/bot_api_9_0
use crate::codegen::project_root;
use serde::Deserialize;
use std::fs;

pub fn get_api_schema() -> ApiSchema {
    let path = project_root().join("custom_v2.json");
    let text = fs::read_to_string(path).unwrap();
    let schema: ApiSchema = serde_json::from_str(&text).unwrap();

    schema
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct ApiSchema {
    pub version: Version,
    pub recent_changes: Date,
    pub methods: Vec<ApiMethod>,
    pub objects: Vec<Object>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Kind {
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
    Null,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct KindWrapper(pub Kind);

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ApiMethod {
    pub name: String,
    pub description: String,
    pub arguments: Vec<Argument>,
    pub maybe_multipart: bool,
    pub return_type: KindWrapper,
    pub documentation_link: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Argument {
    pub name: String,
    pub description: String,
    pub required: bool,
    #[serde(rename = "type_info")]
    pub kind: KindWrapper,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Object {
    pub name: String,
    pub description: String,
    #[serde(flatten)]
    pub data: ObjectData,
    pub documentation_link: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum ObjectData {
    Properties { properties: Vec<Property> },
    AnyOf { any_of: Vec<KindWrapper> },
    Unknown,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Property {
    pub name: String,
    pub description: String,
    pub required: bool,
    #[serde(rename = "type_info")]
    pub kind: KindWrapper,
}
