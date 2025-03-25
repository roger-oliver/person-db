use actix_web::{http::KeepAlive, web, App, HttpServer};
use config::settings::Settings;
use controller::{
    add_person, create_person, get_count_people, get_people_by_param, get_person_by_id,
};
use data_access::warmup_jobs::{remove_warm_up, warm_up};
use futures::TryFutureExt;
use queue::queue_batch_fetch_people;

mod config;
mod controller;
mod data_access;
mod queue;
mod redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (server, pg_pool, redis_client, rabbitmq_channel) = {
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

        let redis_client = settings
            .redis
            .create_client()
            .map_err(|e| {
                eprintln!("failed to create redis client. Error {}", e);
                e
            })
            .await?;

        let rabbitmq_channel = settings
            .rabbit
            .create_channel()
            .map_err(|e| {
                eprintln!("failed to create rabbit client connection. Error {}", e);
                e
            })
            .await?;

        let server = settings.server;

        (server, pg_pool, redis_client, rabbitmq_channel)
    };

    tokio::spawn({
        let server_host = server.get_url().clone();
        let endpoint = "people";

        async move { warm_up(&server_host, endpoint).await }
    });

    tokio::spawn({
        let pg_pool_move = pg_pool.clone();

        async move { remove_warm_up(pg_pool_move).await }
    });

    tokio::spawn({
        let channel = rabbitmq_channel.channel.clone();
        let queue_name = rabbitmq_channel.queue_name.clone();
        let pg_pool_move = pg_pool.clone();

        async move { queue_batch_fetch_people(channel, &queue_name, pg_pool_move).await }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(rabbitmq_channel.clone()))
            .service(create_person)
            .service(get_count_people)
            .service(get_people_by_param)
            .service(get_person_by_id)
            .service(add_person)
    })
    .keep_alive(KeepAlive::Os)
    .bind(server.get_url())?
    .run()
    .await?;

    Ok(())
}
