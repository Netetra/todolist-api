use std::sync::LazyLock;

use chrono::NaiveDateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entity::TaskEntity;

static USER_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9 _-]{0,18}[a-zA-Z0-9]$").unwrap());
static USER_PASSWORD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9!@#$%_]{8,20}$").unwrap());

#[derive(Deserialize, Validate, Debug)]
pub struct UserCredentials {
    #[validate(regex(path=*USER_NAME_REGEX, message="ユーザー名は前後空白禁止で英数字と_と-のみ2文字以上20文字以下です。"))]
    pub name: String,
    #[validate(regex(path=*USER_PASSWORD_REGEX, message="パスワードは英数字と記号(!@#$%)のみ8文字以上20文字以下です。"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct JwtToken {
    pub token: String,
}

#[derive(Serialize)]
pub struct TaskModel {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<TaskEntity> for TaskModel {
    fn from(value: TaskEntity) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            status: value.status.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
