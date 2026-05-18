use sqlx::{Pool, Postgres, postgres::PgQueryResult};

use crate::entity::UserEntity;

pub type Executor = Pool<Postgres>;

pub struct UserRepository {
    executor: Executor,
}

impl UserRepository {
    pub fn new(executor: Executor) -> Self {
        Self { executor }
    }
    pub async fn find(&self, name: &str) -> Result<Option<UserEntity>, sqlx::Error> {
        sqlx::query_as!(
            UserEntity,
            r#"SELECT * FROM "user" WHERE "name" = $1"#,
            name
        )
        .fetch_optional(&self.executor)
        .await
    }
    pub async fn insert(
        &self,
        name: &str,
        password_hash: &str,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO "user" ("name", "password_hash") VALUES ($1, $2)"#,
            name,
            password_hash
        )
        .execute(&self.executor)
        .await
    }
}
