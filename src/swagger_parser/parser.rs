use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

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

#[derive(Debug, Deserialize, Serialize)]
struct SwaggerDoc {
    paths: HashMap<String, PathItem>,
    definitions: Option<HashMap<String, Schema>>,
    components: Option<Components>,
    #[serde(rename = "$ref")]
    refs: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Components {
    schemas: Option<HashMap<String, Schema>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PathItem {
    get: Option<Operation>,
    post: Option<Operation>,
    put: Option<Operation>,
    delete: Option<Operation>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Operation {
    responses: HashMap<String, Response>,
    parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Parameter {
    name: String,
    schema: Option<SchemaRef>,
    r#in: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    description: Option<String>,
    schema: Option<SchemaRef>,
    content: Option<HashMap<String, MediaType>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MediaType {
    schema: Option<SchemaRef>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum SchemaRef {
    Ref { r#ref: String },
    Inline(Box<Schema>),
}

#[derive(Debug, Deserialize, Serialize)]
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

struct ProtoGenerator {
    messages: HashMap<String, String>,
    current_field_number: u32,
}

impl ProtoGenerator {
    fn new() -> Self {
        Self {
            messages: HashMap::new(),
            current_field_number: 1,
        }
    }

    fn generate_proto(&mut self, swagger: &SwaggerDoc) -> Result<String, SwaggerToProtoError> {
        let mut proto_content = String::from("syntax = \"proto3\";\n\n");

        if let Some(definitions) = &swagger.definitions {
            self.generate_messages(&mut proto_content, definitions)?;
        }

        if let Some(components) = &swagger.components {
            if let Some(schemas) = &components.schemas {
                self.generate_messages(&mut proto_content, schemas)?;
            }
        }

        self.generate_services(&mut proto_content, &swagger.paths)?;

        Ok(proto_content)
    }

    fn generate_messages(
        &mut self,
        proto: &mut String,
        schemas: &HashMap<String, Schema>,
    ) -> Result<(), SwaggerToProtoError> {
        for (name, schema) in schemas {
            if self.messages.contains_key(name) {
                continue;
            }

            let message_content = self.convert_schema_to_message(name, schema)?;
            proto.push_str(&message_content);
            self.messages.insert(name.clone(), message_content);
        }
        Ok(())
    }

    fn convert_schema_to_message(
        &mut self,
        name: &str,
        schema: &Schema,
    ) -> Result<String, SwaggerToProtoError> {
        let mut message = format!("message {} {{\n", name);
        let mut field_number = 1;

        if let Some(properties) = &schema.properties {
            for (field_name, field_schema) in properties {
                let proto_type = self.map_schema_to_proto_type(field_schema)?;
                message.push_str(&format!(
                    "  {} {} = {};\n",
                    proto_type, field_name, field_number
                ));
                field_number += 1;
            }
        }

        message.push_str("}\n\n");
        Ok(message)
    }

    fn map_schema_to_proto_type(&mut self, schema: &Schema) -> Result<String, SwaggerToProtoError> {
        match schema.r#type.as_deref() {
            Some("integer") => match schema.format.as_deref() {
                Some("int64") => Ok("int64".to_string()),
                Some("int32") => Ok("int32".to_string()),
                _ => Ok("int32".to_string()),
            },
            Some("number") => match schema.format.as_deref() {
                Some("double") => Ok("double".to_string()),
                Some("float") => Ok("float".to_string()),
                _ => Ok("double".to_string()),
            },
            Some("boolean") => Ok("bool".to_string()),
            Some("string") => match schema.format.as_deref() {
                Some("date") | Some("date-time") => Ok("string".to_string()),
                Some("byte") => Ok("bytes".to_string()),
                _ => Ok("string".to_string()),
            },
            Some("array") => {
                let items_schema = schema
                    .items
                    .as_ref()
                    .ok_or(SwaggerToProtoError::InvalidArrayDefinition)?;

                let item_type = match items_schema.as_ref() {
                    SchemaRef::Ref { r#ref } => {
                        r#ref.split('/').last().unwrap_or("string").to_string()
                    }
                    SchemaRef::Inline(schema) => self.map_schema_to_proto_type(schema)?,
                };

                Ok(format!("repeated {}", item_type))
            }
            Some("object") => {
                if schema.additional_properties.is_some() {
                    Ok("map<string, string>".to_string())
                } else {
                    Err(SwaggerToProtoError::UnsupportedSchemaType(
                        "complex object".to_string(),
                    ))
                }
            }
            None if schema.ref_path.is_some() => {
                let ref_path = schema.ref_path.as_ref().unwrap();
                Ok(ref_path.split('/').last().unwrap_or("string").to_string())
            }
            _ => Err(SwaggerToProtoError::UnsupportedSchemaType(
                schema.r#type.clone().unwrap_or("unknown".to_string()),
            )),
        }
    }

    fn generate_services(
        &mut self,
        proto: &mut String,
        paths: &HashMap<String, PathItem>,
    ) -> Result<(), SwaggerToProtoError> {
        proto.push_str("service ApiService {\n");

        for (path, item) in paths {
            if let Some(get) = &item.get {
                let method_name = path_to_method_name(path, "Get");
                proto.push_str(&format!(
                    "  rpc {} (Empty) returns (Response);\n",
                    method_name
                ));
            }
        }

        proto.push_str("}\n\n");
        Ok(())
    }
}

fn path_to_method_name(path: &str, prefix: &str) -> String {
    let clean_path = path
        .trim_matches('/')
        .replace('/', "_")
        .replace('{', "")
        .replace('}', "");

    format!("{}{}", prefix, to_pascal_case(&clean_path))
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        })
        .collect()
}

// usage
/*
fn main() -> Result<(), SwaggerToProtoError> {
    let swagger_json = fs::read_to_string("swagger.json")?;
    let swagger: SwaggerDoc = serde_json::from_str(&swagger_json)?;

    let mut generator = ProtoGenerator::new();
    let proto_content = generator.generate_proto(&swagger)?;

    fs::write("api.proto", proto_content)?;
    println!("Successfully generated api.proto");

    Ok(())
}
 */
