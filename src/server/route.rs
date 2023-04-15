use crate::application::handler;

use super::server::ApiState;

pub async fn api_v1(state: ApiState<sqlx::PgPool>) -> tide::Server<ApiState<sqlx::PgPool>> {
    let mut api = tide::with_state(state.clone());

    api.at("_health").get(|_| async { 
        Ok("Hello there!")
    });

    api.at("todo").nest({
        let mut todo = tide::with_state(state.clone());

        todo.at("/").get(handler::list_todo);

        todo
    });

    api
}