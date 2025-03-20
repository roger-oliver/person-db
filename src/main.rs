use config::settings::Settings;
use futures::TryFutureExt;

mod config;
mod controller;
mod data_access;
mod queue;
mod redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new().map_err(|e| {
        eprintln!("failed to initialise settings. Error: {}", e);
        e
    })?;

    let pg_pool = settings
        .postgres
        .create_pool()
        .map_err(|e| {
            eprintln!("failed to create postgres pool. Error {}", e);
            e
        })
        .await?;

    Ok(())
}
