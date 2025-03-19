use fred::{
    error::Error,
    prelude::{Client, KeysInterface},
    types::Expiration,
};

pub async fn redis_get(client: &Client, key: &str) -> Result<String, Error> {
    let value = client.get(key).await?;
    Ok(value)
}

pub async fn redis_set(client: &Client, key: &str, value: &str) -> Result<(), Error> {
    let expiration = Some(Expiration::EX(300));

    let _: () = client.set(key, value, expiration, None, false).await?;

    Ok(())
}
