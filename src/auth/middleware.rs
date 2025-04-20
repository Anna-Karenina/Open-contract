use crate::{auth::AuthError, storage::db::DbPool, user::model::User};
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use std::future::{Ready, ready};

use super::service::AuthService;

pub struct AuthenticatedUser(pub User);

impl FromRequest for AuthenticatedUser {
    type Error = AuthError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        let mut conn = req
            .app_data::<web::Data<DbPool>>()
            .expect("DbPool not found in app_data")
            .get()
            .expect("DbPool not found in app_data");

        match token {
            Some(token) => match AuthService::validate_token(&mut *conn, token) {
                Ok(user) => ready(Ok(AuthenticatedUser(user))),
                Err(e) => ready(Err(e)),
            },
            None => ready(Err(AuthError::MissingCredentials)),
        }
    }
}
