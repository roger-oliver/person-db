use std::{str::FromStr, sync::Arc};

use actix_web::{http::header::ContentType, web, HttpResponse};
use fred::prelude::Client;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::rabbitmq_config::RabbitmqChannel,
    data_access::db::{
        db_add_person, db_count_people, db_get_people_by_param, db_get_person_by_id, AddPerson,
    },
    queue::queue_person_create_request,
    redis::{redis_get, redis_set},
};

pub type ApiResult = Result<HttpResponse, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize)]
pub struct SearchParam {
    param: String,
}

#[actix_web::post("/person")]
pub async fn add_person(
    payload: web::Json<AddPerson>,
    redis: web::Data<Arc<Client>>,
    pg_pool: web::Data<PgPool>,
) -> ApiResult {
    if let Some(response) = validate_payload(&payload) {
        return Ok(response);
    }

    let dto = db_add_person(&pg_pool, &payload).await?;

    let serialised_dto = serde_json::to_string(&dto)?;

    redis_set(&redis, &dto.id.to_string(), &serialised_dto).await?;

    Ok(HttpResponse::Created()
        .append_header(("Path", format!("/people/{}", &dto.id)))
        .finish())
}

#[actix_web::get("/people/{id}")]
pub async fn get_person_by_id(
    id: web::Path<String>,
    pg_pool: web::Data<PgPool>,
    redis: web::Data<Arc<Client>>,
) -> ApiResult {
    match redis_get(&redis, &id).await {
        Err(_) => (),
        Ok(m) => {
            return Ok(HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(m))
        }
    }

    let dto = db_get_person_by_id(&pg_pool, Uuid::from_str(&id)?).await?;

    let serialised_dto = serde_json::to_string(&dto)?;

    let _ = redis_set(&redis, &dto.id.to_string(), &serialised_dto).await;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serialised_dto))
}

#[actix_web::get("/people")]
pub async fn get_people_by_param(
    param: web::Query<SearchParam>,
    pg_pool: web::Data<PgPool>,
) -> ApiResult {
    let people_result = db_get_people_by_param(&pg_pool, param.param.clone()).await?;

    Ok(HttpResponse::Ok().json(people_result))
}

#[actix_web::get("/count-people")]
pub async fn get_count_people(pg_pool: web::Data<PgPool>) -> ApiResult {
    let count = db_count_people(&pg_pool).await?;
    Ok(HttpResponse::Ok().body(count.to_string()))
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
