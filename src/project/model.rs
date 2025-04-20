use crate::schema::*;
use crate::user::model::User;
use chrono::NaiveDateTime;
use diesel::Queryable;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    pub project_link: Option<String>,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
    pub proto_file: Option<String>,
    pub creator_id: i32,
    pub collaborators: i32,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name=projects)]
pub struct NewProject {
    pub name: String,
    pub project_link: Option<String>,
    pub description: Option<String>,
    pub proto_file: Option<String>,
    pub creator_id: Option<i32>,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[diesel(table_name = projects)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub project_link: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Queryable)]
pub struct ProjectCollaborators {
    pub collaborators: Vec<User>,
}
