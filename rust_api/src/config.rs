use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok(); // Load .env file variables into the environment
        let database_url = env::var("DATABASE_URL")?;
        Ok(Self { database_url })
    }
}
