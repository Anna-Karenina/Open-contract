use chrono::NaiveDateTime;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Default)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub password_hash: Option<String>,
    pub is_active: bool,
    pub photo_url: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub email: String,
    pub name: String,
    pub password_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i32,
    pub email: String,
    pub name: String,
}

impl From<User> for AuthUser {
    fn from(user: User) -> Self {
        AuthUser {
            id: user.id,
            email: user.email,
            name: user.name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,   // user id
    pub exp: usize, // expiry timestamp
}
