use chrono::NaiveDateTime;
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
    pub async fn find_by_name(&self, name: &str) -> Result<Option<UserEntity>, sqlx::Error> {
        sqlx::query_as!(UserEntity, r#"SELECT * FROM users WHERE name = $1"#, name)
            .fetch_optional(&self.executor)
            .await
    }
    pub async fn find_by_id(&self, id: i32) -> Result<Option<UserEntity>, sqlx::Error> {
        sqlx::query_as!(UserEntity, r#"SELECT * FROM users WHERE id = $1"#, id)
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
    pub async fn fetch_all(&self, user: &UserEntity) -> Result<Vec<TaskEntity>, sqlx::Error> {
        sqlx::query_as!(
            TaskEntity,
            "SELECT id, title, description, status as \"status: TaskStatus\", created_at, updated_at, deadline, user_id FROM tasks WHERE user_id = $1", user.id
        ).fetch_all(&self.executor).await
    }
    pub async fn fetch_all_filter_by_status(
        &self,
        user: &UserEntity,
        status: TaskStatus,
    ) -> Result<Vec<TaskEntity>, sqlx::Error> {
        sqlx::query_as!(
            TaskEntity,
            "SELECT id, title, description, status as \"status: TaskStatus\", created_at, updated_at, deadline, user_id FROM tasks WHERE user_id = $1 AND status = $2", user.id, status as TaskStatus
        ).fetch_all(&self.executor).await
    }
    pub async fn fetch_all_overdue_task(
        &self,
        user: &UserEntity,
        date: NaiveDateTime,
    ) -> Result<Vec<TaskEntity>, sqlx::Error> {
        sqlx::query_as!(
            TaskEntity,
            "SELECT id, title, description, status as \"status: TaskStatus\", created_at, updated_at, deadline, user_id FROM tasks WHERE user_id = $1 AND deadline < $2", user.id, date
        ).fetch_all(&self.executor).await
    }
    pub async fn fetch_one(
        &self,
        user: &UserEntity,
        task_id: i32,
    ) -> Result<Option<TaskEntity>, sqlx::Error> {
        sqlx::query_as!(
            TaskEntity,
            "SELECT id, title, description, status as \"status: TaskStatus\", created_at, updated_at, deadline, user_id FROM tasks WHERE id = $1 AND user_id = $2",
            task_id,
            user.id
        ).fetch_optional(&self.executor).await
    }
    pub async fn insert(
        &self,
        title: &str,
        description: &str,
        status: TaskStatus,
        created_at: NaiveDateTime,
        deadline: Option<NaiveDateTime>,
        user: &UserEntity,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO tasks (title, description, status, created_at, deadline, user_id) VALUES ($1, $2, $3, $4, $5, $6)",
            title,
            description,
            status as TaskStatus,
            created_at,
            deadline,
            user.id
        )
        .execute(&self.executor)
        .await
    }
    pub async fn delete(
        &self,
        task_id: i32,
        user: &UserEntity,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "DELETE FROM tasks WHERE id = $1 AND user_id = $2",
            task_id,
            user.id
        )
        .execute(&self.executor)
        .await
    }
    pub async fn update(
        &self,
        task_id: i32,
        title: &str,
        description: &str,
        status: TaskStatus,
        updated_at: NaiveDateTime,
        deadline: Option<NaiveDateTime>,
        user: &UserEntity,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "UPDATE tasks SET title = $1, description = $2, status = $3, updated_at = $4, deadline = $5 WHERE id = $6 AND user_id = $7",
            title,
            description,
            status as TaskStatus,
            updated_at,
            deadline,
            task_id,
            user.id
        ).execute(&self.executor).await
    }
}
