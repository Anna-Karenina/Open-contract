use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::prelude::*;
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
}
