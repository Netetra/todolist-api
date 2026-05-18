use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::extract::State;

use crate::{
    app::{AppError, AppState},
    model::UserCredentials,
    validate::ValidJson,
};

fn generate_password_hash(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

pub async fn user_register(
    State(state): State<AppState>,
    ValidJson(body): ValidJson<UserCredentials>,
) -> Result<(), AppError> {
    let result = state.user_repo.find(&body.name).await?;
    if result.is_some() {
        return Err(AppError::UserAlreadyExists);
    }
    let password_hash = generate_password_hash(&body.password)?;
    let _ = state.user_repo.insert(&body.name, &password_hash).await?;
    Ok(())
}
