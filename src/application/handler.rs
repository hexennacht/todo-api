use crate::server::server::ApiState;
use tide::StatusCode;
use tide::prelude::*;

use super::request::GetTodoListRequest;

pub async fn list_todo(
    req: tide::Request<ApiState<sqlx::PgPool>>,
) -> tide::Result<tide::Response> {
    let _ = req.state();
    let request = req.query::<GetTodoListRequest>()?;

    let response = tide::Response::builder(StatusCode::Ok)
        .body(json!(request))
        .content_type(tide::http::mime::JSON)
        .header("custom-header", "value")
        .build();

    Ok(response)
}
