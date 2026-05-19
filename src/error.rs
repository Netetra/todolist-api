use std::borrow::Cow;

use axum::{Json, extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use validator::ValidationErrors;

use crate::model::ErrorResponse;

#[allow(dead_code)]
pub enum AppError {
    UserAlreadyExists,
    UserNotFound,
    TaskNotFound,
    SqlxError(sqlx::Error),
    PasswordHashError(argon2::password_hash::Error),
    PasswordVerifyFail(argon2::password_hash::Error),
    JsonParseError(JsonRejection),
    ValidateError(ValidationErrors),
    TokenEncodeError(jsonwebtoken::errors::Error),
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
            Self::TaskNotFound => {
                let body = Json(ErrorResponse {
                    message: "task not found.".to_owned(),
                });
                (StatusCode::NOT_FOUND, body).into_response()
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
