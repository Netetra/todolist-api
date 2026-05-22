use axum::{extract::FromRequestParts, http::request::Parts};
use chrono::NaiveDateTime;

use crate::{error::AppError, model::TaskStatusModel};

#[derive(sqlx::FromRow, Clone)]
pub struct UserEntity {
    pub id: i32,
    #[allow(dead_code)]
    pub name: String,
    pub password_hash: String,
}

impl<S: Send + Sync> FromRequestParts<S> for UserEntity {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<Self>()
            .ok_or(AppError::UserNotFound)?;
        Ok(user.clone())
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

impl From<TaskStatusModel> for TaskStatus {
    fn from(value: TaskStatusModel) -> Self {
        match value {
            TaskStatusModel::Todo => TaskStatus::Todo,
            TaskStatusModel::Doing => TaskStatus::Doing,
            TaskStatusModel::Done => TaskStatus::Done,
        }
    }
}

impl From<TaskStatus> for String {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Todo => "todo".to_owned(),
            TaskStatus::Doing => "doing".to_owned(),
            TaskStatus::Done => "done".to_owned(),
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct TaskEntity {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deadline: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub user_id: i32,
}
