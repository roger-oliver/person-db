use std::time::Duration;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_name: String,
    pub pool_size: u32,
    pub idle_timeout: u64,
    pub acquire_timeout: u64,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 5432,
            user: String::from("postgres"),
            password: String::from("postgres"),
            db_name: String::from("person_db"),
            pool_size: 30,
            idle_timeout: 120,
            acquire_timeout: 30,
        }
    }
}

impl PostgresConfig {
    pub fn get_url_connection(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }

    pub async fn create_pool(&self) -> Result<sqlx::PgPool, sqlx::Error> {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(self.pool_size)
            .acquire_timeout(Duration::from_secs(self.acquire_timeout))
            .idle_timeout(Duration::from_secs(self.idle_timeout))
            .connect(&self.get_url_connection())
            .await
    }
}
