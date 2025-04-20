use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod parse_utils;
pub mod parser;

#[derive(Error, Debug)]
pub enum SwaggerToProtoError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Unsupported schema type: {0}")]
    UnsupportedSchemaType(String),
    #[error("Missing reference: {0}")]
    MissingReference(String),
    #[error("Invalid array definition")]
    InvalidArrayDefinition,
    #[error("Unresolvable schema")]
    UnresolvableSchema,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct SwaggerDoc {
    paths: HashMap<String, PathItem>,
    definitions: Option<HashMap<String, Schema>>,
    components: Option<Components>,
    #[serde(rename = "$ref")]
    refs: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Components {
    schemas: Option<HashMap<String, Schema>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct PathItem {
    get: Option<Operation>,
    post: Option<Operation>,
    put: Option<Operation>,
    delete: Option<Operation>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Operation {
    responses: HashMap<String, Response>,
    parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Parameter {
    name: String,
    schema: Option<SchemaRef>,
    r#in: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Response {
    description: Option<String>,
    schema: Option<SchemaRef>,
    content: Option<HashMap<String, MediaType>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct MediaType {
    schema: Option<SchemaRef>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum SchemaRef {
    Ref { r#ref: String },
    Inline(Box<Schema>),
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Schema {
    r#type: Option<String>,
    format: Option<String>,
    items: Option<Box<SchemaRef>>,
    properties: Option<HashMap<String, Schema>>,
    additional_properties: Option<Box<SchemaRef>>,
    enum_values: Option<Vec<String>>,
    #[serde(rename = "$ref")]
    ref_path: Option<String>,
}
