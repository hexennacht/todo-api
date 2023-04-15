use dotenv::dotenv;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub base_url: String,
}

impl Config {
    pub fn read_env() -> anyhow::Result<Config> {
        dotenv().expect(".env file not found");

        let conf =  match envy::from_env::<Config>() {
            Ok(conf) => Ok(conf),
            Err(err) => Err(err),
        };
        
        return anyhow::Ok(conf.expect("cannot read env variables"));
    }
}