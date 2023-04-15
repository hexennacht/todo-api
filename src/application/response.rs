use serde::Serialize;

use crate::domain::todo::entity;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
struct Todo {
    id: i64,
    name: String,
    description: String,
    done: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetTodoListResponse {
    pub todos: Vec<entity::Todo>,
    pub total: i64,
    pub page: i64,
    pub page_size:  i64,
    pub total_pages: i64,
}