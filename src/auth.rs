use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{app::AppState, error::AppError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    iss: String,
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(iss: &str, sub: &str, exp: DateTime<Utc>) -> Self {
        Self {
            iss: iss.to_owned(),
            sub: sub.to_owned(),
            exp: exp.timestamp() as usize,
        }
    }
}

pub fn generate_password_hash(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
    Ok(())
}

pub fn generate_token(
    iss: &str,
    sub: &str,
    ttl: Duration,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::new(jsonwebtoken::Algorithm::HS512);
    let exp = Utc::now() + ttl;
    let claims = Claims::new(iss, sub, exp);
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = match request.headers().get(header::AUTHORIZATION) {
        Some(value) => value,
        None => {
            return Err(AppError::TokenNotSet);
        }
    };
    let token = auth_header
        .to_str()
        .map_err(|_| AppError::TokenNotSet)?
        .strip_prefix("Bearer ")
        .ok_or(AppError::TokenVerifyError)?;
    let secret = &state.config.jwt_secret;
    let claims: Claims = decode(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    )
    .map_err(|_| AppError::TokenVerifyError)?
    .claims;
    let user_id: i32 = claims.sub.parse().map_err(|_| AppError::TokenVerifyError)?;
    let user = match state.user_repo.find_by_id(user_id).await? {
        Some(value) => value,
        None => {
            return Err(AppError::TokenVerifyError);
        }
    };
    request.extensions_mut().insert(user.clone());
    let response = next.run(request).await;
    Ok(response)
}
