use crate::{application::request::GetTodoListRequest, server::server::ApiState, infrastructure::todo::{repository::Repository}};

use super::entity;

pub async fn get_todo_by_query(req: GetTodoListRequest, state: ApiState<sqlx::PgPool>) -> anyhow::Result<Vec<entity::Todo>> {
    let database_connection = state.database_connection;

    let todos = Repository::new(database_connection)
    .get_todo_by_query(req)
    .await
    .unwrap()
    .iter()
    .map(|row| {
        entity::Todo {
            id: row.id,
            name: row.name.clone(),
            description: row.description.clone(),
            done: row.done,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }).collect();

    anyhow::Ok(todos)
}