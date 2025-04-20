use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Queryable, Serialize, Default, Debug)]

pub struct Comment {
    pub id: i32,
    pub author: i32,
    pub comment: String,
    pub created_at: NaiveDateTime,
    pub contract_id: Option<i32>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name=comments)]
pub struct NewComment {
    pub author: Option<i32>,
    pub comment: String,
    #[serde(deserialize_with = "parse_i32_from_str")]
    pub contract_id: i32,
}

fn parse_i32_from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<i32>().map_err(serde::de::Error::custom)
}
