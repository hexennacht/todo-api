use std::sync::Arc;

use sqlx::{Pool, Postgres};

use super::config::Config;

pub struct ApiState<T> {
    DatabaseConnection: T,
    Config: Config
}

pub async fn start() -> tide::Result<()> {
    let conf = Config {}.read_env().expect("cannot read env");
    
    let state = Arc::new(ApiState::<Pool<sqlx::Postgres>>{DatabaseConnection: todo!(), Config: conf});

    let mut api = tide::new();

    api.at("/api/v1").nest( {
        let mut v1 = tide::with_state(state.clone());

        v1.at("_health").get(|_| async { Ok("Hello, World!") });

        v1
    });
    
    Ok(())
}
