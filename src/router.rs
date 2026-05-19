use axum::{Json, extract::State};
use chrono::Duration;

use crate::{
    app::{AppError, AppState},
    auth::{generate_password_hash, generate_token, verify_password},
    model::{JwtToken, UserCredentials},
    validate::ValidJson,
};

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

pub async fn login(
    State(state): State<AppState>,
    ValidJson(body): ValidJson<UserCredentials>,
) -> Result<Json<JwtToken>, AppError> {
    let result = state.user_repo.find(&body.name).await?;
    let user = match result {
        Some(user) => user,
        None => {
            return Err(AppError::UserNotFound);
        }
    };
    verify_password(&body.password, &user.password_hash)?;

    let sub = user.id.to_string();
    let token = generate_token(
        &state.config.jwt_iss,
        &sub,
        Duration::days(1),
        &state.config.jwt_secret,
    )?;
    Ok(Json(JwtToken { token }))
}
