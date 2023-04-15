use sqlx::PgPool;

use crate::client::database;

use super::{config::Config, route};

#[derive(Debug, Default, Clone)]
pub struct ApiState<T> {
    pub database_connection: T,
    pub config: Config
}

pub async fn start() -> tide::Result<()> {
    // read env vars from .env file to struct Config
    let conf = Config::read_env().expect("cannot read env variable");
    
    // connect to postgres database
    let db_connection = database::connect_to_postgres(conf.database_url.clone()).await;

    let state = ApiState::<PgPool>{
        database_connection: db_connection.clone(), 
        config: conf.clone(),
    };

    let mut api = tide::new();

    api.at("/api/v1").nest( {
        let v1 = route::api_v1(state.clone()).await;
        v1
    });
    
    Ok(())
}
