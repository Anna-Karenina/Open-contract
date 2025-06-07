use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use dot_proto_parser::{
    ConverterError, Field, FieldRule, Message, Method, NameFormatter, ProtoFile, Service,
};
use serde::{Deserialize, Serialize};

use crate::schema::contracts;

pub struct DiffLine {
    pub line_number: usize,
    pub old_line: String,
    pub new_line: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionMetadata {
    pub id: String,
    pub timestamp: String,
    pub author: String,
    pub size: usize,
}

#[derive(Deserialize, Debug)]
pub struct SaveFileRequest {
    pub content: String,
}

#[derive(Deserialize, Queryable, Serialize, Default, Debug)]

pub struct Contract {
    pub id: i32,
    pub project_id: i32,
    pub author_id: i32,
    pub grpc_method: String,
    pub tag: Option<String>,
    pub errors_response: Option<String>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub body: Option<String>,
    pub response: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub http_method: Option<String>,
    pub description: Option<String>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name=contracts)]
pub struct CrateContract {
    pub project_id: i32,
    pub author_id: i32,
    pub grpc_method: String,
    pub tag: Option<String>,
    pub errors_response: Option<String>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub body: Option<String>,
    pub response: Option<String>,
    pub http_method: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub tag: String,
    pub method: String,
    pub path: String,
    #[serde(default)]
    pub path_params: String,
    pub project_id: String,
    pub body: HashMap<String, BodyField>,
    pub response: HashMap<String, BodyField>,
    #[serde(default)]
    pub existing_response_message: Option<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BodyField {
    // Для объектов с fields (order)
    ObjectType {
        #[serde(rename = "type")]
        field_type: String,
        #[serde(rename = "is_required", default)]
        is_required: bool,
        #[serde(default)]
        fields: HashMap<String, BodyField>,
    },
    // Для массивов (items, shops_id)
    ArrayType(Vec<BodyField>),
    // Для простых определений (created_at)
    SimpleType {
        #[serde(rename = "type")]
        field_type: String,
        #[serde(rename = "is_required", default)]
        is_required: bool,
    },
    // Для вложенных объектов без type (items[0])
    NestedObject(HashMap<String, BodyField>),
}

impl NameFormatter for ServiceRequest {}

impl ServiceRequest {
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        let value: serde_json::Value =
            serde_json::from_str(json_str).map_err(|e| format!("Invalid JSON: {}", e))?;

        // Ручная валидация структуры
        if !value.is_object() {
            return Err("Root should be an object".into());
        }

        serde_json::from_str(json_str).map_err(|e| format!("Structure mismatch: {}", e))
    }

    pub fn add_to_proto_context(
        &self,
        mut proto_file: ProtoFile,
    ) -> Result<ProtoFile, ConverterError> {
        let request_message_name = format!("{}Request", self.to_pascal_case(&self.method));
        let mut request_message = Message::new(&request_message_name);

        if !self.description.is_empty() {
            request_message.add_comment(&self.description);
        }

        self.add_fields_to_message(&mut request_message, &self.body)?;

        let response_message_name = if let Some(existing) = &self.existing_response_message {
            existing.clone()
        } else {
            let name = format!("{}Response", self.to_pascal_case(&self.method));
            let mut response_message = Message::new(&name);
            self.add_fields_to_message(&mut response_message, &self.response)?;
            proto_file.add_message(response_message)?;
            name
        };

        let service_name = format!("{}Service", self.to_pascal_case(&self.tag));
        let mut service = proto_file
            .find_service_mut(&service_name)
            .map(|s| s.clone())
            .unwrap_or_else(|| {
                let mut s = Service::new(&service_name);
                if !self.tag.is_empty() {
                    s.add_comment(&format!("Service for {}", self.tag));
                }
                s
            });

        let mut method = Method::new(&self.method, &request_message_name, &response_message_name);

        method.add_comment(&self.description);
        method.add_option("http_method", &self.method);
        method.add_option("http_path", &self.path);

        service.add_method(method)?;

        if proto_file.find_service(&service_name).is_none() {
            proto_file.add_service(service)?;
        }

        proto_file.add_message(request_message)?;

        Ok(proto_file)
    }

    fn add_fields_to_message(
        &self,
        message: &mut Message,
        fields: &HashMap<String, BodyField>,
    ) -> Result<(), ConverterError> {
        let mut field_number = 1;

        for (field_name, field) in fields {
            let (field_type, rule) = self.convert_body_field(field, &message.name)?;

            let mut proto_field = Field::new(
                &self.sanitize_field_name(field_name),
                &field_type,
                field_number,
                rule,
            );

            if let BodyField::SimpleType { is_required, .. } = field {
                if *is_required {
                    proto_field.add_comment("Required field");
                }
            }

            message.add_field(proto_field)?;
            field_number += 1;
        }

        Ok(())
    }

    fn convert_body_field(
        &self,
        field: &BodyField,
        context_name: &str,
    ) -> Result<(String, FieldRule), ConverterError> {
        match field {
            BodyField::SimpleType {
                field_type,
                is_required,
            } => {
                let proto_type = match field_type.as_str() {
                    "string" => "string".to_string(),
                    "number" => "double".to_string(),
                    "integer" => "int64".to_string(),
                    "boolean" => "bool".to_string(),
                    "date" | "datetime" => "google.protobuf.Timestamp".to_string(),
                    other => other.to_string(), // предполагаем, что это уже proto-тип
                };

                let rule = if *is_required {
                    FieldRule::Required
                } else {
                    FieldRule::Optional
                };

                Ok((proto_type, rule))
            }

            BodyField::ObjectType {
                field_type,
                fields,
                is_required,
            } => {
                // Если это встроенный тип (например, map)
                if field_type == "object" && !fields.is_empty() {
                    // Генерируем уникальное имя для вложенного сообщения
                    let nested_name = format!("{}Nested{}", context_name, rand::random::<u32>());
                    let mut nested_message = Message::new(&nested_name);
                    self.add_fields_to_message(&mut nested_message, fields)?;

                    Ok((
                        nested_name,
                        if *is_required {
                            FieldRule::Required
                        } else {
                            FieldRule::Optional
                        },
                    ))
                } else {
                    Ok((
                        field_type.clone(),
                        if *is_required {
                            FieldRule::Required
                        } else {
                            FieldRule::Optional
                        },
                    ))
                }
            }

            BodyField::ArrayType(items) => {
                if items.is_empty() {
                    return Err(ConverterError::InvalidFieldName(
                        "Empty array type".to_string(),
                    ));
                }

                // Берем тип первого элемента как тип массива
                let (item_type, _) = self.convert_body_field(&items[0], context_name)?;
                Ok((format!("repeated {}", item_type), FieldRule::Optional))
            }

            BodyField::NestedObject(fields) => {
                // Генерируем уникальное имя для вложенного сообщения
                let nested_name = format!("{}Nested{}", context_name, rand::random::<u32>());
                let mut nested_message = Message::new(&nested_name);
                self.add_fields_to_message(&mut nested_message, fields)?;

                Ok((nested_name, FieldRule::Optional))
            }
        }
    }
}
