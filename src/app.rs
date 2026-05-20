use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    auth::auth_middleware,
    repository::{Executor, TaskRepository, UserRepository},
    router::{
        delete_task, get_all_task, get_task, login_user, register_task, register_user, update_task,
    },
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
    pub task_repo: TaskRepository,
    pub config: Config,
}
pub type AppState = Arc<AppStateInner>;

pub fn build_app(executor: Executor, config: Config) -> Router {
    let state = Arc::new(AppStateInner {
        user_repo: UserRepository::new(executor.clone()),
        task_repo: TaskRepository::new(executor.clone()),
        config,
    });
    Router::new()
        .route("/todo/task", post(register_task))
        .route(
            "/todo/task/{task_id}",
            get(get_task).delete(delete_task).patch(update_task),
        )
        .route("/todo/tasks", get(get_all_task))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .with_state(state)
}
