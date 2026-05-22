use axum::{
    Json,
    extract::{Path, State},
};
use chrono::{Duration, Utc};

use crate::{
    app::AppState,
    auth::{generate_password_hash, generate_token, verify_password},
    entity::UserEntity,
    error::AppError,
    model::{JwtToken, TaskRequestModel, TaskResponseModel, TaskStatusModel, UserCredentials},
    validate::ValidJson,
};

pub async fn register_user(
    State(state): State<AppState>,
    ValidJson(body): ValidJson<UserCredentials>,
) -> Result<(), AppError> {
    let result = state.user_repo.find_by_name(&body.name).await?;
    if result.is_some() {
        return Err(AppError::UserAlreadyExists);
    }
    let password_hash = generate_password_hash(&body.password)?;
    let _ = state.user_repo.insert(&body.name, &password_hash).await?;
    Ok(())
}

pub async fn login_user(
    State(state): State<AppState>,
    ValidJson(body): ValidJson<UserCredentials>,
) -> Result<Json<JwtToken>, AppError> {
    let result = state.user_repo.find_by_name(&body.name).await?;
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

pub async fn get_all_task(
    State(state): State<AppState>,
    user: UserEntity,
) -> Result<Json<Vec<TaskResponseModel>>, AppError> {
    let tasks: Vec<TaskResponseModel> = state
        .task_repo
        .fetch_all(&user)
        .await?
        .into_iter()
        .map(|t| t.into())
        .collect();
    Ok(Json(tasks))
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    user: UserEntity,
) -> Result<Json<TaskResponseModel>, AppError> {
    match state.task_repo.fetch_one(&user, id).await? {
        Some(task) => Ok(Json(task.into())),
        None => Err(AppError::TaskNotFound),
    }
}

pub async fn register_task(
    State(state): State<AppState>,
    user: UserEntity,
    ValidJson(body): ValidJson<TaskRequestModel>,
) -> Result<(), AppError> {
    let created_at = Utc::now().naive_utc();
    state
        .task_repo
        .insert(
            &body.title,
            &body.description,
            body.status.into(),
            created_at,
            body.deadline,
            &user,
        )
        .await?;
    Ok(())
}

pub async fn delete_task(
    State(state): State<AppState>,
    user: UserEntity,
    Path(task_id): Path<i32>,
) -> Result<(), AppError> {
    let result = state.task_repo.delete(task_id, &user).await?;
    if result.rows_affected() == 0 {
        return Err(AppError::TaskNotFound);
    }
    Ok(())
}

pub async fn update_task(
    State(state): State<AppState>,
    user: UserEntity,
    Path(task_id): Path<i32>,
    Json(body): Json<TaskRequestModel>,
) -> Result<(), AppError> {
    let updated_at = Utc::now().naive_utc();
    state
        .task_repo
        .update(
            task_id,
            &body.title,
            &body.description,
            body.status.into(),
            updated_at,
            body.deadline,
            &user,
        )
        .await?;
    Ok(())
}

pub async fn get_all_task_filter_by_status(
    State(state): State<AppState>,
    user: UserEntity,
    Path(task_status): Path<TaskStatusModel>,
) -> Result<Json<Vec<TaskResponseModel>>, AppError> {
    let tasks: Vec<TaskResponseModel> = state
        .task_repo
        .fetch_all_filter_by_status(&user, task_status.into())
        .await?
        .into_iter()
        .map(|t| t.into())
        .collect();
    Ok(Json(tasks))
}

pub async fn get_overdue_task(
    State(state): State<AppState>,
    user: UserEntity,
) -> Result<Json<Vec<TaskResponseModel>>, AppError> {
    let now = Utc::now().naive_utc();
    let tasks: Vec<TaskResponseModel> = state
        .task_repo
        .fetch_all_overdue_task(&user, now)
        .await?
        .into_iter()
        .map(|t| t.into())
        .collect();
    Ok(Json(tasks))
}
