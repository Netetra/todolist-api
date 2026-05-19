use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    exp: i64,
}

impl Claims {
    pub fn new(iss: &str, sub: &str, exp: DateTime<Utc>) -> Self {
        Self {
            iss: iss.to_owned(),
            sub: sub.to_owned(),
            exp: exp.timestamp(),
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
