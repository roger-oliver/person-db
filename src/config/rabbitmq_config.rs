use std::sync::Arc;

use lapin::{Channel, Connection, ConnectionProperties};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RabbitmqConfig {
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub password: Option<String>,
    pub queue_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RabbitmqChannel {
    pub channel: Arc<Channel>,
    pub queue_name: String,
}

impl Default for RabbitmqConfig {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 5672,
            user: Some(String::from("guest")),
            password: Some(String::from("guest")),
            queue_name: Some(String::from("people-queue")),
        }
    }
}

impl RabbitmqConfig {
    pub fn get_connection_url(&self) -> String {
        format!(
            "amqp://{}:{}@{}:{}//",
            self.user.as_ref().unwrap(),
            self.password.as_ref().unwrap(),
            self.host,
            self.port
        )
    }

    pub async fn create_channel(&self) -> Result<RabbitmqChannel, lapin::Error> {
        let conn_url = self.get_connection_url();

        let channel = Connection::connect(&conn_url, ConnectionProperties::default())
            .await?
            .create_channel()
            .await?;

        Ok(RabbitmqChannel {
            channel: Arc::new(channel),
            queue_name: self.queue_name.clone().unwrap(),
        })
    }
}
