use crate::auth::AuthError;
use crate::user::{
    model::{NewUser, User},
    repository::UserRepository,
};

use super::{
    jwt::{generate_jwt_token, validate_jwt_token, verify_password},
    models::{LoginRequest, LoginResponse, NewSession, OAuthUserInfo},
    session_repository::SessionRepository,
};

use chrono::{Duration, Utc};
use diesel::PgConnection;

pub struct AuthService;

impl AuthService {
    pub async fn login(
        conn: &mut PgConnection,
        request: LoginRequest,
        ip: &str,
        user_agent: Option<String>,
    ) -> Result<LoginResponse, AuthError> {
        let email = match &request {
            LoginRequest::Password { email, .. } => Some(email.as_str()),
            _ => None,
        };

        let result = match request {
            LoginRequest::Password { email, password } => {
                Self::password_login(conn, &email, &password, ip, user_agent).await
            }
            LoginRequest::OAuth { provider, token } => {
                Self::oauth_login(conn, &provider, &token, ip, user_agent).await
            }
            LoginRequest::Token { token } => Self::token_login(conn, &token, ip, user_agent).await,
        };

        result
    }

    async fn password_login(
        conn: &mut PgConnection,
        email: &str,
        password: &str,
        ip: &str,
        user_agent: Option<String>,
    ) -> Result<LoginResponse, AuthError> {
        let user =
            UserRepository::find_by_email(conn, email).map_err(|_| AuthError::UserNotFound)?;

        if !user.is_active {
            return Err(AuthError::UserNotActive);
        }

        let hash = user.password_hash.clone().unwrap();
        if !verify_password(password, &hash) {
            return Err(AuthError::InvalidCredentials);
        }

        Self::create_session(conn, user, ip, user_agent).await
    }

    pub async fn token_login(
        conn: &mut PgConnection,
        token: &str,
        ip: &str,
        user_agent: Option<String>,
    ) -> Result<LoginResponse, AuthError> {
        // Валидация токена
        let user_id = validate_jwt_token(token)?;

        // Поиск пользователя
        let user =
            UserRepository::find_by_id(conn, user_id).map_err(|_| AuthError::UserNotFound)?;

        if !user.is_active {
            return Err(AuthError::UserNotActive);
        }

        // Проверка существующей сессии
        if let Ok(session) = SessionRepository::find_valid_by_token(conn, token, ip) {
            // Обновляем срок действия существующей сессии
            let new_expires = (Utc::now() + Duration::days(7)).naive_utc();
            let updated_session = SessionRepository::refresh_session(conn, session.id, new_expires)
                .map_err(|_| AuthError::DatabaseError)?;

            return Ok(LoginResponse {
                token: updated_session.token,
                user: user.into(),
            });
        }

        // Создаем новую сессию если не нашли существующую
        Self::create_session(conn, user, ip, user_agent).await
    }

    pub async fn oauth_login(
        conn: &mut PgConnection,
        provider: &str,
        oauth_token: &str,
        ip: &str,
        user_agent: Option<String>,
    ) -> Result<LoginResponse, AuthError> {
        // Получаем данные пользователя от провайдера
        let user_info = match provider.to_lowercase().as_str() {
            "google" => Self::verify_google_token(oauth_token).await?,
            "github" => Self::verify_github_token(oauth_token).await?,
            _ => return Err(AuthError::ProviderError("Unsupported provider".into())),
        };

        // Ищем или создаем пользователя
        let user = UserRepository::find_by_email(conn, &user_info.email)
            .or_else(|_| {
                // Создаем нового пользователя если не найден
                let new_user = NewUser {
                    email: user_info.email,
                    name: user_info.username,
                    password_hash: None, // У OAuth пользователей нет пароля
                };
                UserRepository::create(conn, &new_user)
            })
            .map_err(|_| AuthError::DatabaseError)?;

        if !user.is_active {
            return Err(AuthError::UserNotActive);
        }

        // Создаем сессию
        Self::create_session(conn, user, ip, user_agent).await
    }

    async fn verify_google_token(_token: &str) -> Result<OAuthUserInfo, AuthError> {
        // Заглушка
        let user_info = OAuthUserInfo {
            email: "user-from-google@example.com".to_string(),
            username: "google_user".to_string(),
        };

        Ok(user_info)
    }

    async fn verify_github_token(token: &str) -> Result<OAuthUserInfo, AuthError> {
        // Аналогично для GitHub
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {}", token))
            .send()
            .await
            .map_err(|e| AuthError::ProviderError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AuthError::ProviderError("Invalid GitHub token".into()));
        }

        let user_info: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::ProviderError(e.to_string()))?;

        Ok(OAuthUserInfo {
            email: user_info["email"].as_str().unwrap_or_default().to_string(),
            username: user_info["login"].as_str().unwrap_or_default().to_string(),
        })
    }

    async fn create_session(
        conn: &mut PgConnection,
        user: User,
        ip: &str,
        user_agent: Option<String>,
    ) -> Result<LoginResponse, AuthError> {
        let token = generate_jwt_token(user.id)?;
        let expires_at = (Utc::now() + Duration::days(7)).naive_utc();

        let new_session = NewSession {
            user_id: user.id,
            token: token.clone(),
            ip_address: ip.to_string(),
            user_agent,
            expires_at,
        };

        SessionRepository::create(conn, &new_session).map_err(|_| AuthError::DatabaseError)?;

        Ok(LoginResponse { token, user })
    }

    pub fn validate_token(conn: &mut PgConnection, token: &str) -> Result<User, AuthError> {
        // Валидация JWT токена
        let user_id = validate_jwt_token(token)?;

        UserRepository::find_by_id(conn, user_id)
            .map_err(|_| AuthError::UserNotFound)
            .and_then(|user| {
                if user.is_active {
                    Ok(user)
                } else {
                    Err(AuthError::UserNotActive)
                }
            })
    }
}
