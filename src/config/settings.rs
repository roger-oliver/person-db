use config::{Config, Environment};
use dotenvy::{dotenv, from_filename};
use serde::Deserialize;

use super::{
    postgres_config::PostgresConfig, rabbitmq_config::RabbitmqConfig, redis_config::RedisConfig,
    server_config::ServerConfig,
};

#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    pub server: ServerConfig,
    pub rabbit: RabbitmqConfig,
    pub redis: RedisConfig,
    pub postgres: PostgresConfig,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        // dotenv().ok();

        from_filename(".env.local")
            .or_else(|_| dotenv())
            .ok();


        Ok(Config::builder()
            .add_source(Environment::default().separator("__").list_separator("__"))
            .build()?
            .try_deserialize::<Self>()
            .unwrap_or_default())
    }
}
