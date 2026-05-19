use std::{borrow::Cow, sync::Arc};

use axum::{
    Json, Router, extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse,
    routing::post,
};
use validator::ValidationErrors;

use crate::{
    model::ErrorResponse,
    repository::{Executor, UserRepository},
    router::{login, user_register},
};

pub struct Config {
    pub addr: String,
    pub db_url: String,
    pub jwt_secret: String,
    pub jwt_iss: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            addr: std::env::var("ADDR").unwrap(),
            db_url: std::env::var("DATABASE_URL").unwrap(),
            jwt_secret: std::env::var("JWT_SECRET").unwrap(),
            jwt_iss: std::env::var("JWT_ISS").unwrap(),
        }
    }
}

pub struct AppStateInner {
    pub user_repo: UserRepository,
    pub config: Config,
}
pub type AppState = Arc<AppStateInner>;

#[allow(dead_code)]
pub enum AppError {
    UserAlreadyExists,
    UserNotFound,
    SqlxError(sqlx::Error),
    PasswordHashError(argon2::password_hash::Error),
    PasswordVerifyFail(argon2::password_hash::Error),
    JsonParseError(JsonRejection),
    ValidateError(ValidationErrors),
    TokenEncodeError(jsonwebtoken::errors::Error),
}

pub fn build_app(executor: Executor, config: Config) -> Router {
    let state = Arc::new(AppStateInner {
        user_repo: UserRepository::new(executor.clone()),
        config,
    });
    Router::new()
        .route("/auth/register", post(user_register))
        .route("/auth/login", post(login))
        .with_state(state)
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::UserAlreadyExists => {
                let body = Json(ErrorResponse {
                    message: "user already exists.".to_owned(),
                });
                (StatusCode::CONFLICT, body).into_response()
            }
            Self::UserNotFound => {
                let body = Json(ErrorResponse {
                    message: "user not found.".to_owned(),
                });
                (StatusCode::BAD_REQUEST, body).into_response()
            }
            Self::SqlxError(_) | Self::PasswordHashError(_) | Self::TokenEncodeError(_) => {
                let body = Json(ErrorResponse {
                    message: "something went wrong.".to_owned(),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            Self::PasswordVerifyFail(_) => {
                let body = Json(ErrorResponse {
                    message: "password is wrong.".to_owned(),
                });
                (StatusCode::UNAUTHORIZED, body).into_response()
            }
            Self::JsonParseError(_) => {
                let body = Json(ErrorResponse {
                    message: "could not parse JSON.".to_owned(),
                });
                (StatusCode::BAD_REQUEST, body).into_response()
            }
            Self::ValidateError(e) => {
                let (_, errors) = e.field_errors().into_iter().next().unwrap();
                let error = errors.iter().next().unwrap();
                let default_message = Cow::from("validation error.");
                let message = error
                    .message
                    .as_ref()
                    .unwrap_or(&default_message)
                    .to_string();
                let body = Json(ErrorResponse { message });
                (StatusCode::BAD_REQUEST, body).into_response()
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::SqlxError(value)
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::PasswordHashError(value)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        AppError::TokenEncodeError(value)
    }
}
