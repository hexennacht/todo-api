use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::client::{database};

use super::{config::Config, route};

pub struct ApiState<T> {
    DatabaseConnection: T,
    Config: Config
}

pub async fn start() -> tide::Result<()> {
    // read env vars from .env file to struct Config
    let conf = Config { 
        database_url:"".to_string(),
    }
    .read_env()
    .expect("cannot read env variable");
    
    // connect to postgres database
    let db_connection = database::connect_to_postgres(conf.database_url.clone()).await;

    let state = Arc::new(ApiState::<Pool<Postgres>>{
        DatabaseConnection: db_connection, 
        Config: conf,
    });

    let mut api = tide::new();

    api.at("/api/v1").nest( {
        let v1 = route::api_v1(state.clone()).await;
        v1
    });
    
    Ok(())
}
