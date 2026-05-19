use sqlx::{Pool, Postgres, postgres::PgQueryResult};

use crate::entity::{TaskEntity, TaskStatus, UserEntity};

pub type Executor = Pool<Postgres>;

pub struct UserRepository {
    executor: Executor,
}

impl UserRepository {
    pub fn new(executor: Executor) -> Self {
        Self { executor }
    }
    pub async fn find(&self, name: &str) -> Result<Option<UserEntity>, sqlx::Error> {
        sqlx::query_as!(UserEntity, r#"SELECT * FROM users WHERE name = $1"#, name)
            .fetch_optional(&self.executor)
            .await
    }
    pub async fn insert(
        &self,
        name: &str,
        password_hash: &str,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO users (name, password_hash) VALUES ($1, $2)"#,
            name,
            password_hash
        )
        .execute(&self.executor)
        .await
    }
}

pub struct TaskRepository {
    executor: Executor,
}

impl TaskRepository {
    pub fn new(executor: Executor) -> Self {
        Self { executor }
    }
    pub async fn fetch_all(&self) -> Result<Vec<TaskEntity>, sqlx::Error> {
        sqlx::query_as!(
            TaskEntity,
            "SELECT id, title, description, status as \"status: TaskStatus\", created_at, updated_at, user_id FROM tasks"
        ).fetch_all(&self.executor).await
    }
    pub async fn fetch_one(&self, id: i32) -> Result<Option<TaskEntity>, sqlx::Error> {
        sqlx::query_as!(
            TaskEntity,
            "SELECT id, title, description, status as \"status: TaskStatus\", created_at, updated_at, user_id FROM tasks WHERE id = $1",
            id
        ).fetch_optional(&self.executor).await
    }
}
