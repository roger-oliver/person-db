use std::{sync::Arc, time::Duration};

use fred::{
    prelude::{Client, ClientLike, Config, TcpConfig},
    types::Builder,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<String>,
    pub db: Option<u8>,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 6379,
            user: None,
            password: None,
            db: Some(1),
        }
    }
}

impl RedisConfig {
    pub fn get_connection_url(&self) -> String {
        let credentials = match (self.user.as_ref(), self.password.as_ref()) {
            (Some(u), Some(p)) => Some(format!("{}:{}@", u, p)),
            _ => None,
        };

        format!(
            "rediss://{}{}:{}/{}",
            credentials.unwrap_or_default(),
            self.host,
            self.port,
            self.db.unwrap_or_default()
        )
    }

    pub async fn create_client(&self) -> Result<Arc<Client>, fred::error::Error> {
        let config = Config::from_url(&self.get_connection_url())?;

        let client = Builder::from_config(config)
            .with_connection_config(|config| {
                config.connection_timeout = Duration::from_secs(5);
                config.tcp = TcpConfig {
                    nodelay: Some(false),
                    ..Default::default()
                };
            })
            .build()?;

        client.init().await?;

        Ok(Arc::new(client))
    }
}
