use std::{sync::Arc, thread::sleep, time::Duration};

use lapin::{
    options::{BasicAckOptions, BasicGetOptions, BasicPublishOptions},
    BasicProperties, Channel,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::data_access::db::{db_bulk_insert, AddPerson, Person};

pub async fn queue_person_create_request(
    add_person: &AddPerson,
    channel: Arc<Channel>,
    queue_name: &str,
) -> Result<Person, Box<dyn std::error::Error>> {
    let person = Person {
        id: Uuid::new_v4(),
        full_name: add_person.full_name.clone(),
        nickname: add_person.nickname.clone(),
        birth: add_person.birth,
        skills: add_person.skills.clone(),
    };

    let serialised_person = serde_json::to_vec(&person)?;

    match channel
        .basic_publish(
            "",
            queue_name,
            BasicPublishOptions::default(),
            &serialised_person,
            BasicProperties::default(),
        )
        .await
    {
        Ok(_) => Ok(person),
        Err(e) => {
            eprintln!("failed to publish message to queue: error {}", e);
            Err(e)?
        }
    }
}

pub async fn queue_batch_fetch_people(channel: Arc<Channel>, queue_name: &str, pg_pool: PgPool) {
    sleep(Duration::from_secs(5));

    loop {
        let people = fetch_messages_as_chunks(channel.clone(), queue_name, 1000).await;

        if !people.is_empty() {
            match db_bulk_insert(&pg_pool, people).await {
                Ok(c) => println!("{} entries bulk inserted!", c),
                Err(e) => eprintln!("failed to bulk insert! Error: {}", e),
            }
        }
    }
}

async fn fetch_messages_as_chunks(
    channel: Arc<Channel>,
    queue_name: &str,
    chunk_size: u16,
) -> Vec<Person> {
    let mut messages = Vec::new();
    let mut count: u16 = 0;

    while count < chunk_size {
        match channel
            .basic_get(queue_name, BasicGetOptions::default())
            .await
        {
            Ok(Some(delivery)) => {
                let person: Person = serde_json::from_slice(&delivery.data)
                    .expect("failed get string from channel message!");
                messages.push(person);
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("failed to ack message to channel!");
            }
            Ok(None) => break, // no more messages in the channel
            Err(e) => {
                eprintln!("error when getting messages from channel! Err: {}", e);
                break;
            }
        }
        count += 1;
    }
    messages
}
