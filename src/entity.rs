#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
}
