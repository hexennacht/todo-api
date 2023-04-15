use crate::domain;
use crate::server::server::ApiState;
use tide::prelude::*;
use tide::StatusCode;

use super::request::GetTodoListRequest;
use super::response;

pub async fn list_todo(req: tide::Request<ApiState<sqlx::PgPool>>) -> tide::Result<tide::Response> {
    let state = req.state();
    let request = req.query::<GetTodoListRequest>()?;

    let todos = domain::todo::service::get_todo_by_query(request.clone(), state.clone()).await;

    let response = tide::Response::builder(StatusCode::Ok)
        .body(json!(response::GetTodoListResponse {
            todos: todos.unwrap(),
            total: 0,
            page: 1,
            page_size: 25,
            total_pages: 100
        }))
        .content_type(tide::http::mime::JSON)
        .header("custom-header", "value")
        .build();

    Ok(response)
}
