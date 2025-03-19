use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: String::from("localhost"),
            port: 8080,
        }
    }
}

impl ServerConfig {
    pub fn get_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
