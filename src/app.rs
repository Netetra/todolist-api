use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
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
