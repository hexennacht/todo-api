use super::server::ApiState;

pub async fn api_v1(state: ApiState<sqlx::PgPool>) -> tide::Server<ApiState<sqlx::PgPool>> {
    let mut api = tide::with_state(state);

    api.at("_health").get(|_| async { 
        Ok("Oke")
     });

    api
}