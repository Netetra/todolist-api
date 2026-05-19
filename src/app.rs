use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    repository::{Executor, TaskRepository, UserRepository},
    router::{get_all_task, get_task, login, user_register},
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
        .route("/auth/register", post(user_register))
        .route("/auth/login", post(login))
        .route("/todo/task/{id}", get(get_task))
        .route("/todo/tasks", get(get_all_task))
        .with_state(state)
}
