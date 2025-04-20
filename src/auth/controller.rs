use crate::{
    auth::{AuthError, models::LoginRequest, service::AuthService},
    storage::db::DbPool,
    utils::http_utils::{WebResponse, ok_response},
};

use actix_web::{HttpRequest, HttpResponse, web};

use super::{middleware::AuthenticatedUser, session_repository::SessionRepository};

pub type AuthResponse = WebResponse<AuthError>;

pub async fn login(
    pool: web::Data<DbPool>,
    request: web::Json<LoginRequest>,
    req: HttpRequest,
) -> AuthResponse {
    let mut conn = pool.get().map_err(|_| AuthError::DatabaseError)?;

    let ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let response = AuthService::login(&mut conn, request.into_inner(), &ip, user_agent).await?;

    Ok(ok_response(response))
}

pub async fn logout(a_user: AuthenticatedUser, pool: web::Data<DbPool>) -> AuthResponse {
    let mut conn = pool.get().map_err(|_| AuthError::DatabaseError)?;

    SessionRepository::deactivate_all_for_user(&mut conn, a_user.0.id)
        .map_err(|_| AuthError::DatabaseError)?;

    Ok(ok_response("Logged out successfully"))
}

pub async fn get_me(a_user: AuthenticatedUser) -> HttpResponse {
    ok_response(a_user.0)
}
