use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {

}

impl Config {
    pub fn read_env(&self) -> anyhow::Result<Config> {
        dotenv().expect(".env file not found");
        
        let conf =  match envy::from_env::<Config>() {
            Ok(conf) => Ok(conf),
            Err(err) => Err(err),
        };
        
        return anyhow::Ok(conf.expect("cannot read env variables"));
    }
}