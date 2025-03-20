use std::sync::Arc;

use actix_web::{web, HttpResponse};
use fred::prelude::Client;
use serde::Deserialize;

use crate::{
    config::rabbitmq_config::RabbitmqChannel, data_access::db::AddPerson,
    queue::queue_person_create_request, redis::redis_set,
};

pub type ApiResult = Result<HttpResponse, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize)]
pub struct SearchParam {
    param: String,
}

#[actix_web::post("/people")]
pub async fn create_person(
    payload: web::Json<AddPerson>,
    redis_pool: web::Data<Arc<Client>>,
    rabbitmq_channel: web::Data<RabbitmqChannel>,
) -> ApiResult {
    if let Some(response) = validate_payload(&payload) {
        return Ok(response);
    }

    let channel = rabbitmq_channel.channel.clone();
    let queue_name = rabbitmq_channel.queue_name.clone();

    let dto = queue_person_create_request(&payload, channel, &queue_name).await?;

    let serialised_dto = serde_json::to_string(&dto)?;

    redis_set(&redis_pool, &dto.id.to_string(), &serialised_dto).await?;

    Ok(HttpResponse::Created()
        .append_header(("Path", format!("/people/{}", &dto.id)))
        .finish())
}

fn validate_payload(payload: &AddPerson) -> Option<HttpResponse> {
    if payload.full_name.len() > 100 {
        return Some(HttpResponse::BadRequest().finish());
    }

    if payload.nickname.len() > 32 {
        return Some(HttpResponse::BadRequest().finish());
    }

    if let Some(stack) = &payload.skills {
        for skill in stack {
            if skill.len() > 32 {
                return Some(HttpResponse::BadRequest().finish());
            }
        }
    }

    None
}
