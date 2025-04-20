use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use std::env;

use crate::user::model::TokenClaims;

use super::AuthError;

pub fn generate_jwt_token(user_id: i32) -> Result<String, AuthError> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| AuthError::TokenCreation)?;

    let now = Utc::now();
    let expires = now + Duration::days(7);

    let claims = TokenClaims {
        sub: user_id,
        exp: expires.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreation)
}

pub fn validate_jwt_token(token: &str) -> Result<i32, AuthError> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| AuthError::InvalidToken)?;

    let decoded = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::InvalidToken)?;

    Ok(decoded.claims.sub)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    //TODO: Для реальная реализации взять argon2 !Только для примера!
    password == hash
}
