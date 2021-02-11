use std::env;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,)]
pub struct Config {
    pub bind: String,
    pub port: String,
    pub version: i64,
}

impl Config {
    pub fn new(bind: String, port: String, version: i64) -> Self {
        Config {
            bind,
            port,
            version,
        }
    }
    pub fn bind_address(&self) -> String {
        String::from(format!("{}:{}", &self.bind, &self.port))
    }
}

//pub CONFIG: Config

pub fn make_config() -> Config {
    Config::new(
        env::var("DOT_ADDR").unwrap_or("127.0.0.1".to_string()),
        env::var("DOT_PORT").unwrap_or("2984".to_string()),
        1,
    )
}
