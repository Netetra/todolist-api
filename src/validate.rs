use crate::error::AppError;
use axum::{Json, extract::FromRequest};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidJson<T>
where
    S: Sync + Send,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(AppError::JsonParseError)?;
        value.validate().map_err(AppError::ValidateError)?;
        Ok(ValidJson(value))
    }
}
