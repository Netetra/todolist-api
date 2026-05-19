use chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

impl From<TaskStatus> for String {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Todo => "todo".to_owned(),
            TaskStatus::Doing => "doing".to_owned(),
            TaskStatus::Done => "done".to_owned()
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
    pub user_id: i32,
}
