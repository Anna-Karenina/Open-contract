use crate::{schema::*, user::model::User};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::Selectable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(tag = "grant_type")]
pub enum LoginRequest {
    #[serde(rename = "password")]
    Password { email: String, password: String },
    #[serde(rename = "oauth")]
    OAuth { provider: String, token: String },
    #[serde(rename = "token")]
    Token { token: String },
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize,
}

/// Активная сессия пользователя в системе
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: Uuid,
    #[diesel(column_name = user_id)]
    pub user_id: i32,
    pub token: String,
    #[diesel(column_name = ip_address)]
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub is_active: bool,
}

/// Данные для создания новой сессии
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub user_id: i32,
    pub token: String,
    pub ip_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    pub expires_at: NaiveDateTime,
}

/// Данные для обновления сессии
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = sessions)]
pub struct UpdateSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// DTO для отдачи клиенту (без чувствительных данных)
#[derive(Debug, Serialize)]
pub struct SessionInfo {
    pub id: Uuid,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub is_current: bool,
}

impl From<Session> for SessionInfo {
    fn from(session: Session) -> Self {
        SessionInfo {
            id: session.id,
            ip_address: session.ip_address,
            user_agent: session.user_agent,
            created_at: session.created_at,
            expires_at: session.expires_at,
            is_current: false, // Заполняется отдельно
        }
    }
}

/// Ответ с информацией о сессиях пользователя
#[derive(Debug, Serialize)]
pub struct UserSessionsResponse {
    pub current_session: SessionInfo,
    pub other_sessions: Vec<SessionInfo>,
}

/// Параметры для фильтрации сессий
#[derive(Debug, Deserialize)]
pub struct SessionFilter {
    #[serde(default)]
    pub active_only: bool,
    #[serde(default)]
    pub expired_only: bool,
    #[serde(with = "chrono::serde::ts_seconds_option", default)]
    pub created_after: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option", default)]
    pub created_before: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<i64>,
    pub refresh_token: Option<String>,
}

/// Запрос на OAuth аутентификацию
#[derive(Debug, Deserialize)]
pub struct OAuthRequest {
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Debug)]
pub struct OAuthUserInfo {
    pub email: String,
    pub username: String,
}
