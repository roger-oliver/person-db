use std::{str::FromStr, thread::sleep, time::Duration};

use chrono::NaiveDate;
use futures::future::join_all;
use reqwest::Client;
use sqlx::PgPool;

use crate::data_access::db::AddPerson;

pub async fn warm_up(localhost: &str, endpoint: &str) {
    println!("preparing warmup....");

    sleep(Duration::from_secs(2));

    println!("warmup starting....");

    let rq = Client::new();

    let rq_body = |idx: u16| {
        let add_person = AddPerson {
            full_name: format!("WARMUP_DATA::#{}", idx),
            nickname: "WARMUP_DATA".to_string(),
            birth: NaiveDate::from_str("2000-01-01").unwrap(),
            skills: Some(vec![String::from("skill1"), String::from("skill2")]),
        };

        serde_json::to_string(&add_person).unwrap()
    };

    let mut rqts = vec![];

    for i in 0..511 {
        rqts.push(
            rq.post(format!("http://{}/{}", localhost, endpoint))
                .body(rq_body(i))
                .header("Content-Type", "application/json")
                .send(),
        );
    }

    join_all(rqts).await;

    println!("warmup enqueueing concluded....");
}

pub async fn remove_warm_up(pg_pool: PgPool) -> Result<(), sqlx::Error> {
    sleep(Duration::from_secs(10));

    println!("cleaning warm up entries....");

    sqlx::query("delete from people where nickname ilike '%WARMUP%';")
        .execute(&pg_pool)
        .await?;

    println!("cleaned all warm up entries....");

    Ok(())
}
