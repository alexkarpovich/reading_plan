use std::env;
use std::sync::OnceLock;

use crate::adapters::errors::AdapterError;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Config {
    pub WEB_ADDR: String,
    pub DATABASE_URL: String,
}

impl Config {
    fn load_from_env() -> Result<Config, AdapterError> {
        Ok(Config {
            WEB_ADDR: env::var("WEB_ADDR").unwrap(),
            DATABASE_URL: env::var("DATABASE_URL").unwrap(),
        })
    }
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| 
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL: while loading config. Cause - {ex:?}")
        })
    )
}