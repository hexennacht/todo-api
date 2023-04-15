use std::sync::Arc;

use super::server::ApiState;

pub async fn api_v1(state: Arc<ApiState<sqlx::Pool<sqlx::Postgres>>>) -> tide::Server<Arc::<ApiState<sqlx::Pool<sqlx::Postgres>>>> {
    let mut api = tide::with_state(state);

    api.at("_health").get(|_| async { Ok("Hello, World!") });

    api
}