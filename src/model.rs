use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
